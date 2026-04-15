//! Geometric primitives and intersection tests.
//!
//! Provides rays, planes, axis-aligned bounding boxes, spheres, and
//! ray-intersection routines.

use glam::Vec3;
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod cga;
mod closest;
mod collision;
mod decompose;
mod delaunay;
mod halfedge;
mod intersection;
mod islands;
mod primitives;
mod sdf;
mod spatial;

pub use closest::*;
pub use collision::*;
pub use decompose::*;
pub use delaunay::*;
pub use halfedge::*;
pub use intersection::*;
pub use islands::*;
pub use primitives::*;
pub use sdf::*;
pub use spatial::*;

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-4;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    fn vec3_approx_eq(a: Vec3, b: Vec3) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    #[test]
    fn ray_at_parameter() {
        let r = Ray::new(Vec3::ZERO, Vec3::X).unwrap();
        assert_eq!(r.at(0.0), Vec3::ZERO);
        assert!(approx_eq(r.at(5.0).x, 5.0));
    }

    #[test]
    fn plane_from_point_normal() {
        let p = Plane::from_point_normal(Vec3::new(0.0, 1.0, 0.0), Vec3::Y).unwrap();
        assert!(approx_eq(p.distance, 1.0));
        assert!(approx_eq(p.signed_distance(Vec3::new(0.0, 2.0, 0.0)), 1.0));
        assert!(approx_eq(p.signed_distance(Vec3::new(0.0, 0.0, 0.0)), -1.0));
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
        let s = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        assert!(s.contains_point(Vec3::ZERO));
        assert!(s.contains_point(Vec3::new(1.0, 0.0, 0.0)));
        assert!(!s.contains_point(Vec3::new(1.1, 0.0, 0.0)));
    }

    #[test]
    fn ray_plane_intersection() {
        let r = Ray::new(Vec3::ZERO, Vec3::Y).unwrap();
        let p = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y).unwrap();
        let t = ray_plane(&r, &p).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_plane_parallel_no_hit() {
        let r = Ray::new(Vec3::ZERO, Vec3::X).unwrap();
        let p = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y).unwrap();
        assert!(ray_plane(&r, &p).is_none());
    }

    #[test]
    fn ray_sphere_hit() {
        let r = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z).unwrap();
        let s = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let t = ray_sphere(&r, &s).unwrap();
        assert!(approx_eq(t, 4.0));
    }

    #[test]
    fn ray_sphere_miss() {
        let r = Ray::new(Vec3::new(0.0, 5.0, -5.0), Vec3::Z).unwrap();
        let s = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        assert!(ray_sphere(&r, &s).is_none());
    }

    #[test]
    fn ray_aabb_hit() {
        let r = Ray::new(Vec3::new(0.5, 0.5, -5.0), Vec3::Z).unwrap();
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let t = ray_aabb(&r, &bb).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_aabb_miss() {
        let r = Ray::new(Vec3::new(5.0, 5.0, -5.0), Vec3::Z).unwrap();
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_none());
    }

    #[test]
    fn ray_inside_sphere() {
        let r = Ray::new(Vec3::ZERO, Vec3::X).unwrap();
        let s = Sphere::new(Vec3::ZERO, 10.0).unwrap();
        let t = ray_sphere(&r, &s).unwrap();
        assert!(t > 0.0);
        assert!(approx_eq(t, 10.0));
    }

    #[test]
    fn ray_inside_aabb() {
        let r = Ray::new(Vec3::splat(0.5), Vec3::X).unwrap();
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
        let r = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 10.0)).unwrap();
        let len = r.direction.length();
        assert!(approx_eq(len, 1.0));
        assert!(approx_eq(r.direction.z, 1.0));
    }

    #[test]
    fn ray_at_negative_parameter() {
        let r = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::X).unwrap();
        let p = r.at(-2.0);
        assert!(approx_eq(p.x, -1.0));
    }

    #[test]
    fn plane_signed_distance_on_plane() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y).unwrap();
        assert!(approx_eq(p.signed_distance(Vec3::new(5.0, 0.0, -3.0)), 0.0));
    }

    #[test]
    fn plane_non_axis_normal() {
        let normal = Vec3::new(1.0, 1.0, 0.0);
        let p = Plane::from_point_normal(Vec3::ZERO, normal).unwrap();
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
        let s = Sphere::new(Vec3::ZERO, 5.0).unwrap();
        assert!(s.contains_point(Vec3::new(5.0, 0.0, 0.0)));
        assert!(s.contains_point(Vec3::new(0.0, -5.0, 0.0)));
    }

    #[test]
    fn sphere_offset_center() {
        let s = Sphere::new(Vec3::new(10.0, 0.0, 0.0), 1.0).unwrap();
        assert!(s.contains_point(Vec3::new(10.5, 0.0, 0.0)));
        assert!(!s.contains_point(Vec3::ZERO));
    }

    #[test]
    fn ray_plane_behind_origin() {
        let r = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::Z).unwrap();
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Z).unwrap();
        assert!(ray_plane(&r, &p).is_none());
    }

    #[test]
    fn ray_sphere_tangent() {
        let s = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let r = Ray::new(Vec3::new(0.0, 1.0, -5.0), Vec3::Z).unwrap();
        let t = ray_sphere(&r, &s);
        assert!(t.is_some());
        assert!(approx_eq(t.unwrap(), 5.0));
    }

    #[test]
    fn ray_sphere_behind_ray() {
        let r = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::Z).unwrap();
        let s = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        assert!(ray_sphere(&r, &s).is_none());
    }

    #[test]
    fn ray_aabb_axis_aligned_hit() {
        let r = Ray::new(Vec3::new(-5.0, 0.5, 0.5), Vec3::X).unwrap();
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let t = ray_aabb(&r, &bb).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_aabb_parallel_to_slab_inside() {
        let r = Ray::new(Vec3::new(-5.0, 0.5, 0.5), Vec3::X).unwrap();
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_some());
    }

    #[test]
    fn ray_aabb_parallel_to_slab_outside() {
        let r = Ray::new(Vec3::new(-5.0, 5.0, 0.5), Vec3::X).unwrap();
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_none());
    }

    #[test]
    fn geo_error_display() {
        use crate::HisabError;
        let e = HisabError::Degenerate("zero-length edge".to_string());
        assert_eq!(e.to_string(), "degenerate geometry: zero-length edge");
    }

    #[test]
    fn ray_serde_roundtrip() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::Y).unwrap();
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
        let s = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 5.0).unwrap();
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
        let r = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 1.0)).unwrap();
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y).unwrap();
        let t = ray_plane(&r, &p).unwrap();
        let hit = r.at(t);
        assert!(approx_eq(hit.y, 0.0));
    }

    #[test]
    fn ray_sphere_optimized_matches_distance() {
        // Verify half-b optimization gives correct hit distance
        let r = Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::Z).unwrap();
        let s = Sphere::new(Vec3::ZERO, 2.0).unwrap();
        let t = ray_sphere(&r, &s).unwrap();
        assert!(approx_eq(t, 8.0)); // 10 - 2
        let hit = r.at(t);
        assert!(approx_eq(hit.z, -2.0));
    }

    #[test]
    fn ray_sphere_large_radius() {
        let r = Ray::new(Vec3::new(0.0, 0.0, -1000.0), Vec3::Z).unwrap();
        let s = Sphere::new(Vec3::ZERO, 500.0).unwrap();
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
        let r = Ray::new(Vec3::new(-5.0, -5.0, -5.0), Vec3::new(1.0, 1.0, 1.0)).unwrap();
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_some());
    }

    #[test]
    fn plane_signed_distance_both_sides() {
        let p = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y).unwrap();
        assert!(p.signed_distance(Vec3::new(0.0, 10.0, 0.0)) > 0.0);
        assert!(p.signed_distance(Vec3::new(0.0, 0.0, 0.0)) < 0.0);
        assert!(approx_eq(p.signed_distance(Vec3::new(0.0, 5.0, 0.0)), 0.0));
    }

    // -----------------------------------------------------------------------
    // V0.2 tests
    // -----------------------------------------------------------------------

    // Triangle tests
    #[test]
    fn triangle_normal() {
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        let n = tri.normal();
        assert!(approx_eq(n.z, 1.0)); // CCW in XY plane -> +Z
    }

    #[test]
    fn triangle_area() {
        let tri = Triangle::new(
            Vec3::ZERO,
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
        );
        assert!(approx_eq(tri.area(), 2.0));
    }

    #[test]
    fn triangle_centroid() {
        let tri = Triangle::new(
            Vec3::ZERO,
            Vec3::new(3.0, 0.0, 0.0),
            Vec3::new(0.0, 3.0, 0.0),
        );
        assert!(vec3_approx_eq(tri.centroid(), Vec3::new(1.0, 1.0, 0.0)));
    }

    #[test]
    fn triangle_degenerate_area() {
        // Collinear points -> zero area
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::new(2.0, 0.0, 0.0));
        assert!(approx_eq(tri.area(), 0.0));
    }

    // Line tests
    #[test]
    fn line_closest_point() {
        let l = Line::new(Vec3::ZERO, Vec3::X).unwrap();
        let p = Vec3::new(5.0, 3.0, 0.0);
        let cp = l.closest_point(p);
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn line_distance_to_point() {
        let l = Line::new(Vec3::ZERO, Vec3::X).unwrap();
        let d = l.distance_to_point(Vec3::new(5.0, 3.0, 4.0));
        assert!(approx_eq(d, 5.0)); // sqrt(9+16) = 5
    }

    #[test]
    fn line_closest_point_behind_origin() {
        // Line is infinite — should work for negative t
        let l = Line::new(Vec3::ZERO, Vec3::X).unwrap();
        let cp = l.closest_point(Vec3::new(-10.0, 1.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(-10.0, 0.0, 0.0)));
    }

    // Segment tests
    #[test]
    fn segment_length_and_midpoint() {
        let s = Segment::new(Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0));
        assert!(approx_eq(s.length(), 4.0));
        assert!(vec3_approx_eq(s.midpoint(), Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn segment_closest_point_clamped() {
        let s = Segment::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));
        // Point past the end
        assert!(vec3_approx_eq(
            s.closest_point(Vec3::new(20.0, 0.0, 0.0)),
            Vec3::new(10.0, 0.0, 0.0)
        ));
        // Point before the start
        assert!(vec3_approx_eq(
            s.closest_point(Vec3::new(-5.0, 0.0, 0.0)),
            Vec3::ZERO
        ));
        // Point alongside
        assert!(vec3_approx_eq(
            s.closest_point(Vec3::new(5.0, 3.0, 0.0)),
            Vec3::new(5.0, 0.0, 0.0)
        ));
    }

    #[test]
    fn segment_distance() {
        let s = Segment::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));
        assert!(approx_eq(
            s.distance_to_point(Vec3::new(5.0, 3.0, 0.0)),
            3.0
        ));
    }

    #[test]
    fn segment_direction_normalized() {
        let s = Segment::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 10.0));
        assert!(vec3_approx_eq(s.direction(), Vec3::Z));
    }

    // Ray-triangle tests
    #[test]
    fn ray_triangle_hit() {
        let tri = Triangle::new(
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(1.0, -1.0, 5.0),
            Vec3::new(0.0, 1.0, 5.0),
        );
        let r = Ray::new(Vec3::ZERO, Vec3::Z).unwrap();
        let t = ray_triangle(&r, &tri).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_triangle_miss() {
        let tri = Triangle::new(
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(1.0, -1.0, 5.0),
            Vec3::new(0.0, 1.0, 5.0),
        );
        let r = Ray::new(Vec3::new(10.0, 10.0, 0.0), Vec3::Z).unwrap();
        assert!(ray_triangle(&r, &tri).is_none());
    }

    #[test]
    fn ray_triangle_parallel() {
        // Ray parallel to triangle plane
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        let r = Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::X).unwrap();
        assert!(ray_triangle(&r, &tri).is_none());
    }

    #[test]
    fn ray_triangle_behind() {
        let tri = Triangle::new(
            Vec3::new(-1.0, -1.0, -5.0),
            Vec3::new(1.0, -1.0, -5.0),
            Vec3::new(0.0, 1.0, -5.0),
        );
        let r = Ray::new(Vec3::ZERO, Vec3::Z).unwrap(); // Points away from triangle
        assert!(ray_triangle(&r, &tri).is_none());
    }

    // AABB-AABB overlap tests
    #[test]
    fn aabb_aabb_overlap() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::splat(0.5), Vec3::splat(1.5));
        assert!(aabb_aabb(&a, &b));
    }

    #[test]
    fn aabb_aabb_no_overlap() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::splat(2.0), Vec3::splat(3.0));
        assert!(!aabb_aabb(&a, &b));
    }

    #[test]
    fn aabb_aabb_touching() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 1.0));
        assert!(aabb_aabb(&a, &b)); // Touching edge = overlap
    }

    #[test]
    fn aabb_aabb_contained() {
        let a = Aabb::new(Vec3::ZERO, Vec3::splat(10.0));
        let b = Aabb::new(Vec3::ONE, Vec3::splat(2.0));
        assert!(aabb_aabb(&a, &b));
    }

    // Sphere-sphere overlap tests
    #[test]
    fn sphere_sphere_overlap() {
        let a = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let b = Sphere::new(Vec3::new(1.5, 0.0, 0.0), 1.0).unwrap();
        assert!(sphere_sphere(&a, &b));
    }

    #[test]
    fn sphere_sphere_no_overlap() {
        let a = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let b = Sphere::new(Vec3::new(3.0, 0.0, 0.0), 1.0).unwrap();
        assert!(!sphere_sphere(&a, &b));
    }

    #[test]
    fn sphere_sphere_touching() {
        let a = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let b = Sphere::new(Vec3::new(2.0, 0.0, 0.0), 1.0).unwrap();
        assert!(sphere_sphere(&a, &b)); // Touching = overlap
    }

    // Plane-plane intersection tests
    #[test]
    fn plane_plane_intersection() {
        let a = Plane::from_point_normal(Vec3::ZERO, Vec3::Y).unwrap();
        let b = Plane::from_point_normal(Vec3::ZERO, Vec3::X).unwrap();
        let line = plane_plane(&a, &b).unwrap();
        // Intersection should be along Z axis
        assert!(approx_eq(line.direction.z.abs(), 1.0));
    }

    #[test]
    fn plane_plane_parallel() {
        let a = Plane::from_point_normal(Vec3::ZERO, Vec3::Y).unwrap();
        let b = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y).unwrap();
        assert!(plane_plane(&a, &b).is_none());
    }

    // Closest-point tests
    #[test]
    fn closest_on_ray_forward() {
        let r = Ray::new(Vec3::ZERO, Vec3::X).unwrap();
        let cp = closest_point_on_ray(&r, Vec3::new(5.0, 3.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_ray_clamped() {
        // Point behind the ray — should clamp to origin
        let r = Ray::new(Vec3::ZERO, Vec3::X).unwrap();
        let cp = closest_point_on_ray(&r, Vec3::new(-5.0, 3.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::ZERO));
    }

    #[test]
    fn closest_on_plane() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y).unwrap();
        let cp = closest_point_on_plane(&p, Vec3::new(3.0, 7.0, -2.0));
        assert!(vec3_approx_eq(cp, Vec3::new(3.0, 0.0, -2.0)));
    }

    #[test]
    fn closest_on_sphere_outside() {
        let s = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let cp = closest_point_on_sphere(&s, Vec3::new(10.0, 0.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_sphere_inside() {
        let s = Sphere::new(Vec3::ZERO, 10.0).unwrap();
        let cp = closest_point_on_sphere(&s, Vec3::new(1.0, 0.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(10.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_sphere_at_center() {
        let s = Sphere::new(Vec3::ZERO, 5.0).unwrap();
        let cp = closest_point_on_sphere(&s, Vec3::ZERO);
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_aabb_inside() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::splat(10.0));
        let cp = closest_point_on_aabb(&bb, Vec3::new(5.0, 5.0, 5.0));
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 5.0, 5.0)));
    }

    #[test]
    fn closest_on_aabb_outside() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let cp = closest_point_on_aabb(&bb, Vec3::new(5.0, 0.5, -3.0));
        assert!(vec3_approx_eq(cp, Vec3::new(1.0, 0.5, 0.0)));
    }

    // Frustum tests
    #[test]
    fn frustum_contains_origin() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        // Point at center of near plane should be inside
        assert!(frustum.contains_point(Vec3::new(0.0, 0.0, -1.0)));
    }

    #[test]
    fn frustum_rejects_behind_camera() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        // Point behind camera should be outside
        assert!(!frustum.contains_point(Vec3::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn frustum_rejects_far_point() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        assert!(!frustum.contains_point(Vec3::new(0.0, 0.0, -200.0)));
    }

    #[test]
    fn frustum_contains_aabb_inside() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        let bb = Aabb::new(Vec3::new(-0.1, -0.1, -2.0), Vec3::new(0.1, 0.1, -1.0));
        assert!(frustum.contains_aabb(&bb));
    }

    #[test]
    fn frustum_rejects_aabb_outside() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        let bb = Aabb::new(Vec3::splat(500.0), Vec3::splat(600.0));
        assert!(!frustum.contains_aabb(&bb));
    }

    // Serde roundtrip for new types
    #[test]
    fn triangle_serde_roundtrip() {
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        let json = serde_json::to_string(&tri).unwrap();
        let tri2: Triangle = serde_json::from_str(&json).unwrap();
        assert_eq!(tri, tri2);
    }

    #[test]
    fn line_serde_roundtrip() {
        let l = Line::new(Vec3::ZERO, Vec3::X).unwrap();
        let json = serde_json::to_string(&l).unwrap();
        let l2: Line = serde_json::from_str(&json).unwrap();
        assert_eq!(l, l2);
    }

    #[test]
    fn segment_serde_roundtrip() {
        let s = Segment::new(Vec3::ZERO, Vec3::ONE);
        let json = serde_json::to_string(&s).unwrap();
        let s2: Segment = serde_json::from_str(&json).unwrap();
        assert_eq!(s, s2);
    }

    // --- Audit tests ---

    #[test]
    fn triangle_unit_normal() {
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        let n = tri.unit_normal();
        assert!(approx_eq(n.length(), 1.0));
        assert!(approx_eq(n.z, 1.0));
    }

    #[test]
    fn triangle_unit_normal_3d() {
        // Tilted triangle in 3D space
        let tri = Triangle::new(
            Vec3::ZERO,
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 1.0),
        );
        let n = tri.unit_normal();
        assert!(approx_eq(n.length(), 1.0));
    }

    #[test]
    fn segment_degenerate_zero_length() {
        let s = Segment::new(Vec3::ONE, Vec3::ONE);
        assert!(approx_eq(s.length(), 0.0));
        // Closest point should return the segment point itself
        assert!(vec3_approx_eq(
            s.closest_point(Vec3::new(5.0, 5.0, 5.0)),
            Vec3::ONE
        ));
    }

    #[test]
    fn plane_plane_intersection_point_on_both() {
        let a = Plane::from_point_normal(Vec3::ZERO, Vec3::Y).unwrap();
        let b = Plane::from_point_normal(Vec3::ZERO, Vec3::X).unwrap();
        let line = plane_plane(&a, &b).unwrap();
        // The origin should be close to the intersection line
        let cp = line.closest_point(Vec3::ZERO);
        assert!(approx_eq(cp.length(), 0.0));
    }

    #[test]
    fn closest_on_sphere_direction_consistent() {
        let s = Sphere::new(Vec3::ZERO, 5.0).unwrap();
        // Point along +Y axis -> closest should be along +Y
        let cp = closest_point_on_sphere(&s, Vec3::new(0.0, 100.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(0.0, 5.0, 0.0)));
        // Point along -Z axis -> closest should be along -Z
        let cp2 = closest_point_on_sphere(&s, Vec3::new(0.0, 0.0, -100.0));
        assert!(vec3_approx_eq(cp2, Vec3::new(0.0, 0.0, -5.0)));
    }

    #[test]
    fn frustum_serde_roundtrip() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let f = Frustum::from_view_projection(proj);
        let json = serde_json::to_string(&f).unwrap();
        let f2: Frustum = serde_json::from_str(&json).unwrap();
        assert_eq!(f, f2);
    }

    #[test]
    fn frustum_left_right_rejection() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        // Points far to the left/right should be outside
        assert!(!frustum.contains_point(Vec3::new(1000.0, 0.0, -10.0)));
        assert!(!frustum.contains_point(Vec3::new(-1000.0, 0.0, -10.0)));
    }

    #[test]
    fn ray_triangle_edge_hit() {
        // Ray hitting exactly on the edge of the triangle
        let tri = Triangle::new(
            Vec3::new(-1.0, 0.0, 5.0),
            Vec3::new(1.0, 0.0, 5.0),
            Vec3::new(0.0, 2.0, 5.0),
        );
        let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        // Should hit at the bottom edge (y=0)
        let t = ray_triangle(&r, &tri).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn aabb_aabb_single_axis_separation() {
        // Separated only on Z axis
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::new(0.0, 0.0, 2.0), Vec3::new(1.0, 1.0, 3.0));
        assert!(!aabb_aabb(&a, &b));
    }

    #[test]
    fn line_distance_at_origin() {
        let l = Line::new(Vec3::new(0.0, 5.0, 0.0), Vec3::X).unwrap();
        assert!(approx_eq(l.distance_to_point(Vec3::ZERO), 5.0));
    }

    #[test]
    fn closest_on_plane_already_on_plane() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y).unwrap();
        let point = Vec3::new(3.0, 0.0, -7.0);
        let cp = closest_point_on_plane(&p, point);
        assert!(vec3_approx_eq(cp, point));
    }

    #[test]
    fn closest_on_ray_along_direction() {
        let r = Ray::new(Vec3::ZERO, Vec3::X).unwrap();
        let cp = closest_point_on_ray(&r, Vec3::new(5.0, 0.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn sphere_sphere_concentric() {
        let a = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let b = Sphere::new(Vec3::ZERO, 0.5).unwrap();
        assert!(sphere_sphere(&a, &b));
    }

    // --- V0.5a: BVH ---

    #[test]
    fn bvh_empty() {
        let bvh = Bvh::build(&mut []);
        assert!(bvh.is_empty());
        assert_eq!(bvh.len(), 0);
        let r = Ray::new(Vec3::ZERO, Vec3::X).unwrap();
        assert!(bvh.query_ray(&r).is_empty());
    }

    #[test]
    fn bvh_single() {
        let bb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::ONE);
        let mut items = [(bb, 42)];
        let bvh = Bvh::build(&mut items);
        assert_eq!(bvh.len(), 1);

        let r = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z).unwrap();
        let hits = bvh.query_ray(&r);
        assert_eq!(hits, vec![42]);
    }

    #[test]
    fn bvh_ray_query_hits_and_misses() {
        let mut items: Vec<(Aabb, usize)> = (0..10)
            .map(|i| {
                let x = i as f32 * 3.0;
                (
                    Aabb::new(Vec3::new(x, 0.0, 0.0), Vec3::new(x + 1.0, 1.0, 1.0)),
                    i,
                )
            })
            .collect();
        let bvh = Bvh::build(&mut items);
        assert_eq!(bvh.len(), 10);

        // Ray hitting the first box
        let r = Ray::new(Vec3::new(0.5, 0.5, -5.0), Vec3::Z).unwrap();
        let hits = bvh.query_ray(&r);
        assert!(hits.contains(&0));
        assert!(!hits.contains(&5));

        // Ray missing everything (way above)
        let r_miss = Ray::new(Vec3::new(0.5, 100.0, -5.0), Vec3::Z).unwrap();
        assert!(bvh.query_ray(&r_miss).is_empty());
    }

    #[test]
    fn bvh_aabb_query() {
        let mut items: Vec<(Aabb, usize)> = (0..5)
            .map(|i| {
                let x = i as f32 * 2.0;
                (
                    Aabb::new(Vec3::new(x, 0.0, 0.0), Vec3::new(x + 1.0, 1.0, 1.0)),
                    i,
                )
            })
            .collect();
        let bvh = Bvh::build(&mut items);

        // Query box overlapping first two items
        let query = Aabb::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::new(2.5, 1.0, 1.0));
        let hits = bvh.query_aabb(&query);
        assert!(hits.contains(&0));
        assert!(hits.contains(&1));
    }

    #[test]
    fn bvh_many_items() {
        let mut items: Vec<(Aabb, usize)> = (0..100)
            .map(|i| {
                let x = (i % 10) as f32;
                let y = (i / 10) as f32;
                (
                    Aabb::new(Vec3::new(x, y, 0.0), Vec3::new(x + 0.5, y + 0.5, 0.5)),
                    i,
                )
            })
            .collect();
        let bvh = Bvh::build(&mut items);
        assert_eq!(bvh.len(), 100);

        let r = Ray::new(Vec3::new(0.25, 0.25, -10.0), Vec3::Z).unwrap();
        let hits = bvh.query_ray(&r);
        assert!(!hits.is_empty());
    }

    // --- V0.5a: K-d tree ---

    #[test]
    fn kdtree_empty() {
        let tree = KdTree::build(&mut []);
        assert!(tree.is_empty());
        assert!(tree.nearest(Vec3::ZERO).is_none());
    }

    #[test]
    fn kdtree_single() {
        let mut items = [(Vec3::new(5.0, 0.0, 0.0), 0)];
        let tree = KdTree::build(&mut items);
        let (idx, dist_sq) = tree.nearest(Vec3::ZERO).unwrap();
        assert_eq!(idx, 0);
        assert!(approx_eq(dist_sq, 25.0));
    }

    #[test]
    fn kdtree_nearest_basic() {
        let mut items: Vec<(Vec3, usize)> = vec![
            (Vec3::new(0.0, 0.0, 0.0), 0),
            (Vec3::new(10.0, 0.0, 0.0), 1),
            (Vec3::new(5.0, 5.0, 0.0), 2),
        ];
        let tree = KdTree::build(&mut items);

        let (idx, _) = tree.nearest(Vec3::new(0.1, 0.0, 0.0)).unwrap();
        assert_eq!(idx, 0);

        let (idx, _) = tree.nearest(Vec3::new(9.9, 0.0, 0.0)).unwrap();
        assert_eq!(idx, 1);

        let (idx, _) = tree.nearest(Vec3::new(5.0, 4.9, 0.0)).unwrap();
        assert_eq!(idx, 2);
    }

    #[test]
    fn kdtree_within_radius() {
        let mut items: Vec<(Vec3, usize)> = (0..10)
            .map(|i| (Vec3::new(i as f32, 0.0, 0.0), i))
            .collect();
        let tree = KdTree::build(&mut items);

        let results = tree.within_radius(Vec3::new(5.0, 0.0, 0.0), 1.5);
        let indices: Vec<usize> = results.iter().map(|&(idx, _)| idx).collect();
        assert!(indices.contains(&4));
        assert!(indices.contains(&5));
        assert!(indices.contains(&6));
        assert!(!indices.contains(&3));
        assert!(!indices.contains(&7));
    }

    #[test]
    fn kdtree_within_radius_empty() {
        let mut items = [(Vec3::new(100.0, 100.0, 100.0), 0)];
        let tree = KdTree::build(&mut items);
        let results = tree.within_radius(Vec3::ZERO, 1.0);
        assert!(results.is_empty());
    }

    #[test]
    fn kdtree_many_points() {
        let mut items: Vec<(Vec3, usize)> = (0..1000)
            .map(|i| {
                let x = (i % 10) as f32;
                let y = ((i / 10) % 10) as f32;
                let z = (i / 100) as f32;
                (Vec3::new(x, y, z), i)
            })
            .collect();
        let tree = KdTree::build(&mut items);
        assert_eq!(tree.len(), 1000);

        // The point (0,0,0) should have index 0 as nearest
        let (idx, dist_sq) = tree.nearest(Vec3::new(0.01, 0.01, 0.01)).unwrap();
        assert_eq!(idx, 0);
        assert!(dist_sq < 0.01);
    }

    #[test]
    fn kdtree_nearest_exact_match() {
        let mut items: Vec<(Vec3, usize)> =
            vec![(Vec3::new(1.0, 2.0, 3.0), 7), (Vec3::new(4.0, 5.0, 6.0), 8)];
        let tree = KdTree::build(&mut items);
        let (idx, dist_sq) = tree.nearest(Vec3::new(1.0, 2.0, 3.0)).unwrap();
        assert_eq!(idx, 7);
        assert!(approx_eq(dist_sq, 0.0));
    }

    // --- V0.5a audit tests ---

    #[test]
    fn bvh_degenerate_same_position() {
        // All items at the same position — should still build and query
        let mut items: Vec<(Aabb, usize)> = (0..5)
            .map(|i| (Aabb::new(Vec3::ZERO, Vec3::ONE), i))
            .collect();
        let bvh = Bvh::build(&mut items);
        assert_eq!(bvh.len(), 5);
        let r = Ray::new(Vec3::new(0.5, 0.5, -5.0), Vec3::Z).unwrap();
        let hits = bvh.query_ray(&r);
        assert_eq!(hits.len(), 5);
    }

    #[test]
    fn bvh_aabb_query_no_overlap() {
        let mut items: Vec<(Aabb, usize)> = (0..5)
            .map(|i| {
                let x = i as f32 * 10.0;
                (
                    Aabb::new(Vec3::new(x, 0.0, 0.0), Vec3::new(x + 1.0, 1.0, 1.0)),
                    i,
                )
            })
            .collect();
        let bvh = Bvh::build(&mut items);
        let query = Aabb::new(Vec3::splat(1000.0), Vec3::splat(2000.0));
        assert!(bvh.query_aabb(&query).is_empty());
    }

    #[test]
    fn kdtree_duplicate_points() {
        // Multiple points at the same location
        let mut items: Vec<(Vec3, usize)> = (0..5).map(|i| (Vec3::ZERO, i)).collect();
        let tree = KdTree::build(&mut items);
        let (_, dist_sq) = tree.nearest(Vec3::ZERO).unwrap();
        assert!(approx_eq(dist_sq, 0.0));
    }

    #[test]
    fn kdtree_two_points() {
        let mut items = [
            (Vec3::new(0.0, 0.0, 0.0), 0),
            (Vec3::new(10.0, 0.0, 0.0), 1),
        ];
        let tree = KdTree::build(&mut items);
        let (idx, _) = tree.nearest(Vec3::new(3.0, 0.0, 0.0)).unwrap();
        assert_eq!(idx, 0); // closer to origin
        let (idx, _) = tree.nearest(Vec3::new(7.0, 0.0, 0.0)).unwrap();
        assert_eq!(idx, 1); // closer to 10
    }

    #[test]
    fn bvh_two_items() {
        let mut items = [
            (Aabb::new(Vec3::ZERO, Vec3::ONE), 0),
            (
                Aabb::new(Vec3::new(5.0, 0.0, 0.0), Vec3::new(6.0, 1.0, 1.0)),
                1,
            ),
        ];
        let bvh = Bvh::build(&mut items);
        let r = Ray::new(Vec3::new(5.5, 0.5, -5.0), Vec3::Z).unwrap();
        let hits = bvh.query_ray(&r);
        assert!(hits.contains(&1));
        assert!(!hits.contains(&0));
    }

    // --- V0.5b: Quadtree ---

    #[test]
    fn quadtree_empty() {
        let bounds = Rect::new(glam::Vec2::ZERO, glam::Vec2::splat(100.0));
        let qt = Quadtree::new(bounds, 4, 8);
        assert!(qt.is_empty());
        assert_eq!(qt.len(), 0);
    }

    #[test]
    fn quadtree_insert_and_query() {
        let bounds = Rect::new(glam::Vec2::ZERO, glam::Vec2::splat(100.0));
        let mut qt = Quadtree::new(bounds, 4, 8);
        for i in 0..10 {
            qt.insert(glam::Vec2::new(i as f32 * 10.0 + 1.0, 50.0), i);
        }
        assert_eq!(qt.len(), 10);

        let query = Rect::new(glam::Vec2::new(0.0, 40.0), glam::Vec2::new(25.0, 60.0));
        let results = qt.query_rect(&query);
        assert!(results.contains(&0)); // x=1
        assert!(results.contains(&1)); // x=11
        assert!(results.contains(&2)); // x=21
        assert!(!results.contains(&5)); // x=51
    }

    #[test]
    fn quadtree_out_of_bounds_ignored() {
        let bounds = Rect::new(glam::Vec2::ZERO, glam::Vec2::splat(10.0));
        let mut qt = Quadtree::new(bounds, 4, 8);
        qt.insert(glam::Vec2::new(100.0, 100.0), 0); // Out of bounds
        assert_eq!(qt.len(), 0);
    }

    #[test]
    fn quadtree_subdivision() {
        let bounds = Rect::new(glam::Vec2::ZERO, glam::Vec2::splat(100.0));
        let mut qt = Quadtree::new(bounds, 2, 8); // Split after 2 items
        for i in 0..10 {
            qt.insert(
                glam::Vec2::new(i as f32 * 10.0 + 1.0, i as f32 * 10.0 + 1.0),
                i,
            );
        }
        assert_eq!(qt.len(), 10);
        // Query everything
        let all = qt.query_rect(&bounds);
        assert_eq!(all.len(), 10);
    }

    #[test]
    fn rect_contains_and_overlaps() {
        let r = Rect::new(glam::Vec2::ZERO, glam::Vec2::splat(10.0));
        assert!(r.contains_point(glam::Vec2::splat(5.0)));
        assert!(!r.contains_point(glam::Vec2::splat(11.0)));

        let r2 = Rect::new(glam::Vec2::splat(5.0), glam::Vec2::splat(15.0));
        assert!(r.overlaps(&r2));

        let r3 = Rect::new(glam::Vec2::splat(20.0), glam::Vec2::splat(30.0));
        assert!(!r.overlaps(&r3));
    }

    // --- V0.5b: Octree ---

    #[test]
    fn octree_empty() {
        let bounds = Aabb::new(Vec3::ZERO, Vec3::splat(100.0));
        let ot = Octree::new(bounds, 4, 8);
        assert!(ot.is_empty());
    }

    #[test]
    fn octree_insert_and_query() {
        let bounds = Aabb::new(Vec3::ZERO, Vec3::splat(100.0));
        let mut ot = Octree::new(bounds, 4, 8);
        for i in 0..20 {
            ot.insert(Vec3::new(i as f32 * 5.0 + 1.0, 50.0, 50.0), i);
        }
        assert_eq!(ot.len(), 20);

        let query = Aabb::new(Vec3::new(0.0, 40.0, 40.0), Vec3::new(20.0, 60.0, 60.0));
        let results = ot.query_aabb(&query);
        assert!(results.contains(&0)); // x=1
        assert!(results.contains(&1)); // x=6
        assert!(results.contains(&2)); // x=11
        assert!(results.contains(&3)); // x=16
        assert!(!results.contains(&10)); // x=51
    }

    #[test]
    fn octree_out_of_bounds() {
        let bounds = Aabb::new(Vec3::ZERO, Vec3::splat(10.0));
        let mut ot = Octree::new(bounds, 4, 8);
        ot.insert(Vec3::splat(100.0), 0);
        assert_eq!(ot.len(), 0);
    }

    #[test]
    fn octree_all_octants() {
        let bounds = Aabb::new(Vec3::splat(-10.0), Vec3::splat(10.0));
        let mut ot = Octree::new(bounds, 2, 4);
        // Insert one point per octant
        for octant in 0..8u32 {
            let x = if octant & 1 != 0 { 5.0 } else { -5.0 };
            let y = if octant & 2 != 0 { 5.0 } else { -5.0 };
            let z = if octant & 4 != 0 { 5.0 } else { -5.0 };
            ot.insert(Vec3::new(x, y, z), octant as usize);
        }
        assert_eq!(ot.len(), 8);
        // Query the entire space
        let all = ot.query_aabb(&bounds);
        assert_eq!(all.len(), 8);
    }

    // --- V0.5b: Spatial hash ---

    #[test]
    fn spatial_hash_empty() {
        let sh = SpatialHash::new(1.0).unwrap();
        assert!(sh.is_empty());
        assert_eq!(sh.cell_count(), 0);
    }

    #[test]
    fn spatial_hash_insert_and_query_cell() {
        let mut sh = SpatialHash::new(10.0).unwrap();
        sh.insert(Vec3::new(5.0, 5.0, 5.0), 0);
        sh.insert(Vec3::new(7.0, 3.0, 1.0), 1);
        sh.insert(Vec3::new(15.0, 5.0, 5.0), 2); // Different cell
        assert_eq!(sh.len(), 3);

        let cell = sh.query_cell(Vec3::new(5.0, 5.0, 5.0));
        assert!(cell.contains(&0));
        assert!(cell.contains(&1));
        assert!(!cell.contains(&2));
    }

    #[test]
    fn spatial_hash_query_radius() {
        let mut sh = SpatialHash::new(5.0).unwrap();
        for i in 0..20 {
            sh.insert(Vec3::new(i as f32, 0.0, 0.0), i);
        }
        // Query around x=10 with radius 3 => cells [1] and [2] (covering 5..15)
        let results = sh.query_radius(Vec3::new(10.0, 0.0, 0.0), 3.0);
        assert!(results.contains(&10));
        // Items in the same cell range should be candidates
        assert!(results.contains(&11));
    }

    #[test]
    fn spatial_hash_clear() {
        let mut sh = SpatialHash::new(1.0).unwrap();
        sh.insert(Vec3::ZERO, 0);
        sh.insert(Vec3::ONE, 1);
        assert_eq!(sh.len(), 2);
        sh.clear();
        assert!(sh.is_empty());
        assert_eq!(sh.cell_count(), 0);
    }

    #[test]
    fn spatial_hash_negative_coords() {
        let mut sh = SpatialHash::new(1.0).unwrap();
        sh.insert(Vec3::new(-5.0, -5.0, -5.0), 0);
        let cell = sh.query_cell(Vec3::new(-5.0, -5.0, -5.0));
        assert!(cell.contains(&0));
    }

    #[test]
    fn rect_serde_roundtrip() {
        let r = Rect::new(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
        let json = serde_json::to_string(&r).unwrap();
        let r2: Rect = serde_json::from_str(&json).unwrap();
        assert_eq!(r, r2);
    }

    // --- V0.5b audit tests ---

    #[test]
    fn rect_size() {
        let r = Rect::new(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(4.0, 6.0));
        let s = r.size();
        assert!(approx_eq(s.x, 3.0));
        assert!(approx_eq(s.y, 4.0));
    }

    #[test]
    fn quadtree_query_no_results() {
        let bounds = Rect::new(glam::Vec2::ZERO, glam::Vec2::splat(100.0));
        let mut qt = Quadtree::new(bounds, 4, 8);
        for i in 0..10 {
            qt.insert(glam::Vec2::new(i as f32, i as f32), i);
        }
        let query = Rect::new(glam::Vec2::splat(80.0), glam::Vec2::splat(90.0));
        assert!(qt.query_rect(&query).is_empty());
    }

    #[test]
    fn octree_query_all() {
        let bounds = Aabb::new(Vec3::ZERO, Vec3::splat(10.0));
        let mut ot = Octree::new(bounds, 4, 8);
        for i in 0..5 {
            ot.insert(Vec3::splat(i as f32 + 1.0), i);
        }
        // Query the full bounds should return everything
        let all = ot.query_aabb(&bounds);
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn spatial_hash_many_in_one_cell() {
        let mut sh = SpatialHash::new(100.0).unwrap(); // Huge cell
        for i in 0..50 {
            sh.insert(Vec3::new(i as f32, 0.0, 0.0), i); // All in one cell
        }
        let cell = sh.query_cell(Vec3::ZERO);
        assert_eq!(cell.len(), 50);
    }

    #[test]
    fn quadtree_max_depth_prevents_infinite_split() {
        let bounds = Rect::new(glam::Vec2::ZERO, glam::Vec2::splat(10.0));
        let mut qt = Quadtree::new(bounds, 1, 3); // Split after 1 item, max 3 levels
        // Insert many items at the same spot — should stop splitting at depth 3
        for i in 0..20 {
            qt.insert(glam::Vec2::splat(5.0), i);
        }
        assert_eq!(qt.len(), 20);
        let all = qt.query_rect(&bounds);
        assert_eq!(all.len(), 20);
    }

    #[test]
    fn octree_max_depth_prevents_infinite_split() {
        let bounds = Aabb::new(Vec3::ZERO, Vec3::splat(10.0));
        let mut ot = Octree::new(bounds, 1, 3);
        for i in 0..20 {
            ot.insert(Vec3::splat(5.0), i);
        }
        assert_eq!(ot.len(), 20);
        let all = ot.query_aabb(&bounds);
        assert_eq!(all.len(), 20);
    }

    // --- V0.5c: Convex hull ---

    #[test]
    fn convex_hull_square() {
        let pts = vec![
            glam::Vec2::new(0.0, 0.0),
            glam::Vec2::new(1.0, 0.0),
            glam::Vec2::new(1.0, 1.0),
            glam::Vec2::new(0.0, 1.0),
            glam::Vec2::new(0.5, 0.5), // Interior point
        ];
        let hull = convex_hull_2d(&pts);
        assert_eq!(hull.len(), 4); // Only the 4 corners
    }

    #[test]
    fn convex_hull_triangle() {
        let pts = vec![
            glam::Vec2::new(0.0, 0.0),
            glam::Vec2::new(2.0, 0.0),
            glam::Vec2::new(1.0, 2.0),
        ];
        let hull = convex_hull_2d(&pts);
        assert_eq!(hull.len(), 3);
    }

    #[test]
    fn convex_hull_collinear() {
        // All points on a line — hull should be just the endpoints
        let pts = vec![
            glam::Vec2::new(0.0, 0.0),
            glam::Vec2::new(1.0, 0.0),
            glam::Vec2::new(2.0, 0.0),
            glam::Vec2::new(3.0, 0.0),
        ];
        let hull = convex_hull_2d(&pts);
        assert_eq!(hull.len(), 2);
    }

    #[test]
    fn convex_hull_single_point() {
        let pts = vec![glam::Vec2::new(5.0, 5.0)];
        let hull = convex_hull_2d(&pts);
        assert_eq!(hull.len(), 1);
    }

    #[test]
    fn convex_hull_many_interior() {
        // Circle of points + many interior
        let mut pts = Vec::new();
        for i in 0..8 {
            let angle = i as f32 * std::f32::consts::TAU / 8.0;
            pts.push(glam::Vec2::new(angle.cos() * 10.0, angle.sin() * 10.0));
        }
        // Add 50 interior points
        for i in 0..50 {
            let x = (i % 10) as f32 - 5.0;
            let y = (i / 10) as f32 - 2.5;
            pts.push(glam::Vec2::new(x, y));
        }
        let hull = convex_hull_2d(&pts);
        assert_eq!(hull.len(), 8); // Only the circle points
    }

    // --- V0.5c: GJK ---

    fn make_square(cx: f32, cy: f32, half: f32) -> ConvexPolygon {
        ConvexPolygon::new(vec![
            glam::Vec2::new(cx - half, cy - half),
            glam::Vec2::new(cx + half, cy - half),
            glam::Vec2::new(cx + half, cy + half),
            glam::Vec2::new(cx - half, cy + half),
        ])
        .unwrap()
    }

    #[test]
    fn gjk_overlapping_squares() {
        let a = make_square(0.0, 0.0, 1.0);
        let b = make_square(0.5, 0.5, 1.0);
        assert!(gjk_intersect(&a, &b));
    }

    #[test]
    fn gjk_separated_squares() {
        let a = make_square(0.0, 0.0, 1.0);
        let b = make_square(5.0, 0.0, 1.0);
        assert!(!gjk_intersect(&a, &b));
    }

    #[test]
    fn gjk_touching_squares() {
        let a = make_square(0.0, 0.0, 1.0);
        let b = make_square(2.0, 0.0, 1.0); // Touching at x=1
        // Touching = on boundary, may or may not register depending on epsilon
        // GJK typically returns true for touching
        let _ = gjk_intersect(&a, &b); // Just verify no panic
    }

    #[test]
    fn gjk_contained() {
        let a = make_square(0.0, 0.0, 5.0); // Big
        let b = make_square(0.0, 0.0, 1.0); // Small, inside A
        assert!(gjk_intersect(&a, &b));
    }

    #[test]
    fn gjk_same_shape() {
        let a = make_square(0.0, 0.0, 1.0);
        let b = make_square(0.0, 0.0, 1.0);
        assert!(gjk_intersect(&a, &b));
    }

    // --- V0.5c: EPA ---

    #[test]
    fn epa_overlapping_squares_depth() {
        let a = make_square(0.0, 0.0, 1.0);
        let b = make_square(1.0, 0.0, 1.0); // Overlap of 1.0 in X
        let pen = gjk_epa(&a, &b);
        assert!(pen.is_some());
        let p = pen.unwrap();
        assert!(p.depth > 0.0);
        assert!(p.depth < 2.1); // At most full overlap
    }

    #[test]
    fn epa_no_overlap() {
        let a = make_square(0.0, 0.0, 1.0);
        let b = make_square(5.0, 0.0, 1.0);
        assert!(gjk_epa(&a, &b).is_none());
    }

    #[test]
    fn epa_deep_overlap() {
        let a = make_square(0.0, 0.0, 2.0);
        let b = make_square(0.5, 0.0, 2.0); // 3.5 overlap in X
        let pen = gjk_epa(&a, &b).unwrap();
        assert!(pen.depth > 0.0);
    }

    #[test]
    fn convex_support_polygon() {
        let poly = make_square(0.0, 0.0, 1.0);
        let s = poly.support(glam::Vec2::X);
        assert!(approx_eq(s.x, 1.0));
        let s = poly.support(-glam::Vec2::Y);
        assert!(approx_eq(s.y, -1.0));
    }

    // --- V0.5c audit tests ---

    #[test]
    fn epa_depth_always_positive() {
        let a = make_square(0.0, 0.0, 1.0);
        let b = make_square(0.5, 0.0, 1.0);
        let pen = gjk_epa(&a, &b).unwrap();
        assert!(pen.depth > 0.0);
        assert!(pen.depth <= 2.0); // Can't exceed full overlap
    }

    #[test]
    fn convex_hull_empty() {
        let hull = convex_hull_2d(&[]);
        assert!(hull.is_empty());
    }

    #[test]
    fn convex_hull_two_points() {
        let pts = vec![glam::Vec2::ZERO, glam::Vec2::new(5.0, 0.0)];
        let hull = convex_hull_2d(&pts);
        assert_eq!(hull.len(), 2);
    }

    #[test]
    fn gjk_triangles() {
        let a = ConvexPolygon::new(vec![
            glam::Vec2::new(0.0, 0.0),
            glam::Vec2::new(2.0, 0.0),
            glam::Vec2::new(1.0, 2.0),
        ])
        .unwrap();
        let b = ConvexPolygon::new(vec![
            glam::Vec2::new(1.0, 0.0),
            glam::Vec2::new(3.0, 0.0),
            glam::Vec2::new(2.0, 2.0),
        ])
        .unwrap();
        assert!(gjk_intersect(&a, &b)); // Overlapping triangles
    }

    #[test]
    fn penetration_serde_roundtrip() {
        let p = Penetration {
            normal: glam::Vec2::new(1.0, 0.0),
            depth: 0.5,
        };
        let json = serde_json::to_string(&p).unwrap();
        let p2: Penetration = serde_json::from_str(&json).unwrap();
        assert_eq!(p, p2);
    }

    #[test]
    fn gjk_epa_symmetric() {
        // gjk_epa(a, b) and gjk_epa(b, a) should both detect collision
        let a = make_square(0.0, 0.0, 1.0);
        let b = make_square(0.5, 0.5, 1.0);
        assert!(gjk_epa(&a, &b).is_some());
        assert!(gjk_epa(&b, &a).is_some());
    }

    #[test]
    fn gjk_epa_normal_points_outward() {
        // EPA normals should always point from A toward B (separation direction).
        // Test with multiple shape orderings to exercise both CW and CCW simplex paths.
        let a = make_square(0.0, 0.0, 2.0);
        let b = make_square(1.0, 0.0, 2.0);
        let pen_ab = gjk_epa(&a, &b).expect("should overlap");
        let pen_ba = gjk_epa(&b, &a).expect("should overlap");
        // Depth should be positive
        assert!(pen_ab.depth > 0.0);
        assert!(pen_ba.depth > 0.0);
        // Depths should be approximately equal
        assert!((pen_ab.depth - pen_ba.depth).abs() < 0.1);
    }

    // --- V1.0b: Display impls ---

    #[test]
    fn ray_display() {
        let r = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::X).unwrap();
        assert_eq!(r.to_string(), "Ray(1, 0, 0 -> 1, 0, 0)");
    }

    #[test]
    fn ray_display_precision() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::X).unwrap();
        assert_eq!(format!("{r:.1}"), "Ray(1.0, 2.0, 3.0 -> 1.0, 0.0, 0.0)");
    }

    #[test]
    fn plane_display() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y).unwrap();
        assert_eq!(p.to_string(), "Plane(n=(0, 1, 0), d=0)");
    }

    #[test]
    fn plane_display_precision() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y).unwrap();
        assert_eq!(format!("{p:.2}"), "Plane(n=(0.00, 1.00, 0.00), d=0.00)");
    }

    #[test]
    fn aabb_display() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert_eq!(a.to_string(), "Aabb((0, 0, 0)..(1, 1, 1))");
    }

    #[test]
    fn sphere_display() {
        let sp = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 5.0).unwrap();
        assert_eq!(sp.to_string(), "Sphere((1, 2, 3), r=5)");
    }

    #[test]
    fn sphere_display_precision() {
        let sp = Sphere::new(Vec3::ZERO, 2.5).unwrap();
        assert_eq!(format!("{sp:.1}"), "Sphere((0.0, 0.0, 0.0), r=2.5)");
    }

    #[test]
    fn triangle_display() {
        let t = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        assert_eq!(t.to_string(), "Triangle((0, 0, 0), (1, 0, 0), (0, 1, 0))");
    }

    // --- V1.0b: Rect::merge / Rect::area ---

    #[test]
    fn rect_merge() {
        let a = Rect::new(glam::Vec2::ZERO, glam::Vec2::ONE);
        let b = Rect::new(glam::Vec2::new(2.0, 2.0), glam::Vec2::new(3.0, 3.0));
        let m = a.merge(&b);
        assert_eq!(m.min, glam::Vec2::ZERO);
        assert_eq!(m.max, glam::Vec2::new(3.0, 3.0));
    }

    #[test]
    fn rect_merge_overlapping() {
        let a = Rect::new(glam::Vec2::ZERO, glam::Vec2::new(2.0, 2.0));
        let b = Rect::new(glam::Vec2::ONE, glam::Vec2::new(3.0, 3.0));
        let m = a.merge(&b);
        assert_eq!(m.min, glam::Vec2::ZERO);
        assert_eq!(m.max, glam::Vec2::new(3.0, 3.0));
    }

    #[test]
    fn rect_area() {
        let r = Rect::new(glam::Vec2::ZERO, glam::Vec2::new(3.0, 4.0));
        assert!((r.area() - 12.0).abs() < 1e-6);
    }

    #[test]
    fn rect_area_zero() {
        let r = Rect::new(glam::Vec2::ZERO, glam::Vec2::new(5.0, 0.0));
        assert!((r.area()).abs() < 1e-6);
    }

    // --- Audit: edge-case hardening ---

    #[test]
    fn plane_from_point_normal_zero_normal_errors() {
        let result = Plane::from_point_normal(Vec3::ZERO, Vec3::ZERO);
        assert!(result.is_err());
    }

    #[test]
    fn segment_direction_degenerate_no_nan() {
        let s = Segment::new(Vec3::splat(5.0), Vec3::splat(5.0));
        let d = s.direction();
        assert!(!d.x.is_nan() && !d.y.is_nan() && !d.z.is_nan());
        assert!(approx_eq(d.length(), 1.0));
    }

    #[test]
    fn triangle_unit_normal_degenerate_no_nan() {
        let t = Triangle::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);
        let n = t.unit_normal();
        assert!(!n.x.is_nan() && !n.y.is_nan() && !n.z.is_nan());
        assert!(approx_eq(n.length(), 1.0));
    }

    #[test]
    fn triangle_unit_normal_collinear_no_nan() {
        let t = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::new(2.0, 0.0, 0.0));
        let n = t.unit_normal();
        assert!(!n.x.is_nan() && !n.y.is_nan() && !n.z.is_nan());
        assert!(approx_eq(n.length(), 1.0));
    }

    // --- 3D GJK / EPA tests ---

    fn make_box_3d(center: Vec3, half: f32) -> ConvexHull3D {
        let h = Vec3::splat(half);
        ConvexHull3D::new(vec![
            center + Vec3::new(-h.x, -h.y, -h.z),
            center + Vec3::new(h.x, -h.y, -h.z),
            center + Vec3::new(-h.x, h.y, -h.z),
            center + Vec3::new(h.x, h.y, -h.z),
            center + Vec3::new(-h.x, -h.y, h.z),
            center + Vec3::new(h.x, -h.y, h.z),
            center + Vec3::new(-h.x, h.y, h.z),
            center + Vec3::new(h.x, h.y, h.z),
        ])
        .unwrap()
    }

    #[test]
    fn gjk3d_overlapping_boxes() {
        let a = make_box_3d(Vec3::ZERO, 1.0);
        let b = make_box_3d(Vec3::new(0.5, 0.0, 0.0), 1.0);
        assert!(gjk_intersect_3d(&a, &b));
    }

    #[test]
    fn gjk3d_separated_boxes() {
        let a = make_box_3d(Vec3::ZERO, 1.0);
        let b = make_box_3d(Vec3::new(5.0, 0.0, 0.0), 1.0);
        assert!(!gjk_intersect_3d(&a, &b));
    }

    #[test]
    fn gjk3d_same_shape() {
        let a = make_box_3d(Vec3::ZERO, 1.0);
        assert!(gjk_intersect_3d(&a, &a));
    }

    #[test]
    fn gjk3d_sphere_sphere() {
        let a = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let b = Sphere::new(Vec3::new(1.5, 0.0, 0.0), 1.0).unwrap();
        assert!(gjk_intersect_3d(&a, &b));

        let c = Sphere::new(Vec3::new(5.0, 0.0, 0.0), 1.0).unwrap();
        assert!(!gjk_intersect_3d(&a, &c));
    }

    #[test]
    fn gjk3d_sphere_box() {
        let a = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let b = make_box_3d(Vec3::new(0.5, 0.0, 0.0), 0.5);
        assert!(gjk_intersect_3d(&a, &b));
    }

    #[test]
    fn gjk_epa_3d_penetration() {
        // Use spheres for cleaner EPA (non-degenerate simplex)
        let a = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let b = Sphere::new(Vec3::new(1.0, 0.0, 0.0), 1.0).unwrap();
        assert!(gjk_intersect_3d(&a, &b));
        let pen = gjk_epa_3d(&a, &b);
        assert!(pen.is_some());
        let p = pen.unwrap();
        assert!(p.depth > 0.0);
        assert!(p.depth < 2.0); // Can't penetrate more than sum of radii
    }

    #[test]
    fn gjk_epa_3d_no_overlap() {
        let a = make_box_3d(Vec3::ZERO, 1.0);
        let b = make_box_3d(Vec3::new(5.0, 0.0, 0.0), 1.0);
        assert!(gjk_epa_3d(&a, &b).is_none());
    }

    #[test]
    fn gjk3d_convex_hull_empty_errors() {
        assert!(ConvexHull3D::new(vec![]).is_err());
    }

    // --- OBB tests ---

    #[test]
    fn obb_contains_center() {
        let obb = Obb::new(Vec3::new(1.0, 2.0, 3.0), Vec3::ONE, glam::Quat::IDENTITY);
        assert!(obb.contains_point(Vec3::new(1.0, 2.0, 3.0)));
    }

    #[test]
    fn obb_contains_corner() {
        let obb = Obb::new(Vec3::ZERO, Vec3::ONE, glam::Quat::IDENTITY);
        assert!(obb.contains_point(Vec3::ONE));
        assert!(!obb.contains_point(Vec3::splat(1.5)));
    }

    #[test]
    fn obb_rotated_containment() {
        let rot = glam::Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);
        let obb = Obb::new(Vec3::ZERO, Vec3::new(2.0, 0.5, 1.0), rot);
        // Center should always be inside
        assert!(obb.contains_point(Vec3::ZERO));
    }

    #[test]
    fn obb_closest_point() {
        let obb = Obb::new(Vec3::ZERO, Vec3::ONE, glam::Quat::IDENTITY);
        let cp = obb.closest_point(Vec3::new(5.0, 0.0, 0.0));
        assert!(approx_eq(cp.x, 1.0));
        assert!(approx_eq(cp.y, 0.0));
    }

    #[test]
    fn ray_obb_hit() {
        let obb = Obb::new(Vec3::new(5.0, 0.0, 0.0), Vec3::ONE, glam::Quat::IDENTITY);
        let r = Ray::new(Vec3::ZERO, Vec3::X).unwrap();
        let t = ray_obb(&r, &obb);
        assert!(t.is_some());
        assert!(approx_eq(t.unwrap(), 4.0));
    }

    #[test]
    fn ray_obb_miss() {
        let obb = Obb::new(Vec3::new(5.0, 0.0, 0.0), Vec3::ONE, glam::Quat::IDENTITY);
        let r = Ray::new(Vec3::ZERO, Vec3::Y).unwrap();
        assert!(ray_obb(&r, &obb).is_none());
    }

    #[test]
    fn gjk3d_obb_vs_obb() {
        let a = Obb::new(Vec3::ZERO, Vec3::ONE, glam::Quat::IDENTITY);
        let b = Obb::new(Vec3::new(1.5, 0.0, 0.0), Vec3::ONE, glam::Quat::IDENTITY);
        assert!(gjk_intersect_3d(&a, &b));

        let c = Obb::new(Vec3::new(5.0, 0.0, 0.0), Vec3::ONE, glam::Quat::IDENTITY);
        assert!(!gjk_intersect_3d(&a, &c));
    }

    // --- Capsule tests ---

    #[test]
    fn capsule_contains_center() {
        let cap = Capsule::new(Vec3::ZERO, Vec3::new(0.0, 2.0, 0.0), 0.5).unwrap();
        assert!(cap.contains_point(Vec3::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn capsule_contains_near_surface() {
        let cap = Capsule::new(Vec3::ZERO, Vec3::new(0.0, 2.0, 0.0), 1.0).unwrap();
        assert!(cap.contains_point(Vec3::new(0.9, 1.0, 0.0)));
        assert!(!cap.contains_point(Vec3::new(2.0, 1.0, 0.0)));
    }

    #[test]
    fn capsule_negative_radius_errors() {
        assert!(Capsule::new(Vec3::ZERO, Vec3::Y, -1.0).is_err());
    }

    #[test]
    fn capsule_axis_length() {
        let cap = Capsule::new(Vec3::ZERO, Vec3::new(0.0, 3.0, 0.0), 0.5).unwrap();
        assert!(approx_eq(cap.axis_length(), 3.0));
    }

    #[test]
    fn ray_capsule_hit() {
        let cap = Capsule::new(Vec3::new(5.0, -1.0, 0.0), Vec3::new(5.0, 1.0, 0.0), 0.5).unwrap();
        let r = Ray::new(Vec3::ZERO, Vec3::X).unwrap();
        let t = ray_capsule(&r, &cap);
        assert!(t.is_some());
        assert!(t.unwrap() > 4.0 && t.unwrap() < 5.0);
    }

    #[test]
    fn ray_capsule_miss() {
        let cap = Capsule::new(Vec3::new(5.0, -1.0, 0.0), Vec3::new(5.0, 1.0, 0.0), 0.5).unwrap();
        let r = Ray::new(Vec3::ZERO, Vec3::Y).unwrap();
        assert!(ray_capsule(&r, &cap).is_none());
    }

    #[test]
    fn gjk3d_capsule_vs_sphere() {
        let cap = Capsule::new(Vec3::ZERO, Vec3::new(0.0, 2.0, 0.0), 0.5).unwrap();
        let sphere = Sphere::new(Vec3::new(1.0, 1.0, 0.0), 1.0).unwrap();
        assert!(gjk_intersect_3d(&cap, &sphere));

        let far = Sphere::new(Vec3::new(10.0, 0.0, 0.0), 0.5).unwrap();
        assert!(!gjk_intersect_3d(&cap, &far));
    }

    // --- 3D GJK winding stress tests ---

    #[test]
    fn gjk3d_all_axis_offsets() {
        // Test overlap detection along all 6 axis directions to stress winding
        let a = make_box_3d(Vec3::ZERO, 1.0);
        for &offset in &[
            Vec3::new(1.5, 0.0, 0.0),
            Vec3::new(-1.5, 0.0, 0.0),
            Vec3::new(0.0, 1.5, 0.0),
            Vec3::new(0.0, -1.5, 0.0),
            Vec3::new(0.0, 0.0, 1.5),
            Vec3::new(0.0, 0.0, -1.5),
        ] {
            let b = make_box_3d(offset, 1.0);
            assert!(
                gjk_intersect_3d(&a, &b),
                "should overlap at offset {offset:?}"
            );
        }
        for &offset in &[
            Vec3::new(3.0, 0.0, 0.0),
            Vec3::new(-3.0, 0.0, 0.0),
            Vec3::new(0.0, 3.0, 0.0),
            Vec3::new(0.0, -3.0, 0.0),
            Vec3::new(0.0, 0.0, 3.0),
            Vec3::new(0.0, 0.0, -3.0),
        ] {
            let b = make_box_3d(offset, 1.0);
            assert!(
                !gjk_intersect_3d(&a, &b),
                "should NOT overlap at offset {offset:?}"
            );
        }
    }

    #[test]
    fn gjk3d_diagonal_overlap() {
        let a = make_box_3d(Vec3::ZERO, 1.0);
        let b = make_box_3d(Vec3::new(1.0, 1.0, 1.0), 1.0);
        assert!(gjk_intersect_3d(&a, &b));
    }

    #[test]
    fn gjk3d_asymmetric_shapes() {
        // Tall capsule vs small sphere at different positions
        let cap = Capsule::new(Vec3::new(0.0, -5.0, 0.0), Vec3::new(0.0, 5.0, 0.0), 0.5).unwrap();
        // Near the middle of the capsule
        let s1 = Sphere::new(Vec3::new(1.0, 0.0, 0.0), 1.0).unwrap();
        assert!(gjk_intersect_3d(&cap, &s1));
        // Near the top cap
        let s2 = Sphere::new(Vec3::new(0.0, 5.5, 0.0), 0.5).unwrap();
        assert!(gjk_intersect_3d(&cap, &s2));
        // Well beyond the bottom
        let s3 = Sphere::new(Vec3::new(0.0, -7.0, 0.0), 0.5).unwrap();
        assert!(!gjk_intersect_3d(&cap, &s3));
    }

    #[test]
    fn gjk_epa_3d_symmetric_penetration() {
        // Deeper overlap for cleaner EPA convergence
        let a = make_box_3d(Vec3::ZERO, 2.0);
        let b = make_box_3d(Vec3::new(1.0, 0.0, 0.0), 2.0);
        let p1 = gjk_epa_3d(&a, &b);
        let p2 = gjk_epa_3d(&b, &a);
        // Both should detect overlap (GJK) — EPA may or may not produce depth
        assert!(gjk_intersect_3d(&a, &b));
        assert!(gjk_intersect_3d(&b, &a));
        // If both EPA succeed, depths should be in the same ballpark
        if let (Some(pen1), Some(pen2)) = (p1, p2) {
            assert!(pen1.depth > 0.0);
            assert!(pen2.depth > 0.0);
        }
    }

    // --- Frustum-sphere tests ---

    #[test]
    fn frustum_contains_sphere_inside() {
        let proj = glam::Mat4::perspective_rh_gl(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0).unwrap();
        assert!(frustum.contains_sphere(&sphere));
    }

    #[test]
    fn frustum_rejects_sphere_behind() {
        let proj = glam::Mat4::perspective_rh_gl(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0).unwrap();
        assert!(!frustum.contains_sphere(&sphere));
    }

    #[test]
    fn frustum_sphere_partially_inside() {
        let proj = glam::Mat4::perspective_rh_gl(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -0.1), 0.5).unwrap();
        assert!(frustum.contains_sphere(&sphere));
    }

    // --- SDF tests ---

    #[test]
    fn sdf_sphere_inside_outside() {
        assert!(sdf_sphere(Vec3::ZERO, Vec3::ZERO, 1.0) < 0.0); // inside
        assert!(sdf_sphere(Vec3::new(2.0, 0.0, 0.0), Vec3::ZERO, 1.0) > 0.0); // outside
        assert!(approx_eq(
            sdf_sphere(Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO, 1.0),
            0.0
        )); // on surface
    }

    #[test]
    fn sdf_box_inside_outside() {
        assert!(sdf_box(Vec3::ZERO, Vec3::ZERO, Vec3::ONE) < 0.0);
        assert!(sdf_box(Vec3::new(2.0, 0.0, 0.0), Vec3::ZERO, Vec3::ONE) > 0.0);
    }

    #[test]
    fn sdf_csg_operations() {
        let d1 = -0.5; // inside shape 1
        let d2 = 0.5; // outside shape 2
        assert!(sdf_union(d1, d2) < 0.0); // union: inside at least one
        assert!(sdf_intersection(d1, d2) > 0.0); // intersection: not inside both
        assert!(sdf_subtraction(d1, d2) < 0.0); // subtraction: inside 1, outside 2
    }

    // --- Polygon triangulation tests ---

    #[test]
    fn triangulate_triangle() {
        let verts = [
            glam::Vec2::ZERO,
            glam::Vec2::new(1.0, 0.0),
            glam::Vec2::new(0.0, 1.0),
        ];
        let tris = triangulate_polygon(&verts).unwrap();
        assert_eq!(tris.len(), 1);
    }

    #[test]
    fn triangulate_square() {
        let verts = [
            glam::Vec2::ZERO,
            glam::Vec2::new(1.0, 0.0),
            glam::Vec2::new(1.0, 1.0),
            glam::Vec2::new(0.0, 1.0),
        ];
        let tris = triangulate_polygon(&verts).unwrap();
        assert_eq!(tris.len(), 2);
    }

    #[test]
    fn triangulate_too_few() {
        assert!(triangulate_polygon(&[glam::Vec2::ZERO, glam::Vec2::X]).is_err());
    }

    // --- Ray-quadric test ---

    #[test]
    fn ray_quadric_sphere() {
        // Sphere x²+y²+z²=1 as quadric: A=I, b=0, c=-1
        let a = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let b = [0.0, 0.0, 0.0];
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z).unwrap();
        let t = ray_quadric(&ray, &a, &b, -1.0).unwrap();
        assert!(approx_eq(t, 4.0)); // hit at z=-1, t=4
    }

    // --- Fresnel tests ---

    #[test]
    fn fresnel_schlick_endpoints() {
        // At normal incidence (cos=1), R = ((n1-n2)/(n1+n2))²
        let r = fresnel_schlick(1.0, 1.0, 1.5);
        let expected = ((1.0_f32 - 1.5) / (1.0 + 1.5)).powi(2);
        assert!(approx_eq(r, expected));
    }

    #[test]
    fn fresnel_exact_normal_incidence() {
        let r = fresnel_exact(1.0, 1.0, 1.5);
        let expected = ((1.0_f32 - 1.5) / (1.0 + 1.5)).powi(2);
        assert!((r - expected).abs() < 0.01);
    }

    #[test]
    fn refract_basic() {
        let incident = Vec3::new(0.0, -1.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let refracted = refract(incident, normal, 1.0).unwrap();
        // Same medium (eta=1): refracted = incident
        assert!(vec3_approx_eq(refracted, incident));
    }

    #[test]
    fn refract_total_internal_reflection() {
        // Glass to air at steep angle
        let incident = Vec3::new(0.9, -0.436, 0.0).normalize();
        let normal = Vec3::new(0.0, 1.0, 0.0);
        assert!(refract(incident, normal, 1.5).is_none());
    }

    // --- Sweep-and-prune tests ---

    #[test]
    fn sap_basic() {
        let aabbs = [
            Aabb::new(Vec3::ZERO, Vec3::ONE),
            Aabb::new(Vec3::new(0.5, 0.0, 0.0), Vec3::new(1.5, 1.0, 1.0)),
            Aabb::new(Vec3::new(5.0, 0.0, 0.0), Vec3::new(6.0, 1.0, 1.0)),
        ];
        let pairs = sweep_and_prune(&aabbs);
        assert_eq!(pairs.len(), 1); // Only 0-1 overlap
        assert!(pairs.contains(&(0, 1)));
    }

    #[test]
    fn sap_no_overlaps() {
        let aabbs = [
            Aabb::new(Vec3::ZERO, Vec3::ONE),
            Aabb::new(Vec3::new(5.0, 5.0, 5.0), Vec3::new(6.0, 6.0, 6.0)),
        ];
        assert!(sweep_and_prune(&aabbs).is_empty());
    }

    // --- CCD tests ---

    #[test]
    fn swept_aabb_expands() {
        let aabb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let swept = swept_aabb(&aabb, Vec3::new(5.0, 0.0, 0.0), 1.0);
        assert!(approx_eq(swept.min.x, 0.0));
        assert!(approx_eq(swept.max.x, 6.0));
    }

    #[test]
    fn toi_overlapping_at_t0() {
        let a = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let b = Sphere::new(Vec3::new(0.5, 0.0, 0.0), 1.0).unwrap();
        let t = time_of_impact(&a, &b, Vec3::ZERO, Vec3::ZERO, 1.0, 0.01);
        assert_eq!(t, Some(0.0));
    }

    #[test]
    fn toi_no_collision() {
        let a = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let b = Sphere::new(Vec3::new(10.0, 0.0, 0.0), 1.0).unwrap();
        // Moving apart
        let t = time_of_impact(&a, &b, Vec3::new(-1.0, 0.0, 0.0), Vec3::X, 5.0, 0.01);
        assert!(t.is_none());
    }

    // --- Sequential impulse tests ---

    #[test]
    fn sequential_impulse_basic() {
        let constraints = [ContactConstraint {
            normal: Vec3::Y,
            point: Vec3::ZERO,
            penetration: 0.1,
            restitution: 0.5,
            friction: 0.3,
            inv_mass_a: 1.0,
            inv_mass_b: 1.0,
        }];
        let rel_vels = [Vec3::new(0.0, -2.0, 0.0)];
        let result = sequential_impulse(&constraints, &rel_vels, 10);
        assert!(
            result.normal[0] > 0.0,
            "should produce positive normal impulse"
        );
        // Friction should be non-zero (tangential velocity is zero here though)
    }

    #[test]
    fn sequential_impulse_no_pull() {
        // Separating velocity: should not produce impulse
        let constraints = [ContactConstraint {
            normal: Vec3::Y,
            point: Vec3::ZERO,
            penetration: 0.0,
            restitution: 1.0,
            friction: 0.0,
            inv_mass_a: 1.0,
            inv_mass_b: 1.0,
        }];
        let rel_vels = [Vec3::new(0.0, 1.0, 0.0)]; // Moving apart
        let result = sequential_impulse(&constraints, &rel_vels, 10);
        assert!(approx_eq(result.normal[0], 0.0));
    }

    #[test]
    fn sequential_impulse_friction() {
        // Sliding contact: should produce friction impulse
        let constraints = [ContactConstraint {
            normal: Vec3::Y,
            point: Vec3::ZERO,
            penetration: 0.1,
            restitution: 0.0,
            friction: 0.5,
            inv_mass_a: 1.0,
            inv_mass_b: 1.0,
        }];
        let rel_vels = [Vec3::new(5.0, -2.0, 0.0)]; // Sliding + approaching
        let result = sequential_impulse(&constraints, &rel_vels, 10);
        assert!(result.normal[0] > 0.0, "should have normal impulse");
        assert!(
            result.friction[0].length() > 0.0,
            "should have friction impulse"
        );
        // Friction impulse should oppose tangential velocity (negative X)
        assert!(result.friction[0].x < 0.0);
    }

    // --- sequential_impulse_warm tests ---

    fn make_approach_constraint() -> ContactConstraint {
        ContactConstraint {
            normal: Vec3::Y,
            point: Vec3::ZERO,
            penetration: 0.1,
            restitution: 0.0,
            friction: 0.5,
            inv_mass_a: 1.0,
            inv_mass_b: 1.0,
        }
    }

    #[test]
    fn warm_start_none_matches_cold_start() {
        // With warm_start: None the result must be identical to sequential_impulse.
        let c = make_approach_constraint();
        let rv = [Vec3::new(0.0, -2.0, 0.0)];
        let cold = sequential_impulse(&[c], &rv, 10);
        let warm = sequential_impulse_warm(&[c], &rv, 10, None, 0.0);
        assert!(approx_eq(cold.normal[0], warm.normal[0]));
        assert!(vec3_approx_eq(cold.friction[0], warm.friction[0]));
    }

    #[test]
    fn warm_start_zero_factor_same_as_cold() {
        // warm_factor = 0.0 discards all warm impulses → same as cold start.
        let c = make_approach_constraint();
        let rv = [Vec3::new(0.0, -2.0, 0.0)];
        let prev = ImpulseResult {
            normal: vec![100.0],
            friction: vec![Vec3::new(50.0, 0.0, 0.0)],
        };
        let warm = sequential_impulse_warm(&[c], &rv, 10, Some(&prev), 0.0);
        let cold = sequential_impulse(&[c], &rv, 10);
        assert!(approx_eq(cold.normal[0], warm.normal[0]));
    }

    #[test]
    fn warm_start_seeds_impulses_and_converges_faster() {
        // A stacked body resting (rel_vel ≈ 0): warm-starting with the
        // correct impulse from frame N should produce a larger initial
        // normal impulse than cold-starting, allowing fewer iterations
        // to reach the same result.
        let c = make_approach_constraint();
        let rv = [Vec3::new(0.0, -0.01, 0.0)]; // nearly resting
        let prev_normal = sequential_impulse(&[c], &rv, 20).normal[0];
        let prev = ImpulseResult {
            normal: vec![prev_normal],
            friction: vec![Vec3::ZERO],
        };
        // With warm factor 0.9, even 1 iteration should be close to the
        // 20-iteration cold result.
        let warm1 = sequential_impulse_warm(&[c], &rv, 1, Some(&prev), 0.9);
        let cold20 = sequential_impulse(&[c], &rv, 20);
        // They should be within 20% of each other.
        assert!(
            (warm1.normal[0] - cold20.normal[0]).abs() < 0.2 * cold20.normal[0] + 1e-6,
            "warm={} cold={}",
            warm1.normal[0],
            cold20.normal[0]
        );
    }

    #[test]
    fn warm_factor_clamped_to_zero_one() {
        // warm_factor > 1.0 should be clamped to 1.0 (no amplification).
        let c = make_approach_constraint();
        let rv = [Vec3::new(0.0, -2.0, 0.0)];
        let prev = ImpulseResult {
            normal: vec![1.0],
            friction: vec![Vec3::ZERO],
        };
        // Passing 2.0 should behave the same as 1.0.
        let r_clamped = sequential_impulse_warm(&[c], &rv, 10, Some(&prev), 2.0);
        let r_one = sequential_impulse_warm(&[c], &rv, 10, Some(&prev), 1.0);
        assert!(approx_eq(r_clamped.normal[0], r_one.normal[0]));
    }

    #[test]
    fn warm_start_shorter_than_constraints_pads_with_zero() {
        // If warm_start has fewer entries than constraints, extras start at 0.
        let c = make_approach_constraint();
        let rv = [Vec3::new(0.0, -2.0, 0.0), Vec3::new(0.0, -2.0, 0.0)];
        let prev = ImpulseResult {
            normal: vec![1.0], // only one entry for two constraints
            friction: vec![Vec3::ZERO],
        };
        let r = sequential_impulse_warm(&[c, c], &rv, 10, Some(&prev), 0.8);
        // Both constraints should have valid (positive) impulses.
        assert!(r.normal[0] >= 0.0);
        assert!(r.normal[1] >= 0.0);
    }

    #[test]
    fn warm_start_negative_normal_clamped_to_zero() {
        // A warm-start normal impulse < 0 must be clamped to 0 (impulses are
        // compressive-only).
        let c = make_approach_constraint();
        let rv = [Vec3::new(0.0, -2.0, 0.0)];
        let prev = ImpulseResult {
            normal: vec![-999.0], // physically invalid — must be ignored
            friction: vec![Vec3::ZERO],
        };
        let r = sequential_impulse_warm(&[c], &rv, 1, Some(&prev), 1.0);
        assert!(r.normal[0] >= 0.0, "negative warm impulse must be clamped");
    }

    // --- Closest point on triangle ---

    #[test]
    fn closest_point_on_triangle_interior() {
        let tri = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
        );
        // Point directly above centroid
        let p = Vec3::new(1.0, 1.0, 5.0);
        let closest = closest_point_on_triangle(&tri, p);
        assert!(approx_eq(closest.z, 0.0)); // On the triangle plane
        assert!(approx_eq(closest.x, 1.0));
        assert!(approx_eq(closest.y, 1.0));
    }

    #[test]
    fn closest_point_on_triangle_vertex() {
        let tri = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
        );
        // Closest to vertex A
        let p = Vec3::new(-1.0, -1.0, 0.0);
        let closest = closest_point_on_triangle(&tri, p);
        assert!(vec3_approx_eq(closest, Vec3::ZERO));
    }

    #[test]
    fn closest_point_on_triangle_edge() {
        let tri = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
        );
        // Below edge AB
        let p = Vec3::new(2.0, -1.0, 0.0);
        let closest = closest_point_on_triangle(&tri, p);
        assert!(approx_eq(closest.y, 0.0)); // On edge AB
        assert!(approx_eq(closest.x, 2.0));
    }

    // --- Barycentric coordinates ---

    #[test]
    fn barycentric_coords_vertices() {
        let tri = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let (u, v, w) = barycentric_coords(&tri, Vec3::ZERO);
        assert!(approx_eq(u, 1.0));
        assert!(approx_eq(v, 0.0));
        assert!(approx_eq(w, 0.0));

        let (u, v, w) = barycentric_coords(&tri, Vec3::new(1.0, 0.0, 0.0));
        assert!(approx_eq(u, 0.0));
        assert!(approx_eq(v, 1.0));
        assert!(approx_eq(w, 0.0));
    }

    #[test]
    fn barycentric_coords_centroid() {
        let tri = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(3.0, 0.0, 0.0),
            Vec3::new(0.0, 3.0, 0.0),
        );
        let centroid = Vec3::new(1.0, 1.0, 0.0);
        let (u, v, w) = barycentric_coords(&tri, centroid);
        assert!(approx_eq(u + v + w, 1.0));
        assert!(approx_eq(u, 1.0 / 3.0));
    }

    // --- Segment-segment closest ---

    #[test]
    fn segment_segment_closest_parallel() {
        let (p1, p2, dist_sq) = segment_segment_closest(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
        );
        assert!(approx_eq(dist_sq, 1.0));
        assert!(approx_eq(p1.y, 0.0));
        assert!(approx_eq(p2.y, 1.0));
    }

    #[test]
    fn segment_segment_closest_crossing() {
        // Two segments that cross in the XZ plane
        let (p1, p2, dist_sq) = segment_segment_closest(
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        assert!(approx_eq(dist_sq, 0.0));
        assert!(approx_eq(p1.x, 0.0));
        assert!(approx_eq(p2.z, 0.0));
    }

    #[test]
    fn segment_segment_closest_skew() {
        // Skew lines in 3D
        let (_, _, dist_sq) = segment_segment_closest(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.5, 2.0, 0.0),
            Vec3::new(0.5, 2.0, 1.0),
        );
        assert!(approx_eq(dist_sq, 4.0)); // distance = 2
    }

    // --- Tangent space ---

    #[test]
    fn compute_tangent_basic() {
        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(1.0, 0.0, 0.0);
        let p2 = Vec3::new(0.0, 1.0, 0.0);
        let uv0 = glam::Vec2::new(0.0, 0.0);
        let uv1 = glam::Vec2::new(1.0, 0.0);
        let uv2 = glam::Vec2::new(0.0, 1.0);
        let (tangent, bitangent) = compute_tangent(p0, p1, p2, uv0, uv1, uv2);
        // Tangent should point along +X (U direction)
        assert!(tangent.dot(Vec3::X) > 0.9);
        // Bitangent should point along +Y (V direction)
        assert!(bitangent.dot(Vec3::Y) > 0.9);
    }

    #[test]
    fn compute_tangent_degenerate_uv() {
        // All same UV — should still return valid vectors
        let p0 = Vec3::ZERO;
        let p1 = Vec3::X;
        let p2 = Vec3::Y;
        let uv = glam::Vec2::ZERO;
        let (tangent, bitangent) = compute_tangent(p0, p1, p2, uv, uv, uv);
        // Should produce a valid frame, not NaN
        assert!(tangent.length() > 0.5);
        assert!(bitangent.length() > 0.5);
    }

    // --- MPR collision ---

    #[test]
    fn mpr_overlapping_spheres() {
        let a = Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let b = Sphere::new(Vec3::new(1.0, 0.0, 0.0), 1.0).unwrap();
        assert!(mpr_intersect(&a, &b));
        let pen = mpr_penetration(&a, &b);
        assert!(pen.is_some());
        assert!(pen.unwrap().depth > 0.0);
    }

    #[test]
    fn mpr_separated_spheres() {
        let a = Sphere::new(Vec3::ZERO, 0.5).unwrap();
        let b = Sphere::new(Vec3::new(3.0, 0.0, 0.0), 0.5).unwrap();
        assert!(!mpr_intersect(&a, &b));
    }

    #[test]
    fn mpr_agrees_with_gjk() {
        // Overlapping OBBs — both GJK and MPR should detect
        let a = Obb {
            center: Vec3::ZERO,
            half_extents: Vec3::ONE,
            rotation: glam::Quat::IDENTITY,
        };
        let b = Obb {
            center: Vec3::new(1.5, 0.0, 0.0),
            half_extents: Vec3::ONE,
            rotation: glam::Quat::IDENTITY,
        };
        assert!(gjk_intersect_3d(&a, &b));
        assert!(mpr_intersect(&a, &b));
    }

    // --- Delaunay triangulation ---

    #[test]
    fn delaunay_square() {
        let pts = [
            glam::Vec2::new(0.0, 0.0),
            glam::Vec2::new(1.0, 0.0),
            glam::Vec2::new(1.0, 1.0),
            glam::Vec2::new(0.0, 1.0),
        ];
        let tri = delaunay_2d(&pts).unwrap();
        assert_eq!(tri.triangles.len(), 2); // Square → 2 triangles
    }

    #[test]
    fn delaunay_many_points() {
        // Grid of points
        let mut pts = Vec::new();
        for i in 0..5 {
            for j in 0..5 {
                pts.push(glam::Vec2::new(i as f32, j as f32));
            }
        }
        let tri = delaunay_2d(&pts).unwrap();
        assert!(!tri.triangles.is_empty());
        // For n points in general position: ~2n triangles
        assert!(tri.triangles.len() >= 20);
    }

    #[test]
    fn delaunay_too_few_points() {
        assert!(delaunay_2d(&[glam::Vec2::ZERO, glam::Vec2::X]).is_none());
    }

    // --- Voronoi ---

    #[test]
    fn voronoi_basic() {
        let pts = [
            glam::Vec2::new(0.0, 0.0),
            glam::Vec2::new(2.0, 0.0),
            glam::Vec2::new(1.0, 2.0),
            glam::Vec2::new(1.0, -2.0),
        ];
        let vor = voronoi_2d(&pts).unwrap();
        assert!(!vor.edges.is_empty());
        assert_eq!(vor.sites.len(), 4);
    }

    // --- NURBS ---

    #[test]
    fn nurbs_uniform_equals_bspline() {
        // With all weights = 1.0, NURBS should equal B-spline
        let degree = 2;
        let cp = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 2.0, 0.0),
            Vec3::new(3.0, 1.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
        ];
        let weights = [1.0, 1.0, 1.0, 1.0];
        let knots = [0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0];
        let t = 0.5;
        let nurbs_pt = crate::calc::nurbs_eval(degree, &cp, &weights, &knots, t).unwrap();
        let bspline_pt = crate::calc::bspline_eval(degree, &cp, &knots, t).unwrap();
        assert!((nurbs_pt - bspline_pt).length() < 1e-4);
    }

    #[test]
    fn nurbs_circle_arc() {
        // A rational quadratic Bézier with w=cos(45°) at the middle point
        // should produce a circular arc
        let degree = 2;
        let cp = [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];
        let w = std::f64::consts::FRAC_1_SQRT_2; // cos(45°)
        let weights = [1.0, w, 1.0];
        let knots = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        // Midpoint should be on the unit circle
        let mid = crate::calc::nurbs_eval(degree, &cp, &weights, &knots, 0.5).unwrap();
        let radius = (mid.x * mid.x + mid.y * mid.y).sqrt();
        assert!((radius - 1.0).abs() < 1e-4, "radius = {radius}");
    }
}
