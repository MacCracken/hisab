use super::*;

/// A ray defined by an origin and a direction.
///
/// # Examples
///
/// ```
/// use hisab::geo::{Ray, Sphere, ray_sphere};
/// use glam::Vec3;
///
/// let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z).unwrap();
/// let sphere = Sphere::new(Vec3::ZERO, 1.0).unwrap();
/// let t = ray_sphere(&ray, &sphere).unwrap();
/// assert!((t - 4.0).abs() < 1e-5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vec3,
    /// Should be normalized for correct distance results.
    pub direction: Vec3,
}

impl Ray {
    /// Create a new ray. Direction is normalized automatically.
    ///
    /// # Errors
    /// Returns [`crate::HisabError::InvalidInput`] if `direction` is zero-length.
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Result<Self, crate::HisabError> {
        let len_sq = direction.length_squared();
        if len_sq < crate::EPSILON_F32 {
            return Err(crate::HisabError::InvalidInput(
                "ray direction must be non-zero".into(),
            ));
        }
        Ok(Self {
            origin,
            direction: direction.normalize(),
        })
    }

    /// Point along the ray at parameter `t`.
    #[must_use]
    #[inline]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = f.precision();
        let o = self.origin;
        let d = self.direction;
        match p {
            Some(p) => write!(
                f,
                "Ray({:.p$}, {:.p$}, {:.p$} -> {:.p$}, {:.p$}, {:.p$})",
                o.x, o.y, o.z, d.x, d.y, d.z
            ),
            None => write!(
                f,
                "Ray({}, {}, {} -> {}, {}, {})",
                o.x, o.y, o.z, d.x, d.y, d.z
            ),
        }
    }
}

/// An infinite plane defined by a normal and a signed distance from the origin.
///
/// Points **on** the plane satisfy `dot(normal, point) - distance == 0`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

impl Plane {
    /// Create a plane from a point on the plane and a normal.
    ///
    /// # Errors
    /// Returns [`crate::HisabError::InvalidInput`] if `normal` is zero-length.
    #[inline]
    pub fn from_point_normal(point: Vec3, normal: Vec3) -> Result<Self, crate::HisabError> {
        let len_sq = normal.length_squared();
        if len_sq < crate::EPSILON_F32 {
            return Err(crate::HisabError::InvalidInput(
                "plane normal must be non-zero".into(),
            ));
        }
        let n = normal * len_sq.sqrt().recip();
        Ok(Self {
            normal: n,
            distance: n.dot(point),
        })
    }

    /// Signed distance from a point to the plane.
    /// Positive = same side as normal, negative = opposite side.
    #[must_use]
    #[inline]
    pub fn signed_distance(&self, point: Vec3) -> f32 {
        self.normal.dot(point) - self.distance
    }
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = f.precision();
        let n = self.normal;
        match p {
            Some(p) => write!(
                f,
                "Plane(n=({:.p$}, {:.p$}, {:.p$}), d={:.p$})",
                n.x, n.y, n.z, self.distance
            ),
            None => write!(
                f,
                "Plane(n=({}, {}, {}), d={})",
                n.x, n.y, n.z, self.distance
            ),
        }
    }
}

/// An axis-aligned bounding box.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    /// Create a new AABB. Min/max are corrected if swapped.
    #[must_use]
    #[inline]
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }

    /// Check whether a point is inside (or on the boundary of) this AABB.
    #[must_use]
    #[inline]
    pub fn contains(&self, point: Vec3) -> bool {
        point.cmpge(self.min).all() && point.cmple(self.max).all()
    }

    /// Center point of the AABB.
    #[must_use]
    #[inline]
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// Size (extents) of the AABB.
    #[must_use]
    #[inline]
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// Merge two AABBs into one that encloses both.
    #[must_use]
    #[inline]
    pub fn merge(&self, other: &Aabb) -> Aabb {
        Aabb {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Compute the AABB of this AABB after applying an affine transform.
    ///
    /// Uses the Arvo/Koppelman method, which avoids transforming all 8 corners.
    /// For each output axis `i`, the contribution of input column `j` is:
    /// `min[i] += min(m[i][j]*old_min[j], m[i][j]*old_max[j])`.
    ///
    /// Only the upper-left 3×3 rotation/scale portion of `transform` and the
    /// translation column are used (the homogeneous row is ignored, so this is
    /// correct for affine — but not projective — transforms).
    #[must_use]
    #[inline]
    pub fn transformed(&self, transform: glam::Mat4) -> Aabb {
        // Extract the 3×3 linear part and the translation.
        let col = [
            transform.x_axis.truncate(), // column 0
            transform.y_axis.truncate(), // column 1
            transform.z_axis.truncate(), // column 2
        ];
        let translation = transform.w_axis.truncate();

        let old_min = self.min.to_array();
        let old_max = self.max.to_array();

        let mut new_min = translation;
        let mut new_max = translation;

        // For each output axis i, accumulate contributions from each input axis j.
        let new_min_arr = new_min.as_mut();
        let new_max_arr = new_max.as_mut();
        for j in 0..3 {
            let col_arr = col[j].to_array();
            for i in 0..3 {
                let lo = col_arr[i] * old_min[j];
                let hi = col_arr[i] * old_max[j];
                if lo < hi {
                    new_min_arr[i] += lo;
                    new_max_arr[i] += hi;
                } else {
                    new_min_arr[i] += hi;
                    new_max_arr[i] += lo;
                }
            }
        }

        Aabb {
            min: new_min,
            max: new_max,
        }
    }
}

impl fmt::Display for Aabb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = f.precision();
        match p {
            Some(p) => write!(
                f,
                "Aabb(({:.p$}, {:.p$}, {:.p$})..({:.p$}, {:.p$}, {:.p$}))",
                self.min.x, self.min.y, self.min.z, self.max.x, self.max.y, self.max.z
            ),
            None => write!(
                f,
                "Aabb(({}, {}, {})..({}, {}, {}))",
                self.min.x, self.min.y, self.min.z, self.max.x, self.max.y, self.max.z
            ),
        }
    }
}

/// A sphere defined by a center and radius.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    /// Create a new sphere.
    ///
    /// # Errors
    /// Returns [`crate::HisabError::InvalidInput`] if `radius` is negative.
    #[inline]
    pub fn new(center: Vec3, radius: f32) -> Result<Self, crate::HisabError> {
        if radius < 0.0 {
            return Err(crate::HisabError::InvalidInput(
                "sphere radius must be non-negative".into(),
            ));
        }
        Ok(Self { center, radius })
    }

    /// Check whether a point is inside (or on the surface of) this sphere.
    #[must_use]
    #[inline]
    pub fn contains_point(&self, point: Vec3) -> bool {
        (point - self.center).length_squared() <= self.radius * self.radius
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = f.precision();
        let c = self.center;
        match p {
            Some(p) => write!(
                f,
                "Sphere(({:.p$}, {:.p$}, {:.p$}), r={:.p$})",
                c.x, c.y, c.z, self.radius
            ),
            None => write!(f, "Sphere(({}, {}, {}), r={})", c.x, c.y, c.z, self.radius),
        }
    }
}

/// Ray-plane intersection. Returns the `t` parameter if the ray hits the plane
/// (only `t >= 0`, i.e. forward hits).
#[must_use]
#[inline]
pub fn ray_plane(ray: &Ray, plane: &Plane) -> Option<f32> {
    let denom = plane.normal.dot(ray.direction);
    if denom.abs() < crate::EPSILON_F32 {
        return None; // Ray parallel to plane
    }
    let t = (plane.distance - plane.normal.dot(ray.origin)) / denom;
    if t >= 0.0 { Some(t) } else { None }
}

/// Ray-sphere intersection using the quadratic formula.
/// Returns the nearest `t >= 0` if the ray hits the sphere.
///
/// Assumes `ray.direction` is normalized (guaranteed by `Ray::new`),
/// so the quadratic coefficient `a = 1` and is eliminated.
#[must_use]
#[inline]
pub fn ray_sphere(ray: &Ray, sphere: &Sphere) -> Option<f32> {
    let oc = ray.origin - sphere.center;
    // With normalized direction: a=1, so b=2*dot(oc,d), c=dot(oc,oc)-r²
    // Use half-b form: half_b = dot(oc, d), discriminant = half_b² - c
    let half_b = oc.dot(ray.direction);
    let c = oc.dot(oc) - sphere.radius * sphere.radius;
    let discriminant = half_b * half_b - c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t1 = -half_b - sqrt_d;
    let t2 = -half_b + sqrt_d;

    if t1 >= 0.0 {
        Some(t1)
    } else if t2 >= 0.0 {
        Some(t2)
    } else {
        None
    }
}

/// Ray-AABB intersection using the slab method.
/// Returns the nearest `t >= 0` if the ray hits the AABB.
#[must_use]
#[inline]
pub fn ray_aabb(ray: &Ray, aabb: &Aabb) -> Option<f32> {
    let origin = ray.origin.to_array();
    let dir = ray.direction.to_array();
    let bb_min = aabb.min.to_array();
    let bb_max = aabb.max.to_array();

    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;

    for i in 0..3 {
        if dir[i].abs() < crate::EPSILON_F32 {
            if origin[i] < bb_min[i] || origin[i] > bb_max[i] {
                return None;
            }
        } else {
            let inv_d = 1.0 / dir[i];
            let mut t1 = (bb_min[i] - origin[i]) * inv_d;
            let mut t2 = (bb_max[i] - origin[i]) * inv_d;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            t_min = t_min.max(t1);
            t_max = t_max.min(t2);
            if t_min > t_max {
                return None;
            }
        }
    }

    if t_min >= 0.0 {
        Some(t_min)
    } else if t_max >= 0.0 {
        Some(t_max)
    } else {
        None
    }
}

// ---------------------------------------------------------------------------

// OBB (Oriented Bounding Box)
// ---------------------------------------------------------------------------

/// An oriented bounding box defined by a center, half-extents, and rotation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Obb {
    /// Center of the OBB.
    pub center: Vec3,
    /// Half-extents along each local axis.
    pub half_extents: Vec3,
    /// Rotation quaternion (local → world).
    pub rotation: glam::Quat,
}

impl Obb {
    /// Create a new OBB.
    #[must_use]
    #[inline]
    pub fn new(center: Vec3, half_extents: Vec3, rotation: glam::Quat) -> Self {
        Self {
            center,
            half_extents,
            rotation,
        }
    }

    /// The three local axes (columns of the rotation matrix) in world space.
    #[must_use]
    #[inline]
    pub fn axes(&self) -> [Vec3; 3] {
        let m = glam::Mat3::from_quat(self.rotation);
        [m.x_axis, m.y_axis, m.z_axis]
    }

    /// Check whether a point is inside (or on the surface of) this OBB.
    #[must_use]
    #[inline]
    pub fn contains_point(&self, point: Vec3) -> bool {
        let d = point - self.center;
        let axes = self.axes();
        let he = self.half_extents.to_array();
        for (i, axis) in axes.iter().enumerate() {
            if d.dot(*axis).abs() > he[i] + crate::EPSILON_F32 {
                return false;
            }
        }
        true
    }

    /// Closest point on this OBB to a given point.
    #[must_use]
    #[inline]
    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        let d = point - self.center;
        let axes = self.axes();
        let he = self.half_extents.to_array();
        let mut result = self.center;
        for (i, axis) in axes.iter().enumerate() {
            let dist = d.dot(*axis).clamp(-he[i], he[i]);
            result += *axis * dist;
        }
        result
    }
}

/// Ray-OBB intersection. Returns the `t` parameter if the ray hits the OBB.
#[must_use]
#[inline]
pub fn ray_obb(ray: &Ray, obb: &Obb) -> Option<f32> {
    let d = obb.center - ray.origin;
    let axes = obb.axes();
    let he = obb.half_extents.to_array();

    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;

    for i in 0..3 {
        let e = axes[i].dot(d);
        let f = axes[i].dot(ray.direction);

        if f.abs() > crate::EPSILON_F32 {
            let inv_f = 1.0 / f;
            let mut t1 = (e - he[i]) * inv_f;
            let mut t2 = (e + he[i]) * inv_f;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            t_min = t_min.max(t1);
            t_max = t_max.min(t2);
            if t_min > t_max {
                return None;
            }
        } else if (-e - he[i]) > 0.0 || (-e + he[i]) < 0.0 {
            return None;
        }
    }

    if t_min >= 0.0 {
        Some(t_min)
    } else if t_max >= 0.0 {
        Some(t_max)
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Capsule
// ---------------------------------------------------------------------------

/// A capsule defined by a line segment and a radius.
///
/// The capsule is the Minkowski sum of the segment and a sphere.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Capsule {
    /// Start point of the capsule's axis.
    pub start: Vec3,
    /// End point of the capsule's axis.
    pub end: Vec3,
    /// Radius of the capsule.
    pub radius: f32,
}

impl Capsule {
    /// Create a new capsule.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HisabError::InvalidInput`] if `radius` is negative.
    #[inline]
    pub fn new(start: Vec3, end: Vec3, radius: f32) -> Result<Self, crate::HisabError> {
        if radius < 0.0 {
            return Err(crate::HisabError::InvalidInput(
                "capsule radius must be non-negative".into(),
            ));
        }
        Ok(Self { start, end, radius })
    }

    /// Check whether a point is inside the capsule.
    #[must_use]
    #[inline]
    pub fn contains_point(&self, point: Vec3) -> bool {
        let seg = Segment::new(self.start, self.end);
        seg.distance_to_point(point) <= self.radius + crate::EPSILON_F32
    }

    /// Length of the capsule's axis (not including the hemispherical caps).
    #[must_use]
    #[inline]
    pub fn axis_length(&self) -> f32 {
        (self.end - self.start).length()
    }
}

/// Ray-capsule intersection. Returns the nearest `t >= 0` if the ray hits.
#[must_use]
pub fn ray_capsule(ray: &Ray, capsule: &Capsule) -> Option<f32> {
    // Test against the infinite cylinder, then clamp to segment + check hemispheres
    let ab = capsule.end - capsule.start;
    let ab_len_sq = ab.dot(ab);

    if ab_len_sq < crate::EPSILON_F32 {
        // Degenerate capsule: just a sphere
        let sphere = Sphere {
            center: capsule.start,
            radius: capsule.radius,
        };
        return ray_sphere(ray, &sphere);
    }

    // Closest approach of ray to segment axis
    let ao = ray.origin - capsule.start;
    let d_par = ray.direction.dot(ab) / ab_len_sq;
    let o_par = ao.dot(ab) / ab_len_sq;

    let d_perp = ray.direction - ab * d_par;
    let o_perp = ao - ab * o_par;

    let a = d_perp.dot(d_perp);
    let b = 2.0 * d_perp.dot(o_perp);
    let c = o_perp.dot(o_perp) - capsule.radius * capsule.radius;

    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        // Try sphere caps
        let s1 = Sphere {
            center: capsule.start,
            radius: capsule.radius,
        };
        let s2 = Sphere {
            center: capsule.end,
            radius: capsule.radius,
        };
        let t1 = ray_sphere(ray, &s1);
        let t2 = ray_sphere(ray, &s2);
        return match (t1, t2) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) | (None, Some(a)) => Some(a),
            _ => None,
        };
    }

    let inv_2a = 0.5 / a;
    let sqrt_disc = disc.sqrt();
    let t1 = (-b - sqrt_disc) * inv_2a;
    let t2 = (-b + sqrt_disc) * inv_2a;

    let mut best: Option<f32> = None;
    let mut check = |t: f32| {
        if t >= 0.0 {
            let p = ray.at(t);
            let proj = (p - capsule.start).dot(ab) / ab_len_sq;
            if (0.0..=1.0).contains(&proj) {
                best = Some(best.map_or(t, |b: f32| b.min(t)));
            }
        }
    };
    check(t1);
    check(t2);

    // Also check hemisphere caps
    let s1 = Sphere {
        center: capsule.start,
        radius: capsule.radius,
    };
    let s2 = Sphere {
        center: capsule.end,
        radius: capsule.radius,
    };
    if let Some(t) = ray_sphere(ray, &s1) {
        let p = ray.at(t);
        if (p - capsule.start).dot(ab) <= 0.0 {
            best = Some(best.map_or(t, |b: f32| b.min(t)));
        }
    }
    if let Some(t) = ray_sphere(ray, &s2) {
        let p = ray.at(t);
        if (p - capsule.end).dot(ab) >= 0.0 {
            best = Some(best.map_or(t, |b: f32| b.min(t)));
        }
    }

    best
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Mat4, Vec3};

    const EPS: f32 = 1e-5;

    fn approx_vec3(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < EPS
    }

    // --- Aabb::transformed tests --------------------------------------------

    #[test]
    fn transformed_identity_unchanged() {
        let aabb = Aabb::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(1.0, 2.0, 3.0));
        let result = aabb.transformed(Mat4::IDENTITY);
        assert!(approx_vec3(result.min, aabb.min));
        assert!(approx_vec3(result.max, aabb.max));
    }

    #[test]
    fn transformed_translation_only() {
        let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let t = Mat4::from_translation(Vec3::new(3.0, 5.0, -2.0));
        let result = aabb.transformed(t);
        assert!(approx_vec3(result.min, Vec3::new(2.0, 4.0, -3.0)));
        assert!(approx_vec3(result.max, Vec3::new(4.0, 6.0, -1.0)));
    }

    #[test]
    fn transformed_uniform_scale() {
        let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let s = Mat4::from_scale(Vec3::splat(2.0));
        let result = aabb.transformed(s);
        assert!(approx_vec3(result.min, Vec3::splat(-2.0)));
        assert!(approx_vec3(result.max, Vec3::splat(2.0)));
    }

    #[test]
    fn transformed_90_deg_rotation() {
        // Rotate 90° around Z: (x,y,z) → (-y, x, z)
        // An AABB [0,1]×[0,1]×[0,1] should become [-1,0]×[0,1]×[0,1]
        let aabb = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let r = Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2);
        let result = aabb.transformed(r);
        // new_min.x = -1, new_max.x = 0 (within rounding)
        assert!(
            (result.min.x - (-1.0)).abs() < 1e-5,
            "min.x = {}",
            result.min.x
        );
        assert!(
            (result.max.x - 0.0).abs() < 1e-5,
            "max.x = {}",
            result.max.x
        );
        assert!(
            (result.min.y - 0.0).abs() < 1e-5,
            "min.y = {}",
            result.min.y
        );
        assert!(
            (result.max.y - 1.0).abs() < 1e-5,
            "max.y = {}",
            result.max.y
        );
    }

    #[test]
    fn transformed_result_min_le_max() {
        // Regardless of the transform, min should never exceed max.
        let aabb = Aabb::new(Vec3::new(-2.0, -3.0, -1.0), Vec3::new(2.0, 3.0, 1.0));
        let m = Mat4::from_cols_array(&[
            -1.0, 0.5, 0.0, 0.0, 0.3, 2.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 2.0, 3.0, 1.0,
        ]);
        let result = aabb.transformed(m);
        assert!(result.min.x <= result.max.x);
        assert!(result.min.y <= result.max.y);
        assert!(result.min.z <= result.max.z);
    }
}
