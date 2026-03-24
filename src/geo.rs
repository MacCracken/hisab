//! Geometric primitives and intersection tests.
//!
//! Provides rays, planes, axis-aligned bounding boxes, spheres, and
//! ray-intersection routines.

use glam::Vec3;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A ray defined by an origin and a direction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vec3,
    /// Should be normalized for correct distance results.
    pub direction: Vec3,
}

impl Ray {
    /// Create a new ray. Direction is normalized automatically.
    #[inline]
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
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Check whether a point is inside (or on the surface of) this sphere.
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

// ---------------------------------------------------------------------------
// Primitives
// ---------------------------------------------------------------------------

/// A triangle defined by three vertices.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self {
            vertices: [a, b, c],
        }
    }

    /// Face normal (not normalized). Returns the cross product of two edges.
    ///
    /// The magnitude equals twice the triangle's area. Use [`unit_normal`](Self::unit_normal)
    /// for a normalized version.
    #[inline]
    pub fn normal(&self) -> Vec3 {
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        edge1.cross(edge2)
    }

    /// Normalized face normal.
    #[inline]
    pub fn unit_normal(&self) -> Vec3 {
        self.normal().normalize()
    }

    /// Area of the triangle.
    #[inline]
    pub fn area(&self) -> f32 {
        self.normal().length() * 0.5
    }

    /// Centroid (average of the three vertices).
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
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Closest point on this infinite line to the given point.
    #[inline]
    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        let v = point - self.origin;
        let t = v.dot(self.direction);
        self.origin + self.direction * t
    }

    /// Distance from a point to this line.
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
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }

    /// Length of the segment.
    #[inline]
    pub fn length(&self) -> f32 {
        (self.end - self.start).length()
    }

    /// Midpoint of the segment.
    #[inline]
    pub fn midpoint(&self) -> Vec3 {
        (self.start + self.end) * 0.5
    }

    /// Normalized direction from start to end.
    #[inline]
    pub fn direction(&self) -> Vec3 {
        (self.end - self.start).normalize()
    }

    /// Closest point on this segment to the given point.
    #[inline]
    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        let ab = self.end - self.start;
        let len_sq = ab.dot(ab);
        if len_sq < 1e-12 {
            return self.start; // Degenerate segment
        }
        let t = ((point - self.start).dot(ab) / len_sq).clamp(0.0, 1.0);
        self.start + ab * t
    }

    /// Distance from a point to this segment.
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
    #[inline]
    pub fn contains_point(&self, point: Vec3) -> bool {
        self.planes.iter().all(|p| p.signed_distance(point) >= 0.0)
    }

    /// Conservative check whether an AABB intersects the frustum.
    ///
    /// Returns `false` only if the AABB is fully outside at least one plane.
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
}

// ---------------------------------------------------------------------------
// Intersection / overlap functions
// ---------------------------------------------------------------------------

/// Ray-triangle intersection using the Möller–Trumbore algorithm.
///
/// Returns the `t` parameter if the ray hits the triangle (only `t >= 0`).
#[inline]
pub fn ray_triangle(ray: &Ray, tri: &Triangle) -> Option<f32> {
    let edge1 = tri.vertices[1] - tri.vertices[0];
    let edge2 = tri.vertices[2] - tri.vertices[0];
    let h = ray.direction.cross(edge2);
    let a = edge1.dot(h);

    if a.abs() < 1e-8 {
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
#[inline]
pub fn aabb_aabb(a: &Aabb, b: &Aabb) -> bool {
    a.min.cmple(b.max).all() && b.min.cmple(a.max).all()
}

/// Check whether two spheres overlap.
#[inline]
pub fn sphere_sphere(a: &Sphere, b: &Sphere) -> bool {
    let r = a.radius + b.radius;
    (a.center - b.center).length_squared() <= r * r
}

/// Intersection of two planes. Returns the line of intersection, or `None` if parallel.
pub fn plane_plane(a: &Plane, b: &Plane) -> Option<Line> {
    let dir = a.normal.cross(b.normal);
    let len_sq = dir.dot(dir);
    if len_sq < 1e-12 {
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
// Closest-point functions
// ---------------------------------------------------------------------------

/// Closest point on a ray to a given point (clamped to `t >= 0`).
#[inline]
pub fn closest_point_on_ray(ray: &Ray, point: Vec3) -> Vec3 {
    let t = (point - ray.origin).dot(ray.direction).max(0.0);
    ray.origin + ray.direction * t
}

/// Closest point on a plane to a given point.
#[inline]
pub fn closest_point_on_plane(plane: &Plane, point: Vec3) -> Vec3 {
    point - plane.normal * plane.signed_distance(point)
}

/// Closest point on a sphere's surface to a given point.
///
/// If the point is at the sphere's center, returns the point offset by the radius along +X.
#[inline]
pub fn closest_point_on_sphere(sphere: &Sphere, point: Vec3) -> Vec3 {
    let dir = point - sphere.center;
    let len = dir.length();
    if len < 1e-8 {
        return sphere.center + Vec3::new(sphere.radius, 0.0, 0.0);
    }
    sphere.center + dir * (sphere.radius / len)
}

/// Closest point on an AABB's surface or interior to a given point.
#[inline]
pub fn closest_point_on_aabb(aabb: &Aabb, point: Vec3) -> Vec3 {
    point.clamp(aabb.min, aabb.max)
}

// ---------------------------------------------------------------------------
// BVH (Bounding Volume Hierarchy)
// ---------------------------------------------------------------------------

/// A node in a BVH tree.
#[derive(Debug, Clone)]
enum BvhNode {
    Leaf {
        bounds: Aabb,
        index: usize,
    },
    Internal {
        bounds: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

/// A Bounding Volume Hierarchy for fast ray-AABB broadphase queries.
///
/// Stores a tree of AABBs built from a list of primitives. Each primitive
/// is represented by its AABB and an index into the caller's primitive array.
#[derive(Debug, Clone)]
#[must_use]
pub struct Bvh {
    root: Option<BvhNode>,
    len: usize,
}

impl Bvh {
    /// Build a BVH from a list of (aabb, index) pairs using midpoint splitting.
    pub fn build(items: &mut [(Aabb, usize)]) -> Self {
        let len = items.len();
        if items.is_empty() {
            return Self { root: None, len: 0 };
        }
        let root = Self::build_recursive(items);
        Self {
            root: Some(root),
            len,
        }
    }

    fn build_recursive(items: &mut [(Aabb, usize)]) -> BvhNode {
        if items.len() == 1 {
            return BvhNode::Leaf {
                bounds: items[0].0,
                index: items[0].1,
            };
        }

        // Compute bounding box of all items
        let mut bounds = items[0].0;
        for item in items.iter().skip(1) {
            bounds = bounds.merge(&item.0);
        }

        // Split along the longest axis at the midpoint
        let size = bounds.size();
        let axis = if size.x >= size.y && size.x >= size.z {
            0
        } else if size.y >= size.z {
            1
        } else {
            2
        };

        let mid = match axis {
            0 => (bounds.min.x + bounds.max.x) * 0.5,
            1 => (bounds.min.y + bounds.max.y) * 0.5,
            _ => (bounds.min.z + bounds.max.z) * 0.5,
        };

        // Partition items around the midpoint (O(n) instead of O(n log n) sort)
        let center_val = |aabb: &Aabb| -> f32 {
            let arr = aabb.center().to_array();
            arr[axis]
        };

        // In-place partition: items with center < mid go left
        let mut split = 0;
        for i in 0..items.len() {
            if center_val(&items[i].0) < mid {
                items.swap(i, split);
                split += 1;
            }
        }
        // Ensure at least one item on each side
        if split == 0 {
            split = 1;
        } else if split == items.len() {
            split = items.len() - 1;
        }

        let (left_items, right_items) = items.split_at_mut(split);
        let left = Self::build_recursive(left_items);
        let right = Self::build_recursive(right_items);

        BvhNode::Internal {
            bounds,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Number of primitives in the BVH.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the BVH is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Query all primitive indices whose AABBs are hit by the ray.
    ///
    /// Returns indices in no particular order. Use the indices to test
    /// against the actual primitives for exact intersection.
    pub fn query_ray(&self, ray: &Ray) -> Vec<usize> {
        let mut results = Vec::new();
        if let Some(ref root) = self.root {
            Self::query_ray_recursive(root, ray, &mut results);
        }
        results
    }

    fn query_ray_recursive(node: &BvhNode, ray: &Ray, results: &mut Vec<usize>) {
        match node {
            BvhNode::Leaf { bounds, index } => {
                if ray_aabb(ray, bounds).is_some() {
                    results.push(*index);
                }
            }
            BvhNode::Internal {
                bounds,
                left,
                right,
            } => {
                if ray_aabb(ray, bounds).is_some() {
                    Self::query_ray_recursive(left, ray, results);
                    Self::query_ray_recursive(right, ray, results);
                }
            }
        }
    }

    /// Query all primitive indices whose AABBs overlap the given AABB.
    pub fn query_aabb(&self, query: &Aabb) -> Vec<usize> {
        let mut results = Vec::new();
        if let Some(ref root) = self.root {
            Self::query_aabb_recursive(root, query, &mut results);
        }
        results
    }

    fn query_aabb_recursive(node: &BvhNode, query: &Aabb, results: &mut Vec<usize>) {
        match node {
            BvhNode::Leaf { bounds, index } => {
                if aabb_aabb(bounds, query) {
                    results.push(*index);
                }
            }
            BvhNode::Internal {
                bounds,
                left,
                right,
            } => {
                if aabb_aabb(bounds, query) {
                    Self::query_aabb_recursive(left, query, results);
                    Self::query_aabb_recursive(right, query, results);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// K-d tree
// ---------------------------------------------------------------------------

/// A node in a k-d tree.
#[derive(Debug, Clone)]
enum KdNode {
    Leaf {
        point: Vec3,
        index: usize,
    },
    Split {
        axis: usize,
        split_val: f32,
        left: Box<KdNode>,
        right: Box<KdNode>,
    },
}

/// A 3D k-d tree for nearest-neighbor point queries.
#[derive(Debug, Clone)]
#[must_use]
pub struct KdTree {
    root: Option<KdNode>,
    len: usize,
}

impl KdTree {
    /// Build a k-d tree from a list of (point, index) pairs.
    pub fn build(items: &mut [(Vec3, usize)]) -> Self {
        let len = items.len();
        if items.is_empty() {
            return Self { root: None, len: 0 };
        }
        let root = Self::build_recursive(items, 0);
        Self {
            root: Some(root),
            len,
        }
    }

    fn build_recursive(items: &mut [(Vec3, usize)], depth: usize) -> KdNode {
        if items.len() == 1 {
            return KdNode::Leaf {
                point: items[0].0,
                index: items[0].1,
            };
        }

        let axis = depth % 3;
        let mid = items.len() / 2;
        // O(n) partial sort: only ensures the median element is in position
        items.select_nth_unstable_by(mid, |a, b| {
            a.0.to_array()[axis]
                .partial_cmp(&b.0.to_array()[axis])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let split_val = items[mid].0.to_array()[axis];

        let (left_items, right_items) = items.split_at_mut(mid);
        let left = Self::build_recursive(left_items, depth + 1);
        let right = Self::build_recursive(right_items, depth + 1);

        KdNode::Split {
            axis,
            split_val,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Number of points in the tree.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Find the nearest neighbor to `query`.
    ///
    /// Returns `(index, distance_squared)` of the closest point,
    /// or `None` if the tree is empty.
    pub fn nearest(&self, query: Vec3) -> Option<(usize, f32)> {
        let mut best_idx = 0;
        let mut best_dist_sq = f32::INFINITY;
        if let Some(ref root) = self.root {
            Self::nearest_recursive(root, query, &mut best_idx, &mut best_dist_sq);
        }
        if best_dist_sq.is_finite() {
            Some((best_idx, best_dist_sq))
        } else {
            None
        }
    }

    fn nearest_recursive(node: &KdNode, query: Vec3, best_idx: &mut usize, best_dist_sq: &mut f32) {
        match node {
            KdNode::Leaf { point, index } => {
                let dist_sq = (*point - query).length_squared();
                if dist_sq < *best_dist_sq {
                    *best_dist_sq = dist_sq;
                    *best_idx = *index;
                }
            }
            KdNode::Split {
                axis,
                split_val,
                left,
                right,
            } => {
                let query_val = query.to_array()[*axis];
                let diff = query_val - split_val;

                // Visit the closer side first
                let (near, far) = if diff < 0.0 {
                    (left.as_ref(), right.as_ref())
                } else {
                    (right.as_ref(), left.as_ref())
                };

                Self::nearest_recursive(near, query, best_idx, best_dist_sq);

                // Only visit the far side if the splitting plane is closer than current best
                if diff * diff < *best_dist_sq {
                    Self::nearest_recursive(far, query, best_idx, best_dist_sq);
                }
            }
        }
    }

    /// Find all points within `radius` of `query`.
    ///
    /// Returns a list of `(index, distance_squared)`.
    pub fn within_radius(&self, query: Vec3, radius: f32) -> Vec<(usize, f32)> {
        let mut results = Vec::new();
        let radius_sq = radius * radius;
        if let Some(ref root) = self.root {
            Self::radius_recursive(root, query, radius_sq, &mut results);
        }
        results
    }

    fn radius_recursive(
        node: &KdNode,
        query: Vec3,
        radius_sq: f32,
        results: &mut Vec<(usize, f32)>,
    ) {
        match node {
            KdNode::Leaf { point, index } => {
                let dist_sq = (*point - query).length_squared();
                if dist_sq <= radius_sq {
                    results.push((*index, dist_sq));
                }
            }
            KdNode::Split {
                axis,
                split_val,
                left,
                right,
            } => {
                let query_val = query.to_array()[*axis];
                let diff = query_val - split_val;

                let (near, far) = if diff < 0.0 {
                    (left.as_ref(), right.as_ref())
                } else {
                    (right.as_ref(), left.as_ref())
                };

                Self::radius_recursive(near, query, radius_sq, results);

                if diff * diff <= radius_sq {
                    Self::radius_recursive(far, query, radius_sq, results);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Quadtree (2D spatial index)
// ---------------------------------------------------------------------------

/// A 2D axis-aligned bounding rectangle for quadtree operations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub min: glam::Vec2,
    pub max: glam::Vec2,
}

impl Rect {
    /// Create a new rectangle. Min/max are corrected if swapped.
    #[inline]
    pub fn new(a: glam::Vec2, b: glam::Vec2) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }

    /// Check whether a 2D point is inside this rectangle.
    #[inline]
    pub fn contains_point(&self, p: glam::Vec2) -> bool {
        p.cmpge(self.min).all() && p.cmple(self.max).all()
    }

    /// Check whether two rectangles overlap.
    #[inline]
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.min.cmple(other.max).all() && other.min.cmple(self.max).all()
    }

    /// Center of the rectangle.
    #[inline]
    pub fn center(&self) -> glam::Vec2 {
        (self.min + self.max) * 0.5
    }

    /// Size (extents) of the rectangle.
    #[inline]
    pub fn size(&self) -> glam::Vec2 {
        self.max - self.min
    }

    /// Merge two rectangles into one that encloses both.
    #[inline]
    pub fn merge(&self, other: &Rect) -> Rect {
        Rect {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Area of the rectangle.
    #[inline]
    pub fn area(&self) -> f32 {
        let s = self.size();
        s.x * s.y
    }
}

/// A quadtree node.
#[derive(Debug, Clone)]
enum QuadNode {
    Empty,
    Leaf(Vec<(glam::Vec2, usize)>),
    Split {
        children: Box<[QuadNode; 4]>, // NW, NE, SW, SE
        bounds: Rect,
    },
}

/// A 2D quadtree for spatial point queries.
///
/// Points are inserted into a fixed-bounds region. The tree subdivides
/// when a leaf exceeds `max_per_leaf` items, up to `max_depth` levels.
#[derive(Debug, Clone)]
pub struct Quadtree {
    root: QuadNode,
    bounds: Rect,
    len: usize,
    max_per_leaf: usize,
    max_depth: usize,
}

impl Quadtree {
    /// Create a new empty quadtree covering the given bounds.
    ///
    /// `max_per_leaf`: max items before a leaf splits (default 8).
    /// `max_depth`: max subdivision depth (default 8).
    pub fn new(bounds: Rect, max_per_leaf: usize, max_depth: usize) -> Self {
        Self {
            root: QuadNode::Empty,
            bounds,
            len: 0,
            max_per_leaf,
            max_depth,
        }
    }

    /// Number of items in the quadtree.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the quadtree is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Insert a point with an associated index.
    pub fn insert(&mut self, point: glam::Vec2, index: usize) {
        if !self.bounds.contains_point(point) {
            return; // Out of bounds, ignore
        }
        Self::insert_recursive(
            &mut self.root,
            point,
            index,
            &self.bounds,
            self.max_per_leaf,
            self.max_depth,
            0,
        );
        self.len += 1;
    }

    fn insert_recursive(
        node: &mut QuadNode,
        point: glam::Vec2,
        index: usize,
        bounds: &Rect,
        max_per_leaf: usize,
        max_depth: usize,
        depth: usize,
    ) {
        match node {
            QuadNode::Empty => {
                *node = QuadNode::Leaf(vec![(point, index)]);
            }
            QuadNode::Leaf(items) => {
                items.push((point, index));
                if items.len() > max_per_leaf && depth < max_depth {
                    // Split
                    let center = bounds.center();
                    let mut children = Box::new([
                        QuadNode::Empty,
                        QuadNode::Empty,
                        QuadNode::Empty,
                        QuadNode::Empty,
                    ]);
                    for &(p, idx) in items.iter() {
                        let quadrant = Self::quadrant(p, center);
                        let child_bounds = Self::child_bounds(bounds, quadrant);
                        Self::insert_recursive(
                            &mut children[quadrant],
                            p,
                            idx,
                            &child_bounds,
                            max_per_leaf,
                            max_depth,
                            depth + 1,
                        );
                    }
                    *node = QuadNode::Split {
                        children,
                        bounds: *bounds,
                    };
                }
            }
            QuadNode::Split {
                children,
                bounds: node_bounds,
            } => {
                let center = node_bounds.center();
                let quadrant = Self::quadrant(point, center);
                let child_bounds = Self::child_bounds(node_bounds, quadrant);
                Self::insert_recursive(
                    &mut children[quadrant],
                    point,
                    index,
                    &child_bounds,
                    max_per_leaf,
                    max_depth,
                    depth + 1,
                );
            }
        }
    }

    fn quadrant(point: glam::Vec2, center: glam::Vec2) -> usize {
        let x = if point.x >= center.x { 1 } else { 0 };
        let y = if point.y >= center.y { 0 } else { 2 }; // NW=0, NE=1, SW=2, SE=3
        x + y
    }

    fn child_bounds(bounds: &Rect, quadrant: usize) -> Rect {
        let center = bounds.center();
        match quadrant {
            0 => Rect::new(
                glam::Vec2::new(bounds.min.x, center.y),
                glam::Vec2::new(center.x, bounds.max.y),
            ), // NW
            1 => Rect::new(center, bounds.max), // NE
            2 => Rect::new(bounds.min, center), // SW
            _ => Rect::new(
                glam::Vec2::new(center.x, bounds.min.y),
                glam::Vec2::new(bounds.max.x, center.y),
            ), // SE
        }
    }

    /// Query all indices of points within the given rectangle.
    pub fn query_rect(&self, query: &Rect) -> Vec<usize> {
        let mut results = Vec::new();
        Self::query_recursive(&self.root, &self.bounds, query, &mut results);
        results
    }

    fn query_recursive(node: &QuadNode, bounds: &Rect, query: &Rect, results: &mut Vec<usize>) {
        if !bounds.overlaps(query) {
            return;
        }
        match node {
            QuadNode::Empty => {}
            QuadNode::Leaf(items) => {
                for &(p, idx) in items {
                    if query.contains_point(p) {
                        results.push(idx);
                    }
                }
            }
            QuadNode::Split {
                children,
                bounds: node_bounds,
            } => {
                for q in 0..4 {
                    let child_bounds = Self::child_bounds(node_bounds, q);
                    Self::query_recursive(&children[q], &child_bounds, query, results);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Octree (3D spatial index)
// ---------------------------------------------------------------------------

/// An octree node.
#[derive(Debug, Clone)]
enum OctNode {
    Empty,
    Leaf(Vec<(Vec3, usize)>),
    Split {
        children: Box<[OctNode; 8]>,
        bounds: Aabb,
    },
}

/// A 3D octree for spatial point queries.
///
/// Same design as [`Quadtree`] but in 3D with 8 children per split.
#[derive(Debug, Clone)]
pub struct Octree {
    root: OctNode,
    bounds: Aabb,
    len: usize,
    max_per_leaf: usize,
    max_depth: usize,
}

impl Octree {
    /// Create a new empty octree covering the given bounds.
    pub fn new(bounds: Aabb, max_per_leaf: usize, max_depth: usize) -> Self {
        Self {
            root: OctNode::Empty,
            bounds,
            len: 0,
            max_per_leaf,
            max_depth,
        }
    }

    /// Number of items in the octree.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the octree is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Insert a point with an associated index.
    pub fn insert(&mut self, point: Vec3, index: usize) {
        if !self.bounds.contains(point) {
            return;
        }
        Self::insert_recursive(
            &mut self.root,
            point,
            index,
            &self.bounds,
            self.max_per_leaf,
            self.max_depth,
            0,
        );
        self.len += 1;
    }

    fn insert_recursive(
        node: &mut OctNode,
        point: Vec3,
        index: usize,
        bounds: &Aabb,
        max_per_leaf: usize,
        max_depth: usize,
        depth: usize,
    ) {
        match node {
            OctNode::Empty => {
                *node = OctNode::Leaf(vec![(point, index)]);
            }
            OctNode::Leaf(items) => {
                items.push((point, index));
                if items.len() > max_per_leaf && depth < max_depth {
                    let center = bounds.center();
                    let mut children = Box::new([
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                        OctNode::Empty,
                    ]);
                    for &(p, idx) in items.iter() {
                        let octant = Self::octant(p, center);
                        let child_bounds = Self::child_bounds(bounds, octant);
                        Self::insert_recursive(
                            &mut children[octant],
                            p,
                            idx,
                            &child_bounds,
                            max_per_leaf,
                            max_depth,
                            depth + 1,
                        );
                    }
                    *node = OctNode::Split {
                        children,
                        bounds: *bounds,
                    };
                }
            }
            OctNode::Split {
                children,
                bounds: node_bounds,
            } => {
                let center = node_bounds.center();
                let octant = Self::octant(point, center);
                let child_bounds = Self::child_bounds(node_bounds, octant);
                Self::insert_recursive(
                    &mut children[octant],
                    point,
                    index,
                    &child_bounds,
                    max_per_leaf,
                    max_depth,
                    depth + 1,
                );
            }
        }
    }

    fn octant(point: Vec3, center: Vec3) -> usize {
        let x = if point.x >= center.x { 1 } else { 0 };
        let y = if point.y >= center.y { 2 } else { 0 };
        let z = if point.z >= center.z { 4 } else { 0 };
        x | y | z
    }

    fn child_bounds(bounds: &Aabb, octant: usize) -> Aabb {
        let center = bounds.center();
        let min = Vec3::new(
            if octant & 1 != 0 {
                center.x
            } else {
                bounds.min.x
            },
            if octant & 2 != 0 {
                center.y
            } else {
                bounds.min.y
            },
            if octant & 4 != 0 {
                center.z
            } else {
                bounds.min.z
            },
        );
        let max = Vec3::new(
            if octant & 1 != 0 {
                bounds.max.x
            } else {
                center.x
            },
            if octant & 2 != 0 {
                bounds.max.y
            } else {
                center.y
            },
            if octant & 4 != 0 {
                bounds.max.z
            } else {
                center.z
            },
        );
        Aabb::new(min, max)
    }

    /// Query all indices of points within the given AABB.
    pub fn query_aabb(&self, query: &Aabb) -> Vec<usize> {
        let mut results = Vec::new();
        Self::query_recursive(&self.root, &self.bounds, query, &mut results);
        results
    }

    fn query_recursive(node: &OctNode, bounds: &Aabb, query: &Aabb, results: &mut Vec<usize>) {
        if !aabb_aabb(bounds, query) {
            return;
        }
        match node {
            OctNode::Empty => {}
            OctNode::Leaf(items) => {
                for &(p, idx) in items {
                    if query.contains(p) {
                        results.push(idx);
                    }
                }
            }
            OctNode::Split {
                children,
                bounds: node_bounds,
            } => {
                for oct in 0..8 {
                    let child_bounds = Self::child_bounds(node_bounds, oct);
                    Self::query_recursive(&children[oct], &child_bounds, query, results);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Spatial hash grid
// ---------------------------------------------------------------------------

/// A spatial hash grid for fast broadphase queries.
///
/// Divides space into a uniform grid of cells. Each cell stores indices
/// of items whose positions fall within it. O(1) insert and query for
/// localized spatial lookups.
#[derive(Debug, Clone)]
pub struct SpatialHash {
    inv_cell_size: f32,
    cells: std::collections::HashMap<(i32, i32, i32), Vec<usize>>,
    len: usize,
}

impl SpatialHash {
    /// Create a new spatial hash grid with the given cell size.
    pub fn new(cell_size: f32) -> Self {
        assert!(cell_size > 0.0, "cell_size must be positive");
        Self {
            inv_cell_size: 1.0 / cell_size,
            cells: std::collections::HashMap::new(),
            len: 0,
        }
    }

    /// Number of items inserted.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the grid is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Number of occupied cells.
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    fn cell_key(&self, point: Vec3) -> (i32, i32, i32) {
        (
            (point.x * self.inv_cell_size).floor() as i32,
            (point.y * self.inv_cell_size).floor() as i32,
            (point.z * self.inv_cell_size).floor() as i32,
        )
    }

    /// Insert a point with an associated index.
    pub fn insert(&mut self, point: Vec3, index: usize) {
        let key = self.cell_key(point);
        self.cells.entry(key).or_default().push(index);
        self.len += 1;
    }

    /// Clear all items.
    pub fn clear(&mut self) {
        self.cells.clear();
        self.len = 0;
    }

    /// Query all indices in the same cell as the given point.
    pub fn query_cell(&self, point: Vec3) -> &[usize] {
        let key = self.cell_key(point);
        self.cells.get(&key).map_or(&[], |v| v.as_slice())
    }

    /// Query all indices within a radius of the given point (broadphase).
    ///
    /// Checks all cells that could contain points within `radius`.
    /// Returns candidate indices — the caller should perform exact distance
    /// checks. Cost is O((2*ceil(r/cell_size)+1)^3) in cell lookups.
    pub fn query_radius(&self, point: Vec3, radius: f32) -> Vec<usize> {
        let mut results = Vec::new();
        let cells_r = (radius * self.inv_cell_size).ceil() as i32;
        let center = self.cell_key(point);

        for dx in -cells_r..=cells_r {
            for dy in -cells_r..=cells_r {
                for dz in -cells_r..=cells_r {
                    let key = (center.0 + dx, center.1 + dy, center.2 + dz);
                    if let Some(items) = self.cells.get(&key) {
                        results.extend_from_slice(items);
                    }
                }
            }
        }
        results
    }
}

// ---------------------------------------------------------------------------
// Convex hull (2D, Andrew's monotone chain)
// ---------------------------------------------------------------------------

/// Compute the 2D convex hull of a set of points.
///
/// Returns the hull vertices in counter-clockwise order using Andrew's
/// monotone chain algorithm. O(n log n).
///
/// For fewer than 2 points, returns the input as-is. For collinear points,
/// returns only the two endpoints.
pub fn convex_hull_2d(points: &[glam::Vec2]) -> Vec<glam::Vec2> {
    let mut pts: Vec<glam::Vec2> = points.to_vec();
    let n = pts.len();
    if n < 2 {
        return pts;
    }

    pts.sort_unstable_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
    });

    // Cross product of OA and OB vectors
    let cross = |o: glam::Vec2, a: glam::Vec2, b: glam::Vec2| -> f32 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    };

    let mut hull: Vec<glam::Vec2> = Vec::with_capacity(2 * n);

    // Lower hull
    for &p in &pts {
        while hull.len() >= 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0 {
            hull.pop();
        }
        hull.push(p);
    }

    // Upper hull
    let lower_len = hull.len() + 1;
    for &p in pts.iter().rev() {
        while hull.len() >= lower_len && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0.0
        {
            hull.pop();
        }
        hull.push(p);
    }

    hull.pop(); // Remove the last point (same as first)
    hull
}

// ---------------------------------------------------------------------------
// GJK (Gilbert-Johnson-Keerthi) collision detection
// ---------------------------------------------------------------------------

/// A convex shape that can compute a support point in a given direction.
///
/// The support function returns the point on the shape that is farthest
/// in the given direction.
pub trait ConvexSupport {
    /// Return the point on the shape farthest in `direction`.
    fn support(&self, direction: glam::Vec2) -> glam::Vec2;
}

/// A convex polygon for GJK/EPA (2D).
///
/// Vertices should be in counter-clockwise order. Use [`convex_hull_2d`]
/// to construct from an arbitrary point set.
#[derive(Debug, Clone)]
pub struct ConvexPolygon {
    pub vertices: Vec<glam::Vec2>,
}

impl ConvexPolygon {
    /// Create a convex polygon. Vertices should be in CCW order.
    /// Use [`convex_hull_2d`] to ensure convexity.
    pub fn new(vertices: Vec<glam::Vec2>) -> Self {
        Self { vertices }
    }
}

impl ConvexSupport for ConvexPolygon {
    #[inline]
    fn support(&self, direction: glam::Vec2) -> glam::Vec2 {
        let mut best = self.vertices[0];
        let mut best_dot = best.dot(direction);
        for &v in &self.vertices[1..] {
            let d = v.dot(direction);
            if d > best_dot {
                best_dot = d;
                best = v;
            }
        }
        best
    }
}

/// Result of a Minkowski difference support query.
#[inline]
fn minkowski_support(
    a: &dyn ConvexSupport,
    b: &dyn ConvexSupport,
    direction: glam::Vec2,
) -> glam::Vec2 {
    a.support(direction) - b.support(-direction)
}

/// Triple product: (A × B) × C — returns the vector perpendicular to C
/// in the direction away from A, used for simplex evolution.
#[inline]
fn triple_cross_2d(a: glam::Vec2, b: glam::Vec2, c: glam::Vec2) -> glam::Vec2 {
    // In 2D: (A × B) × C = B * (C·A) - A * (C·B)
    let ca = c.dot(a);
    let cb = c.dot(b);
    b * ca - a * cb
}

/// GJK collision test between two convex shapes (2D).
///
/// Returns `true` if the shapes overlap. Uses the simplex-based GJK algorithm.
pub fn gjk_intersect(a: &dyn ConvexSupport, b: &dyn ConvexSupport) -> bool {
    // Initial direction: from center of A to center of B (approximated)
    let mut direction = glam::Vec2::new(1.0, 0.0);

    let mut simplex: Vec<glam::Vec2> = Vec::with_capacity(3);
    let s = minkowski_support(a, b, direction);
    simplex.push(s);
    direction = -s;

    for _ in 0..64 {
        let new_point = minkowski_support(a, b, direction);

        if new_point.dot(direction) < 0.0 {
            return false; // No collision — didn't pass the origin
        }

        simplex.push(new_point);

        // Check if simplex contains the origin
        match simplex.len() {
            2 => {
                // Line case
                let b_pt = simplex[1];
                let a_pt = simplex[0];
                let ab = a_pt - b_pt;
                let ao = -b_pt;
                direction = triple_cross_2d(ab, ao, ab);
                if direction.length_squared() < 1e-12 {
                    return true; // Origin is on the line segment
                }
            }
            3 => {
                // Triangle case
                let c = simplex[2];
                let b_pt = simplex[1];
                let a_pt = simplex[0];
                let cb = b_pt - c;
                let ca = a_pt - c;
                let co = -c;

                let cb_perp = triple_cross_2d(ca, cb, cb);
                let ca_perp = triple_cross_2d(cb, ca, ca);

                if cb_perp.dot(co) > 0.0 {
                    // Region outside CB
                    simplex.remove(0); // Remove A
                    direction = cb_perp;
                } else if ca_perp.dot(co) > 0.0 {
                    // Region outside CA
                    simplex.remove(1); // Remove B
                    direction = ca_perp;
                } else {
                    // Origin is inside the triangle
                    return true;
                }
            }
            _ => unreachable!(),
        }
    }

    false // Max iterations
}

// ---------------------------------------------------------------------------
// EPA (Expanding Polytope Algorithm) — penetration depth
// ---------------------------------------------------------------------------

/// Penetration result from EPA.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Penetration {
    /// Penetration normal (direction to separate).
    pub normal: glam::Vec2,
    /// Penetration depth (distance to separate).
    pub depth: f32,
}

/// Compute the penetration depth and normal between two overlapping convex shapes
/// using the Expanding Polytope Algorithm (EPA).
///
/// Must only be called when GJK has confirmed an intersection. The `simplex`
/// should be the final 3-point simplex from GJK that contains the origin.
///
/// Returns `None` if the shapes are not actually overlapping or if EPA fails.
pub fn epa_penetration(
    a: &dyn ConvexSupport,
    b: &dyn ConvexSupport,
    simplex: &[glam::Vec2],
) -> Option<Penetration> {
    if simplex.len() < 3 {
        return None;
    }

    let mut polytope: Vec<glam::Vec2> = simplex.to_vec();

    for _ in 0..64 {
        // Find the closest edge to the origin
        let mut closest_dist = f32::INFINITY;
        let mut closest_normal = glam::Vec2::ZERO;
        let mut closest_idx = 0;

        for i in 0..polytope.len() {
            let j = (i + 1) % polytope.len();
            let edge = polytope[j] - polytope[i];
            // Outward normal (2D: perpendicular)
            let normal = glam::Vec2::new(edge.y, -edge.x).normalize();
            let dist = normal.dot(polytope[i]);

            if dist < closest_dist {
                closest_dist = dist;
                closest_normal = normal;
                closest_idx = j;
            }
        }

        // Get new support point in the direction of the closest normal
        let support = minkowski_support(a, b, closest_normal);
        let d = support.dot(closest_normal);

        if (d - closest_dist).abs() < 1e-6 {
            // Converged — depth is always positive
            return Some(Penetration {
                normal: closest_normal,
                depth: closest_dist.abs(),
            });
        }

        // Insert the new point into the polytope
        polytope.insert(closest_idx, support);
    }

    // Return best estimate
    let mut closest_dist = f32::INFINITY;
    let mut closest_normal = glam::Vec2::ZERO;
    for i in 0..polytope.len() {
        let j = (i + 1) % polytope.len();
        let edge = polytope[j] - polytope[i];
        let normal = glam::Vec2::new(edge.y, -edge.x).normalize();
        let dist = normal.dot(polytope[i]);
        if dist < closest_dist {
            closest_dist = dist;
            closest_normal = normal;
        }
    }
    Some(Penetration {
        normal: closest_normal,
        depth: closest_dist.abs(),
    })
}

/// Combined GJK + EPA: test intersection and compute penetration if overlapping.
///
/// Returns `None` if shapes don't overlap, or `Some(Penetration)` with
/// the separation normal and depth.
pub fn gjk_epa(a: &dyn ConvexSupport, b: &dyn ConvexSupport) -> Option<Penetration> {
    let mut direction = glam::Vec2::new(1.0, 0.0);
    let mut simplex: Vec<glam::Vec2> = Vec::with_capacity(3);

    let s = minkowski_support(a, b, direction);
    simplex.push(s);
    direction = -s;

    for _ in 0..64 {
        let new_point = minkowski_support(a, b, direction);
        if new_point.dot(direction) < 0.0 {
            return None;
        }
        simplex.push(new_point);

        match simplex.len() {
            2 => {
                let b_pt = simplex[1];
                let a_pt = simplex[0];
                let ab = a_pt - b_pt;
                let ao = -b_pt;
                direction = triple_cross_2d(ab, ao, ab);
                if direction.length_squared() < 1e-12 {
                    // Degenerate — origin on line, need third point
                    direction = glam::Vec2::new(-ab.y, ab.x);
                }
            }
            3 => {
                let c = simplex[2];
                let b_pt = simplex[1];
                let a_pt = simplex[0];
                let cb = b_pt - c;
                let ca = a_pt - c;
                let co = -c;

                let cb_perp = triple_cross_2d(ca, cb, cb);
                let ca_perp = triple_cross_2d(cb, ca, ca);

                if cb_perp.dot(co) > 0.0 {
                    simplex.remove(0);
                    direction = cb_perp;
                } else if ca_perp.dot(co) > 0.0 {
                    simplex.remove(1);
                    direction = ca_perp;
                } else {
                    // Contains origin — run EPA
                    return epa_penetration(a, b, &simplex);
                }
            }
            _ => unreachable!(),
        }
    }

    None
}

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
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        assert_eq!(r.at(0.0), Vec3::ZERO);
        assert!(approx_eq(r.at(5.0).x, 5.0));
    }

    #[test]
    fn plane_from_point_normal() {
        let p = Plane::from_point_normal(Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
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
        use crate::HisabError;
        let e = HisabError::Degenerate("zero-length edge".to_string());
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
        let l = Line::new(Vec3::ZERO, Vec3::X);
        let p = Vec3::new(5.0, 3.0, 0.0);
        let cp = l.closest_point(p);
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn line_distance_to_point() {
        let l = Line::new(Vec3::ZERO, Vec3::X);
        let d = l.distance_to_point(Vec3::new(5.0, 3.0, 4.0));
        assert!(approx_eq(d, 5.0)); // sqrt(9+16) = 5
    }

    #[test]
    fn line_closest_point_behind_origin() {
        // Line is infinite — should work for negative t
        let l = Line::new(Vec3::ZERO, Vec3::X);
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
        let r = Ray::new(Vec3::ZERO, Vec3::Z);
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
        let r = Ray::new(Vec3::new(10.0, 10.0, 0.0), Vec3::Z);
        assert!(ray_triangle(&r, &tri).is_none());
    }

    #[test]
    fn ray_triangle_parallel() {
        // Ray parallel to triangle plane
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        let r = Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::X);
        assert!(ray_triangle(&r, &tri).is_none());
    }

    #[test]
    fn ray_triangle_behind() {
        let tri = Triangle::new(
            Vec3::new(-1.0, -1.0, -5.0),
            Vec3::new(1.0, -1.0, -5.0),
            Vec3::new(0.0, 1.0, -5.0),
        );
        let r = Ray::new(Vec3::ZERO, Vec3::Z); // Points away from triangle
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
        let a = Sphere::new(Vec3::ZERO, 1.0);
        let b = Sphere::new(Vec3::new(1.5, 0.0, 0.0), 1.0);
        assert!(sphere_sphere(&a, &b));
    }

    #[test]
    fn sphere_sphere_no_overlap() {
        let a = Sphere::new(Vec3::ZERO, 1.0);
        let b = Sphere::new(Vec3::new(3.0, 0.0, 0.0), 1.0);
        assert!(!sphere_sphere(&a, &b));
    }

    #[test]
    fn sphere_sphere_touching() {
        let a = Sphere::new(Vec3::ZERO, 1.0);
        let b = Sphere::new(Vec3::new(2.0, 0.0, 0.0), 1.0);
        assert!(sphere_sphere(&a, &b)); // Touching = overlap
    }

    // Plane-plane intersection tests
    #[test]
    fn plane_plane_intersection() {
        let a = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let b = Plane::from_point_normal(Vec3::ZERO, Vec3::X);
        let line = plane_plane(&a, &b).unwrap();
        // Intersection should be along Z axis
        assert!(approx_eq(line.direction.z.abs(), 1.0));
    }

    #[test]
    fn plane_plane_parallel() {
        let a = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let b = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
        assert!(plane_plane(&a, &b).is_none());
    }

    // Closest-point tests
    #[test]
    fn closest_on_ray_forward() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        let cp = closest_point_on_ray(&r, Vec3::new(5.0, 3.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_ray_clamped() {
        // Point behind the ray — should clamp to origin
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        let cp = closest_point_on_ray(&r, Vec3::new(-5.0, 3.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::ZERO));
    }

    #[test]
    fn closest_on_plane() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let cp = closest_point_on_plane(&p, Vec3::new(3.0, 7.0, -2.0));
        assert!(vec3_approx_eq(cp, Vec3::new(3.0, 0.0, -2.0)));
    }

    #[test]
    fn closest_on_sphere_outside() {
        let s = Sphere::new(Vec3::ZERO, 1.0);
        let cp = closest_point_on_sphere(&s, Vec3::new(10.0, 0.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_sphere_inside() {
        let s = Sphere::new(Vec3::ZERO, 10.0);
        let cp = closest_point_on_sphere(&s, Vec3::new(1.0, 0.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(10.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_sphere_at_center() {
        let s = Sphere::new(Vec3::ZERO, 5.0);
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
        let l = Line::new(Vec3::ZERO, Vec3::X);
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
        let a = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let b = Plane::from_point_normal(Vec3::ZERO, Vec3::X);
        let line = plane_plane(&a, &b).unwrap();
        // The origin should be close to the intersection line
        let cp = line.closest_point(Vec3::ZERO);
        assert!(approx_eq(cp.length(), 0.0));
    }

    #[test]
    fn closest_on_sphere_direction_consistent() {
        let s = Sphere::new(Vec3::ZERO, 5.0);
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
        let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::Z);
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
        let l = Line::new(Vec3::new(0.0, 5.0, 0.0), Vec3::X);
        assert!(approx_eq(l.distance_to_point(Vec3::ZERO), 5.0));
    }

    #[test]
    fn closest_on_plane_already_on_plane() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let point = Vec3::new(3.0, 0.0, -7.0);
        let cp = closest_point_on_plane(&p, point);
        assert!(vec3_approx_eq(cp, point));
    }

    #[test]
    fn closest_on_ray_along_direction() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        let cp = closest_point_on_ray(&r, Vec3::new(5.0, 0.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn sphere_sphere_concentric() {
        let a = Sphere::new(Vec3::ZERO, 1.0);
        let b = Sphere::new(Vec3::ZERO, 0.5);
        assert!(sphere_sphere(&a, &b));
    }

    // --- V0.5a: BVH ---

    #[test]
    fn bvh_empty() {
        let bvh = Bvh::build(&mut []);
        assert!(bvh.is_empty());
        assert_eq!(bvh.len(), 0);
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        assert!(bvh.query_ray(&r).is_empty());
    }

    #[test]
    fn bvh_single() {
        let bb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::ONE);
        let mut items = [(bb, 42)];
        let bvh = Bvh::build(&mut items);
        assert_eq!(bvh.len(), 1);

        let r = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z);
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
        let r = Ray::new(Vec3::new(0.5, 0.5, -5.0), Vec3::Z);
        let hits = bvh.query_ray(&r);
        assert!(hits.contains(&0));
        assert!(!hits.contains(&5));

        // Ray missing everything (way above)
        let r_miss = Ray::new(Vec3::new(0.5, 100.0, -5.0), Vec3::Z);
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

        let r = Ray::new(Vec3::new(0.25, 0.25, -10.0), Vec3::Z);
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
        let r = Ray::new(Vec3::new(0.5, 0.5, -5.0), Vec3::Z);
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
        let r = Ray::new(Vec3::new(5.5, 0.5, -5.0), Vec3::Z);
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
        let sh = SpatialHash::new(1.0);
        assert!(sh.is_empty());
        assert_eq!(sh.cell_count(), 0);
    }

    #[test]
    fn spatial_hash_insert_and_query_cell() {
        let mut sh = SpatialHash::new(10.0);
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
        let mut sh = SpatialHash::new(5.0);
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
        let mut sh = SpatialHash::new(1.0);
        sh.insert(Vec3::ZERO, 0);
        sh.insert(Vec3::ONE, 1);
        assert_eq!(sh.len(), 2);
        sh.clear();
        assert!(sh.is_empty());
        assert_eq!(sh.cell_count(), 0);
    }

    #[test]
    fn spatial_hash_negative_coords() {
        let mut sh = SpatialHash::new(1.0);
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
        let mut sh = SpatialHash::new(100.0); // Huge cell
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
        ]);
        let b = ConvexPolygon::new(vec![
            glam::Vec2::new(1.0, 0.0),
            glam::Vec2::new(3.0, 0.0),
            glam::Vec2::new(2.0, 2.0),
        ]);
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

    // --- V1.0b: Display impls ---

    #[test]
    fn ray_display() {
        let r = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::X);
        assert_eq!(r.to_string(), "Ray(1, 0, 0 -> 1, 0, 0)");
    }

    #[test]
    fn ray_display_precision() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::X);
        assert_eq!(format!("{r:.1}"), "Ray(1.0, 2.0, 3.0 -> 1.0, 0.0, 0.0)");
    }

    #[test]
    fn plane_display() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        assert_eq!(p.to_string(), "Plane(n=(0, 1, 0), d=0)");
    }

    #[test]
    fn plane_display_precision() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        assert_eq!(format!("{p:.2}"), "Plane(n=(0.00, 1.00, 0.00), d=0.00)");
    }

    #[test]
    fn aabb_display() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert_eq!(a.to_string(), "Aabb((0, 0, 0)..(1, 1, 1))");
    }

    #[test]
    fn sphere_display() {
        let sp = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 5.0);
        assert_eq!(sp.to_string(), "Sphere((1, 2, 3), r=5)");
    }

    #[test]
    fn sphere_display_precision() {
        let sp = Sphere::new(Vec3::ZERO, 2.5);
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
}
