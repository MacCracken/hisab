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

// ---------------------------------------------------------------------------
