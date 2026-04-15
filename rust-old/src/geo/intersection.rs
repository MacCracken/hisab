use super::*;

// Primitives
// ---------------------------------------------------------------------------

/// A triangle defined by three vertices.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
}

impl Triangle {
    /// Create a triangle from three vertices.
    #[must_use]
    #[inline]
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self {
            vertices: [a, b, c],
        }
    }

    /// Face normal (not normalized). Returns the cross product of two edges.
    ///
    /// The magnitude equals twice the triangle's area. Use [`unit_normal`](Self::unit_normal)
    /// for a normalized version.
    #[must_use]
    #[inline]
    pub fn normal(&self) -> Vec3 {
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        edge1.cross(edge2)
    }

    /// Normalized face normal.
    ///
    /// Returns [`Vec3::Y`] for degenerate triangles (collinear/coincident vertices).
    #[must_use]
    #[inline]
    pub fn unit_normal(&self) -> Vec3 {
        let n = self.normal();
        let len_sq = n.length_squared();
        if len_sq < crate::EPSILON_F32 {
            return Vec3::Y; // Degenerate — arbitrary fallback
        }
        n * len_sq.sqrt().recip()
    }

    /// Area of the triangle.
    #[must_use]
    #[inline]
    pub fn area(&self) -> f32 {
        self.normal().length() * 0.5
    }

    /// Centroid (average of the three vertices).
    #[must_use]
    #[inline]
    pub fn centroid(&self) -> Vec3 {
        (self.vertices[0] + self.vertices[1] + self.vertices[2]) / 3.0
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = f.precision();
        let [a, b, c] = self.vertices;
        match p {
            Some(p) => write!(
                f,
                "Triangle(({:.p$}, {:.p$}, {:.p$}), ({:.p$}, {:.p$}, {:.p$}), ({:.p$}, {:.p$}, {:.p$}))",
                a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z
            ),
            None => write!(
                f,
                "Triangle(({}, {}, {}), ({}, {}, {}), ({}, {}, {}))",
                a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z
            ),
        }
    }
}

/// An infinite line defined by a point and a direction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub origin: Vec3,
    /// Normalized direction.
    pub direction: Vec3,
}

impl Line {
    /// Create a new line. Direction is normalized automatically.
    ///
    /// # Errors
    /// Returns [`crate::HisabError::InvalidInput`] if `direction` is zero-length.
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Result<Self, crate::HisabError> {
        let len_sq = direction.length_squared();
        if len_sq < crate::EPSILON_F32 {
            return Err(crate::HisabError::InvalidInput(
                "line direction must be non-zero".into(),
            ));
        }
        Ok(Self {
            origin,
            direction: direction.normalize(),
        })
    }

    /// Closest point on this infinite line to the given point.
    #[must_use]
    #[inline]
    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        let v = point - self.origin;
        let t = v.dot(self.direction);
        self.origin + self.direction * t
    }

    /// Distance from a point to this line.
    #[must_use]
    #[inline]
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        (point - self.closest_point(point)).length()
    }
}

/// A line segment defined by start and end points.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub start: Vec3,
    pub end: Vec3,
}

impl Segment {
    /// Create a segment from start and end points.
    #[must_use]
    #[inline]
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }

    /// Length of the segment.
    #[must_use]
    #[inline]
    pub fn length(&self) -> f32 {
        (self.end - self.start).length()
    }

    /// Midpoint of the segment.
    #[must_use]
    #[inline]
    pub fn midpoint(&self) -> Vec3 {
        (self.start + self.end) * 0.5
    }

    /// Normalized direction from start to end.
    ///
    /// Returns [`Vec3::X`] for degenerate (zero-length) segments.
    #[must_use]
    #[inline]
    pub fn direction(&self) -> Vec3 {
        let d = self.end - self.start;
        let len_sq = d.length_squared();
        if len_sq < crate::EPSILON_F32 {
            return Vec3::X; // Degenerate — arbitrary fallback
        }
        d * len_sq.sqrt().recip()
    }

    /// Closest point on this segment to the given point.
    #[must_use]
    #[inline]
    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        let ab = self.end - self.start;
        let len_sq = ab.dot(ab);
        if len_sq < crate::EPSILON_F32 {
            return self.start; // Degenerate segment
        }
        let t = ((point - self.start).dot(ab) / len_sq).clamp(0.0, 1.0);
        self.start + ab * t
    }

    /// Distance from a point to this segment.
    #[must_use]
    #[inline]
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        (point - self.closest_point(point)).length()
    }
}

/// A view frustum defined by six planes (near, far, left, right, top, bottom).
///
/// The planes' normals point **inward** — a point is inside the frustum if it is
/// on the positive side of all six planes.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    /// Extract frustum planes from a view-projection matrix.
    ///
    /// Uses the Gribb/Hartmann method. Planes are normalized.
    #[must_use]
    pub fn from_view_projection(vp: glam::Mat4) -> Self {
        let r = vp.to_cols_array_2d();
        // Row-based extraction (transposed column-major)
        let row = |i: usize| -> [f32; 4] { [r[0][i], r[1][i], r[2][i], r[3][i]] };
        let r0 = row(0);
        let r1 = row(1);
        let r2 = row(2);
        let r3 = row(3);

        let make_plane = |a: f32, b: f32, c: f32, d: f32| -> Plane {
            let len = (a * a + b * b + c * c).sqrt();
            Plane {
                normal: Vec3::new(a / len, b / len, c / len),
                distance: -d / len,
            }
        };

        let planes = [
            // Near:   row3 + row2
            make_plane(r3[0] + r2[0], r3[1] + r2[1], r3[2] + r2[2], r3[3] + r2[3]),
            // Far:    row3 - row2
            make_plane(r3[0] - r2[0], r3[1] - r2[1], r3[2] - r2[2], r3[3] - r2[3]),
            // Left:   row3 + row0
            make_plane(r3[0] + r0[0], r3[1] + r0[1], r3[2] + r0[2], r3[3] + r0[3]),
            // Right:  row3 - row0
            make_plane(r3[0] - r0[0], r3[1] - r0[1], r3[2] - r0[2], r3[3] - r0[3]),
            // Top:    row3 - row1
            make_plane(r3[0] - r1[0], r3[1] - r1[1], r3[2] - r1[2], r3[3] - r1[3]),
            // Bottom: row3 + row1
            make_plane(r3[0] + r1[0], r3[1] + r1[1], r3[2] + r1[2], r3[3] + r1[3]),
        ];

        Self { planes }
    }

    /// Check whether a point is inside the frustum.
    #[must_use]
    #[inline]
    pub fn contains_point(&self, point: Vec3) -> bool {
        self.planes.iter().all(|p| p.signed_distance(point) >= 0.0)
    }

    /// Conservative check whether an AABB intersects the frustum.
    ///
    /// Returns `false` only if the AABB is fully outside at least one plane.
    #[must_use]
    #[inline]
    pub fn contains_aabb(&self, aabb: &Aabb) -> bool {
        for plane in &self.planes {
            // Find the corner most aligned with the plane normal (P-vertex)
            let p = Vec3::new(
                if plane.normal.x >= 0.0 {
                    aabb.max.x
                } else {
                    aabb.min.x
                },
                if plane.normal.y >= 0.0 {
                    aabb.max.y
                } else {
                    aabb.min.y
                },
                if plane.normal.z >= 0.0 {
                    aabb.max.z
                } else {
                    aabb.min.z
                },
            );
            if plane.signed_distance(p) < 0.0 {
                return false;
            }
        }
        true
    }

    /// Conservative check whether a sphere intersects the frustum.
    ///
    /// Returns `false` only if the sphere is fully outside at least one plane.
    #[must_use]
    #[inline]
    pub fn contains_sphere(&self, sphere: &Sphere) -> bool {
        for plane in &self.planes {
            if plane.signed_distance(sphere.center) < -sphere.radius {
                return false;
            }
        }
        true
    }

    /// Conservative check whether an OBB intersects the frustum.
    ///
    /// Uses the separating axis test against each frustum plane. For each plane,
    /// computes the OBB's projection radius (the extent of the OBB along the
    /// plane normal) and checks whether the OBB center is within that radius of
    /// the plane.
    ///
    /// Returns `false` only if the OBB is fully outside at least one plane.
    #[must_use]
    #[inline]
    pub fn contains_obb(&self, obb: &Obb) -> bool {
        let axes = obb.axes();
        let he = obb.half_extents.to_array();
        for plane in &self.planes {
            let signed_dist = plane.signed_distance(obb.center);
            // Projection radius: sum of |half_extent_i * dot(axis_i, plane_normal)|
            let radius = he[0] * axes[0].dot(plane.normal).abs()
                + he[1] * axes[1].dot(plane.normal).abs()
                + he[2] * axes[2].dot(plane.normal).abs();
            if signed_dist + radius < 0.0 {
                return false;
            }
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Intersection / overlap functions
// ---------------------------------------------------------------------------

/// Ray-triangle intersection using the Möller–Trumbore algorithm.
///
/// Returns the `t` parameter if the ray hits the triangle (only `t >= 0`).
#[must_use]
#[inline]
pub fn ray_triangle(ray: &Ray, tri: &Triangle) -> Option<f32> {
    let edge1 = tri.vertices[1] - tri.vertices[0];
    let edge2 = tri.vertices[2] - tri.vertices[0];
    let h = ray.direction.cross(edge2);
    let a = edge1.dot(h);

    if a.abs() < crate::EPSILON_F32 {
        return None; // Ray parallel to triangle
    }

    let f = 1.0 / a;
    let s = ray.origin - tri.vertices[0];
    let u = f * s.dot(h);

    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = s.cross(edge1);
    let v = f * ray.direction.dot(q);

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * edge2.dot(q);
    if t >= 0.0 { Some(t) } else { None }
}

/// Check whether two AABBs overlap.
#[must_use]
#[inline]
pub fn aabb_aabb(a: &Aabb, b: &Aabb) -> bool {
    a.min.cmple(b.max).all() && b.min.cmple(a.max).all()
}

/// Check whether two spheres overlap.
#[must_use]
#[inline]
pub fn sphere_sphere(a: &Sphere, b: &Sphere) -> bool {
    let r = a.radius + b.radius;
    (a.center - b.center).length_squared() <= r * r
}

/// Intersection of two planes. Returns the line of intersection, or `None` if parallel.
#[must_use]
pub fn plane_plane(a: &Plane, b: &Plane) -> Option<Line> {
    let dir = a.normal.cross(b.normal);
    let len_sq = dir.dot(dir);
    if len_sq < crate::EPSILON_F32 {
        return None; // Planes are parallel
    }
    // Find a point on the intersection line
    let point = (dir.cross(b.normal) * a.distance + a.normal.cross(dir) * b.distance) / len_sq;
    Some(Line {
        origin: point,
        direction: dir.normalize(),
    })
}

// ---------------------------------------------------------------------------

// Ray-quadric intersection
// ---------------------------------------------------------------------------

/// Ray-quadric intersection for general quadric surfaces.
///
/// A quadric is defined by `xᵀAx + bᵀx + c = 0` where A is 3×3.
/// Returns the nearest `t >= 0` if the ray hits the surface.
///
/// - `a_mat`: 3×3 symmetric matrix (row-major as `[[f32; 3]; 3]`).
/// - `b_vec`: linear coefficient vector `[f32; 3]`.
/// - `c_val`: constant term.
#[must_use]
#[inline]
pub fn ray_quadric(ray: &Ray, a_mat: &[[f32; 3]; 3], b_vec: &[f32; 3], c_val: f32) -> Option<f32> {
    let o = ray.origin.to_array();
    let d = ray.direction.to_array();

    // Quadratic: (dᵀAd)t² + (2oᵀAd + bᵀd)t + (oᵀAo + bᵀo + c) = 0
    let mut a_coeff = 0.0f32;
    let mut b_coeff = 0.0f32;
    let mut c_coeff = c_val;

    for i in 0..3 {
        c_coeff += b_vec[i] * o[i];
        b_coeff += b_vec[i] * d[i];
        for j in 0..3 {
            a_coeff += d[i] * a_mat[i][j] * d[j];
            b_coeff += 2.0 * o[i] * a_mat[i][j] * d[j];
            c_coeff += o[i] * a_mat[i][j] * o[j];
        }
    }

    if a_coeff.abs() < crate::EPSILON_F32 {
        // Linear case
        if b_coeff.abs() < crate::EPSILON_F32 {
            return None;
        }
        let t = -c_coeff / b_coeff;
        return if t >= 0.0 { Some(t) } else { None };
    }

    let disc = b_coeff * b_coeff - 4.0 * a_coeff * c_coeff;
    if disc < 0.0 {
        return None;
    }
    let sqrt_disc = disc.sqrt();
    let inv_2a = 0.5 / a_coeff;
    let t1 = (-b_coeff - sqrt_disc) * inv_2a;
    let t2 = (-b_coeff + sqrt_disc) * inv_2a;

    if t1 >= 0.0 {
        Some(t1)
    } else if t2 >= 0.0 {
        Some(t2)
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Fresnel equations
// ---------------------------------------------------------------------------

/// Compute the refraction direction using Snell's law.
///
/// `incident` and `normal` must be normalized.
/// `eta` is `n1 / n2` (ratio of refractive indices).
/// Returns `None` for total internal reflection.
#[must_use]
#[inline]
pub fn refract(incident: Vec3, normal: Vec3, eta: f32) -> Option<Vec3> {
    let cos_i = -incident.dot(normal);
    let sin2_t = eta * eta * (1.0 - cos_i * cos_i);
    if sin2_t > 1.0 {
        return None; // Total internal reflection
    }
    let cos_t = (1.0 - sin2_t).sqrt();
    Some(incident * eta + normal * (eta * cos_i - cos_t))
}

/// Fresnel reflectance (Schlick's approximation).
///
/// `cos_theta`: cosine of the angle between incident ray and surface normal.
/// `n1`, `n2`: refractive indices of the two media.
#[must_use]
#[inline]
pub fn fresnel_schlick(cos_theta: f32, n1: f32, n2: f32) -> f32 {
    let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

/// Exact Fresnel reflectance (unpolarized light).
///
/// Returns the average of s-polarized and p-polarized reflectance.
#[must_use]
#[inline]
pub fn fresnel_exact(cos_i: f32, n1: f32, n2: f32) -> f32 {
    let sin2_t = (n1 / n2).powi(2) * (1.0 - cos_i * cos_i);
    if sin2_t >= 1.0 {
        return 1.0; // Total internal reflection
    }
    let cos_t = (1.0 - sin2_t).sqrt();
    let rs = ((n1 * cos_i - n2 * cos_t) / (n1 * cos_i + n2 * cos_t)).powi(2);
    let rp = ((n1 * cos_t - n2 * cos_i) / (n1 * cos_t + n2 * cos_i)).powi(2);
    (rs + rp) * 0.5
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Mat4, Quat, Vec3};

    fn ortho_frustum() -> Frustum {
        // Orthographic projection covering ±10 on X/Y and depth 0..20 (NDC Z in [0,1]).
        let proj = Mat4::orthographic_rh(-10.0, 10.0, -10.0, 10.0, 0.1, 20.0);
        Frustum::from_view_projection(proj)
    }

    fn unit_obb_at(center: Vec3) -> Obb {
        Obb::new(center, Vec3::splat(1.0), Quat::IDENTITY)
    }

    // --- Frustum-OBB tests ---------------------------------------------------

    #[test]
    fn frustum_obb_center_inside() {
        let frustum = ortho_frustum();
        let obb = unit_obb_at(Vec3::new(0.0, 0.0, -10.0));
        assert!(frustum.contains_obb(&obb), "OBB at origin should be inside");
    }

    #[test]
    fn frustum_obb_fully_outside_x() {
        let frustum = ortho_frustum();
        // Center at x=20, half_extent=1 → fully outside the x=10 boundary.
        let obb = unit_obb_at(Vec3::new(20.0, 0.0, -10.0));
        assert!(
            !frustum.contains_obb(&obb),
            "OBB far outside on X should be culled"
        );
    }

    #[test]
    fn frustum_obb_straddling_plane_not_culled() {
        let frustum = ortho_frustum();
        // Center at x=9.5, half_extent=2 → projection radius along X is 2, so
        // the OBB extends from 7.5 to 11.5 — it straddles the x=10 plane.
        let obb = Obb::new(Vec3::new(9.5, 0.0, -10.0), Vec3::splat(2.0), Quat::IDENTITY);
        assert!(
            frustum.contains_obb(&obb),
            "OBB straddling plane must not be culled"
        );
    }

    #[test]
    fn frustum_obb_rotated_projection_radius() {
        let frustum = ortho_frustum();
        // 45° rotation around Z increases the projection of a (2,2,1) box onto X/Y.
        // With half-extents (2,2,1) rotated 45°: radius along X = 2*cos45 + 2*sin45 ≈ 2.83.
        // Center at (8, 0, -10): signed_dist = 10 - 8 = 2; 2.83 > 2 so it overlaps.
        let rot = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);
        let obb = Obb::new(Vec3::new(8.0, 0.0, -10.0), Vec3::new(2.0, 2.0, 1.0), rot);
        assert!(
            frustum.contains_obb(&obb),
            "Rotated OBB overlapping boundary should not be culled"
        );
    }

    #[test]
    fn frustum_obb_just_outside() {
        let frustum = ortho_frustum();
        // Center at x=11.5, half_extent=1 → even with radius=1, min edge is at 10.5 > 10.
        let obb = Obb::new(
            Vec3::new(11.5, 0.0, -10.0),
            Vec3::splat(1.0),
            Quat::IDENTITY,
        );
        assert!(
            !frustum.contains_obb(&obb),
            "OBB just outside boundary should be culled"
        );
    }
}
