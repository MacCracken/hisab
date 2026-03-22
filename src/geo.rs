//! Geometric primitives and intersection tests.
//!
//! Provides rays, planes, axis-aligned bounding boxes, spheres, and
//! ray-intersection routines.

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// A ray defined by an origin and a direction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vec3,
    /// Should be normalized for correct distance results.
    pub direction: Vec3,
}

impl Ray {
    /// Create a new ray. Direction is normalized automatically.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Point along the ray at parameter `t`.
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
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
    pub fn from_point_normal(point: Vec3, normal: Vec3) -> Self {
        let n = normal.normalize();
        Self {
            normal: n,
            distance: n.dot(point),
        }
    }

    /// Signed distance from a point to the plane.
    /// Positive = same side as normal, negative = opposite side.
    pub fn signed_distance(&self, point: Vec3) -> f32 {
        self.normal.dot(point) - self.distance
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
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }

    /// Check whether a point is inside (or on the boundary of) this AABB.
    #[inline]
    pub fn contains(&self, point: Vec3) -> bool {
        point.cmpge(self.min).all() && point.cmple(self.max).all()
    }

    /// Center point of the AABB.
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// Size (extents) of the AABB.
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// Merge two AABBs into one that encloses both.
    pub fn merge(&self, other: &Aabb) -> Aabb {
        Aabb {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
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
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Check whether a point is inside (or on the surface of) this sphere.
    #[inline]
    pub fn contains_point(&self, point: Vec3) -> bool {
        (point - self.center).length_squared() <= self.radius * self.radius
    }
}

/// Ray-plane intersection. Returns the `t` parameter if the ray hits the plane
/// (only `t >= 0`, i.e. forward hits).
#[inline]
pub fn ray_plane(ray: &Ray, plane: &Plane) -> Option<f32> {
    let denom = plane.normal.dot(ray.direction);
    if denom.abs() < 1e-8 {
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
#[inline]
pub fn ray_aabb(ray: &Ray, aabb: &Aabb) -> Option<f32> {
    let origin = ray.origin.to_array();
    let dir = ray.direction.to_array();
    let bb_min = aabb.min.to_array();
    let bb_max = aabb.max.to_array();

    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;

    for i in 0..3 {
        if dir[i].abs() < 1e-8 {
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

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-4;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn ray_at_parameter() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        assert_eq!(r.at(0.0), Vec3::ZERO);
        assert!(approx_eq(r.at(5.0).x, 5.0));
    }

    #[test]
    fn plane_from_point_normal() {
        let p = Plane::from_point_normal(Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
        assert!(approx_eq(p.distance, 1.0));
        assert!(approx_eq(p.signed_distance(Vec3::new(0.0, 2.0, 0.0)), 1.0));
        assert!(approx_eq(
            p.signed_distance(Vec3::new(0.0, 0.0, 0.0)),
            -1.0
        ));
    }

    #[test]
    fn aabb_contains() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(bb.contains(Vec3::splat(0.5)));
        assert!(!bb.contains(Vec3::splat(2.0)));
        assert!(bb.contains(Vec3::ZERO));
    }

    #[test]
    fn aabb_center_and_size() {
        let bb = Aabb::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(bb.center(), Vec3::ZERO);
        assert_eq!(bb.size(), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn aabb_merge() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.5, 0.5, 0.5));
        let merged = a.merge(&b);
        assert_eq!(merged.min, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(merged.max, Vec3::ONE);
    }

    #[test]
    fn sphere_contains() {
        let s = Sphere::new(Vec3::ZERO, 1.0);
        assert!(s.contains_point(Vec3::ZERO));
        assert!(s.contains_point(Vec3::new(1.0, 0.0, 0.0)));
        assert!(!s.contains_point(Vec3::new(1.1, 0.0, 0.0)));
    }

    #[test]
    fn ray_plane_intersection() {
        let r = Ray::new(Vec3::ZERO, Vec3::Y);
        let p = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
        let t = ray_plane(&r, &p).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_plane_parallel_no_hit() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        let p = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
        assert!(ray_plane(&r, &p).is_none());
    }

    #[test]
    fn ray_sphere_hit() {
        let r = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z);
        let s = Sphere::new(Vec3::ZERO, 1.0);
        let t = ray_sphere(&r, &s).unwrap();
        assert!(approx_eq(t, 4.0));
    }

    #[test]
    fn ray_sphere_miss() {
        let r = Ray::new(Vec3::new(0.0, 5.0, -5.0), Vec3::Z);
        let s = Sphere::new(Vec3::ZERO, 1.0);
        assert!(ray_sphere(&r, &s).is_none());
    }

    #[test]
    fn ray_aabb_hit() {
        let r = Ray::new(Vec3::new(0.5, 0.5, -5.0), Vec3::Z);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let t = ray_aabb(&r, &bb).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_aabb_miss() {
        let r = Ray::new(Vec3::new(5.0, 5.0, -5.0), Vec3::Z);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_none());
    }

    #[test]
    fn ray_inside_sphere() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        let s = Sphere::new(Vec3::ZERO, 10.0);
        let t = ray_sphere(&r, &s).unwrap();
        assert!(t > 0.0);
        assert!(approx_eq(t, 10.0));
    }

    #[test]
    fn ray_inside_aabb() {
        let r = Ray::new(Vec3::splat(0.5), Vec3::X);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let t = ray_aabb(&r, &bb).unwrap();
        assert!(t >= 0.0);
    }

    #[test]
    fn aabb_auto_corrects_min_max() {
        let bb = Aabb::new(Vec3::ONE, Vec3::ZERO);
        assert_eq!(bb.min, Vec3::ZERO);
        assert_eq!(bb.max, Vec3::ONE);
    }

    #[test]
    fn ray_normalizes_direction() {
        let r = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 10.0));
        let len = r.direction.length();
        assert!(approx_eq(len, 1.0));
        assert!(approx_eq(r.direction.z, 1.0));
    }

    #[test]
    fn ray_at_negative_parameter() {
        let r = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::X);
        let p = r.at(-2.0);
        assert!(approx_eq(p.x, -1.0));
    }

    #[test]
    fn plane_signed_distance_on_plane() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        assert!(approx_eq(p.signed_distance(Vec3::new(5.0, 0.0, -3.0)), 0.0));
    }

    #[test]
    fn plane_non_axis_normal() {
        let normal = Vec3::new(1.0, 1.0, 0.0);
        let p = Plane::from_point_normal(Vec3::ZERO, normal);
        assert!(approx_eq(p.normal.length(), 1.0));
        assert!(approx_eq(p.distance, 0.0));
    }

    #[test]
    fn aabb_contains_boundary_max() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(bb.contains(Vec3::ONE));
    }

    #[test]
    fn aabb_merge_identical() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let merged = bb.merge(&bb);
        assert_eq!(merged.min, Vec3::ZERO);
        assert_eq!(merged.max, Vec3::ONE);
    }

    #[test]
    fn aabb_merge_disjoint() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::splat(5.0), Vec3::splat(6.0));
        let merged = a.merge(&b);
        assert_eq!(merged.min, Vec3::ZERO);
        assert_eq!(merged.max, Vec3::splat(6.0));
    }

    #[test]
    fn sphere_surface_point() {
        let s = Sphere::new(Vec3::ZERO, 5.0);
        assert!(s.contains_point(Vec3::new(5.0, 0.0, 0.0)));
        assert!(s.contains_point(Vec3::new(0.0, -5.0, 0.0)));
    }

    #[test]
    fn sphere_offset_center() {
        let s = Sphere::new(Vec3::new(10.0, 0.0, 0.0), 1.0);
        assert!(s.contains_point(Vec3::new(10.5, 0.0, 0.0)));
        assert!(!s.contains_point(Vec3::ZERO));
    }

    #[test]
    fn ray_plane_behind_origin() {
        let r = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::Z);
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Z);
        assert!(ray_plane(&r, &p).is_none());
    }

    #[test]
    fn ray_sphere_tangent() {
        let s = Sphere::new(Vec3::ZERO, 1.0);
        let r = Ray::new(Vec3::new(0.0, 1.0, -5.0), Vec3::Z);
        let t = ray_sphere(&r, &s);
        assert!(t.is_some());
        assert!(approx_eq(t.unwrap(), 5.0));
    }

    #[test]
    fn ray_sphere_behind_ray() {
        let r = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::Z);
        let s = Sphere::new(Vec3::ZERO, 1.0);
        assert!(ray_sphere(&r, &s).is_none());
    }

    #[test]
    fn ray_aabb_axis_aligned_hit() {
        let r = Ray::new(Vec3::new(-5.0, 0.5, 0.5), Vec3::X);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let t = ray_aabb(&r, &bb).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_aabb_parallel_to_slab_inside() {
        let r = Ray::new(Vec3::new(-5.0, 0.5, 0.5), Vec3::X);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_some());
    }

    #[test]
    fn ray_aabb_parallel_to_slab_outside() {
        let r = Ray::new(Vec3::new(-5.0, 5.0, 0.5), Vec3::X);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_none());
    }

    #[test]
    fn geo_error_display() {
        use crate::GanitError;
        let e = GanitError::Degenerate("zero-length edge".to_string());
        assert_eq!(e.to_string(), "degenerate geometry: zero-length edge");
    }

    #[test]
    fn ray_serde_roundtrip() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::Y);
        let json = serde_json::to_string(&r).unwrap();
        let r2: Ray = serde_json::from_str(&json).unwrap();
        assert_eq!(r, r2);
    }

    #[test]
    fn aabb_serde_roundtrip() {
        let bb = Aabb::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(4.0, 5.0, 6.0));
        let json = serde_json::to_string(&bb).unwrap();
        let bb2: Aabb = serde_json::from_str(&json).unwrap();
        assert_eq!(bb, bb2);
    }

    #[test]
    fn sphere_serde_roundtrip() {
        let s = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 5.0);
        let json = serde_json::to_string(&s).unwrap();
        let s2: Sphere = serde_json::from_str(&json).unwrap();
        assert_eq!(s, s2);
    }

    #[test]
    fn aabb_zero_size() {
        let bb = Aabb::new(Vec3::splat(3.0), Vec3::splat(3.0));
        assert_eq!(bb.size(), Vec3::ZERO);
        assert_eq!(bb.center(), Vec3::splat(3.0));
        assert!(bb.contains(Vec3::splat(3.0)));
    }

    #[test]
    fn ray_plane_intersection_at_angle() {
        let r = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 1.0));
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let t = ray_plane(&r, &p).unwrap();
        let hit = r.at(t);
        assert!(approx_eq(hit.y, 0.0));
    }

    #[test]
    fn ray_sphere_optimized_matches_distance() {
        // Verify half-b optimization gives correct hit distance
        let r = Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::Z);
        let s = Sphere::new(Vec3::ZERO, 2.0);
        let t = ray_sphere(&r, &s).unwrap();
        assert!(approx_eq(t, 8.0)); // 10 - 2
        let hit = r.at(t);
        assert!(approx_eq(hit.z, -2.0));
    }

    #[test]
    fn ray_sphere_large_radius() {
        let r = Ray::new(Vec3::new(0.0, 0.0, -1000.0), Vec3::Z);
        let s = Sphere::new(Vec3::ZERO, 500.0);
        let t = ray_sphere(&r, &s).unwrap();
        assert!(approx_eq(t, 500.0));
    }

    #[test]
    fn aabb_contains_just_outside() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(!bb.contains(Vec3::new(1.001, 0.5, 0.5)));
        assert!(!bb.contains(Vec3::new(0.5, -0.001, 0.5)));
        assert!(!bb.contains(Vec3::new(0.5, 0.5, 1.001)));
    }

    #[test]
    fn aabb_contains_all_corners() {
        let bb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        // All 8 corners should be contained
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    assert!(bb.contains(Vec3::new(x, y, z)));
                }
            }
        }
    }

    #[test]
    fn ray_aabb_diagonal_ray() {
        // Diagonal ray through AABB
        let r = Ray::new(Vec3::new(-5.0, -5.0, -5.0), Vec3::new(1.0, 1.0, 1.0));
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_some());
    }

    #[test]
    fn plane_signed_distance_both_sides() {
        let p = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
        assert!(p.signed_distance(Vec3::new(0.0, 10.0, 0.0)) > 0.0);
        assert!(p.signed_distance(Vec3::new(0.0, 0.0, 0.0)) < 0.0);
        assert!(approx_eq(p.signed_distance(Vec3::new(0.0, 5.0, 0.0)), 0.0));
    }
}
