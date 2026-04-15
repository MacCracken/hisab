use super::*;

// Closest-point functions
// ---------------------------------------------------------------------------

/// Closest point on a ray to a given point (clamped to `t >= 0`).
#[must_use]
#[inline]
pub fn closest_point_on_ray(ray: &Ray, point: Vec3) -> Vec3 {
    let t = (point - ray.origin).dot(ray.direction).max(0.0);
    ray.origin + ray.direction * t
}

/// Closest point on a plane to a given point.
#[must_use]
#[inline]
pub fn closest_point_on_plane(plane: &Plane, point: Vec3) -> Vec3 {
    point - plane.normal * plane.signed_distance(point)
}

/// Closest point on a sphere's surface to a given point.
///
/// If the point is at the sphere's center, returns the point offset by the radius along +X.
#[must_use]
#[inline]
pub fn closest_point_on_sphere(sphere: &Sphere, point: Vec3) -> Vec3 {
    let dir = point - sphere.center;
    let len = dir.length();
    if len < crate::EPSILON_F32 {
        return sphere.center + Vec3::new(sphere.radius, 0.0, 0.0);
    }
    sphere.center + dir * (sphere.radius / len)
}

/// Closest point on an AABB's surface or interior to a given point.
#[must_use]
#[inline]
pub fn closest_point_on_aabb(aabb: &Aabb, point: Vec3) -> Vec3 {
    point.clamp(aabb.min, aabb.max)
}

/// Closest point on a triangle to a given point (3D).
///
/// Uses Voronoi region testing to determine whether the closest point lies
/// on a vertex, edge, or the interior of the triangle.
#[must_use]
pub fn closest_point_on_triangle(triangle: &Triangle, point: Vec3) -> Vec3 {
    let a = triangle.vertices[0];
    let b = triangle.vertices[1];
    let c = triangle.vertices[2];
    let ab = b - a;
    let ac = c - a;
    let ap = point - a;

    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return a; // Vertex A region
    }

    let bp = point - b;
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= 0.0 && d4 <= d3 {
        return b; // Vertex B region
    }

    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return a + ab * v; // Edge AB
    }

    let cp = point - c;
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= 0.0 && d5 <= d6 {
        return c; // Vertex C region
    }

    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return a + ac * w; // Edge AC
    }

    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return b + (c - b) * w; // Edge BC
    }

    // Interior
    let denom = 1.0 / (va + vb + vc);
    let v = vb * denom;
    let w = vc * denom;
    a + ab * v + ac * w
}

/// Barycentric coordinates of a point projected onto a triangle (3D).
///
/// Returns `(u, v, w)` such that `point ≈ u*a + v*b + w*c` and `u + v + w = 1`.
/// The point is inside the triangle if all coordinates are in \[0, 1\].
#[must_use]
#[inline]
pub fn barycentric_coords(triangle: &Triangle, point: Vec3) -> (f32, f32, f32) {
    let v0 = triangle.vertices[1] - triangle.vertices[0];
    let v1 = triangle.vertices[2] - triangle.vertices[0];
    let v2 = point - triangle.vertices[0];

    let d00 = v0.dot(v0);
    let d01 = v0.dot(v1);
    let d11 = v1.dot(v1);
    let d20 = v2.dot(v0);
    let d21 = v2.dot(v1);

    let denom = d00 * d11 - d01 * d01;
    if denom.abs() < crate::EPSILON_F32 {
        // Degenerate triangle — return centroid weights
        return (1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0);
    }
    let inv_denom = 1.0 / denom;
    let v = (d11 * d20 - d01 * d21) * inv_denom;
    let w = (d00 * d21 - d01 * d20) * inv_denom;
    let u = 1.0 - v - w;
    (u, v, w)
}

/// Closest points between two 3D line segments, and the squared distance.
///
/// Returns `(point_on_seg1, point_on_seg2, squared_distance)`.
/// Used for capsule-capsule collision, edge-edge contact generation,
/// and wire/rope physics.
#[must_use]
pub fn segment_segment_closest(a0: Vec3, a1: Vec3, b0: Vec3, b1: Vec3) -> (Vec3, Vec3, f32) {
    let d1 = a1 - a0;
    let d2 = b1 - b0;
    let r = a0 - b0;

    let a = d1.dot(d1);
    let e = d2.dot(d2);
    let f = d2.dot(r);

    // Both segments degenerate to points
    if a < crate::EPSILON_F32 && e < crate::EPSILON_F32 {
        let dist_sq = r.dot(r);
        return (a0, b0, dist_sq);
    }

    let (s, t);
    if a < crate::EPSILON_F32 {
        // First segment degenerates to a point
        s = 0.0;
        t = (f / e).clamp(0.0, 1.0);
    } else {
        let c = d1.dot(r);
        if e < crate::EPSILON_F32 {
            // Second segment degenerates to a point
            t = 0.0;
            s = (-c / a).clamp(0.0, 1.0);
        } else {
            // General case
            let b_val = d1.dot(d2);
            let denom = a * e - b_val * b_val;

            s = if denom.abs() < crate::EPSILON_F32 {
                0.0 // Parallel segments
            } else {
                ((b_val * f - c * e) / denom).clamp(0.0, 1.0)
            };

            // Compute t from s
            let t_num = b_val * s + f;
            t = if t_num < 0.0 {
                let s_new = (-c / a).clamp(0.0, 1.0);
                let p1 = a0 + d1 * s_new;
                let p2 = b0;
                return (p1, p2, (p1 - p2).length_squared());
            } else if t_num > e {
                let s_new = ((b_val - c) / a).clamp(0.0, 1.0);
                let p1 = a0 + d1 * s_new;
                let p2 = b1;
                return (p1, p2, (p1 - p2).length_squared());
            } else {
                t_num / e
            };
        }
    }

    let p1 = a0 + d1 * s;
    let p2 = b0 + d2 * t;
    (p1, p2, (p1 - p2).length_squared())
}

// ---------------------------------------------------------------------------
// Tangent space computation
// ---------------------------------------------------------------------------

/// Compute tangent and bitangent vectors for a triangle given UV coordinates.
///
/// This is the standard approach used for normal mapping (compatible with
/// Mikktspace conventions when averaged over a mesh). Returns `(tangent, bitangent)`.
///
/// The tangent vector points in the direction of increasing U, and the bitangent
/// in the direction of increasing V, both in the plane of the triangle.
#[must_use]
pub fn compute_tangent(
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    uv0: glam::Vec2,
    uv1: glam::Vec2,
    uv2: glam::Vec2,
) -> (Vec3, Vec3) {
    let edge1 = p1 - p0;
    let edge2 = p2 - p0;
    let duv1 = uv1 - uv0;
    let duv2 = uv2 - uv0;

    let denom = duv1.x * duv2.y - duv2.x * duv1.y;
    if denom.abs() < crate::EPSILON_F32 {
        // Degenerate UV — return a perpendicular frame from the normal
        let normal = edge1.cross(edge2);
        let tangent = if normal.x.abs() < 0.9 {
            Vec3::X.cross(normal).normalize_or_zero()
        } else {
            Vec3::Y.cross(normal).normalize_or_zero()
        };
        let bitangent = normal.cross(tangent).normalize_or_zero();
        return (tangent, bitangent);
    }

    let inv = 1.0 / denom;
    let tangent = (edge1 * duv2.y - edge2 * duv1.y) * inv;
    let bitangent = (edge2 * duv1.x - edge1 * duv2.x) * inv;

    (tangent.normalize_or_zero(), bitangent.normalize_or_zero())
}
