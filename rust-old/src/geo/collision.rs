use super::*;

/// Maximum iterations for GJK collision detection.
pub const GJK_MAX_ITERATIONS: usize = 64;
/// Maximum iterations for EPA penetration depth.
pub const EPA_MAX_ITERATIONS: usize = 64;

// Convex hull (2D, Andrew's monotone chain)
// ---------------------------------------------------------------------------

/// Compute the 2D convex hull of a set of points.
///
/// Returns the hull vertices in counter-clockwise order using Andrew's
/// monotone chain algorithm. O(n log n).
///
/// For fewer than 2 points, returns the input as-is. For collinear points,
/// returns only the two endpoints.
///
/// **Note:** This function clones the input slice for sorting. For very large
/// point sets, consider pre-sorting the points yourself.
#[must_use]
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
    ///
    /// # Errors
    ///
    /// Returns [`crate::HisabError::InvalidInput`] if `vertices` is empty.
    pub fn new(vertices: Vec<glam::Vec2>) -> Result<Self, crate::HisabError> {
        if vertices.is_empty() {
            return Err(crate::HisabError::InvalidInput(
                "convex polygon requires at least one vertex".into(),
            ));
        }
        Ok(Self { vertices })
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

/// GJK simplex evolution result.
enum GjkResult {
    /// No intersection found.
    NoIntersection,
    /// Intersection confirmed — simplex contains origin.
    Intersection([glam::Vec2; 3], usize),
}

/// Core GJK loop shared by `gjk_intersect` and `gjk_epa`.
///
/// Returns the final simplex if the origin is contained, or `NoIntersection`.
/// When the line-case direction degenerates, `on_line_degenerate` controls
/// the fallback: `true` returns early as intersecting, `false` picks a
/// perpendicular direction and keeps going (needed for EPA).
fn gjk_core(
    a: &dyn ConvexSupport,
    b: &dyn ConvexSupport,
    early_intersect_on_line: bool,
) -> GjkResult {
    let mut direction = glam::Vec2::new(1.0, 0.0);
    let mut simplex = [glam::Vec2::ZERO; 3];
    let mut simplex_len: usize = 0;

    let s = minkowski_support(a, b, direction);
    simplex[simplex_len] = s;
    simplex_len += 1;
    direction = -s;

    for _ in 0..GJK_MAX_ITERATIONS {
        let new_point = minkowski_support(a, b, direction);
        if new_point.dot(direction) < 0.0 {
            return GjkResult::NoIntersection;
        }
        simplex[simplex_len] = new_point;
        simplex_len += 1;

        match simplex_len {
            2 => {
                let b_pt = simplex[1];
                let a_pt = simplex[0];
                let ab = a_pt - b_pt;
                let ao = -b_pt;
                direction = triple_cross_2d(ab, ao, ab);
                if direction.length_squared() < crate::EPSILON_F32 {
                    if early_intersect_on_line {
                        return GjkResult::Intersection(simplex, simplex_len);
                    }
                    // Need third point — pick perpendicular
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
                    simplex[0] = simplex[1];
                    simplex[1] = simplex[2];
                    simplex_len -= 1;
                    direction = cb_perp;
                } else if ca_perp.dot(co) > 0.0 {
                    simplex[1] = simplex[2];
                    simplex_len -= 1;
                    direction = ca_perp;
                } else {
                    return GjkResult::Intersection(simplex, simplex_len);
                }
            }
            _ => return GjkResult::NoIntersection,
        }
    }

    GjkResult::NoIntersection
}

/// GJK collision test between two convex shapes (2D).
///
/// Returns `true` if the shapes overlap. Uses the simplex-based GJK algorithm.
#[must_use]
pub fn gjk_intersect(a: &dyn ConvexSupport, b: &dyn ConvexSupport) -> bool {
    matches!(gjk_core(a, b, true), GjkResult::Intersection(..))
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
#[must_use]
pub fn epa_penetration(
    a: &dyn ConvexSupport,
    b: &dyn ConvexSupport,
    simplex: &[glam::Vec2],
) -> Option<Penetration> {
    if simplex.len() < 3 {
        return None;
    }

    // Pre-allocate for typical EPA expansion.
    let mut polytope: Vec<glam::Vec2> = Vec::with_capacity(32);
    polytope.extend_from_slice(simplex);

    // Ensure CCW winding — EPA normals assume CCW polytope order.
    let signed_area = (polytope[1].x - polytope[0].x) * (polytope[2].y - polytope[0].y)
        - (polytope[2].x - polytope[0].x) * (polytope[1].y - polytope[0].y);
    if signed_area < 0.0 {
        polytope.swap(0, 1);
    }

    for _ in 0..EPA_MAX_ITERATIONS {
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

        if (d - closest_dist).abs() < crate::EPSILON_F32 {
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
#[must_use]
pub fn gjk_epa(a: &dyn ConvexSupport, b: &dyn ConvexSupport) -> Option<Penetration> {
    match gjk_core(a, b, false) {
        GjkResult::NoIntersection => None,
        GjkResult::Intersection(simplex, len) => epa_penetration(a, b, &simplex[..len]),
    }
}

// ---------------------------------------------------------------------------

// Continuous Collision Detection (CCD)
// ---------------------------------------------------------------------------

/// Expand an AABB along a velocity vector to create a swept bounding volume.
#[must_use]
#[inline]
pub fn swept_aabb(aabb: &Aabb, velocity: Vec3, dt: f32) -> Aabb {
    let end_min = aabb.min + velocity * dt;
    let end_max = aabb.max + velocity * dt;
    Aabb::new(aabb.min.min(end_min), aabb.max.max(end_max))
}

/// Compute the time of impact between two moving convex shapes.
///
/// Uses conservative advancement: iteratively advance time by
/// `distance / closing_speed` until contact or timeout.
///
/// - `a`, `b`: convex shapes.
/// - `vel_a`, `vel_b`: linear velocities.
/// - `max_t`: maximum time horizon.
/// - `tol`: distance tolerance for contact.
///
/// Returns `Some(t)` at first contact, or `None` if no collision within `max_t`.
#[must_use]
pub fn time_of_impact(
    a: &dyn ConvexSupport3D,
    b: &dyn ConvexSupport3D,
    vel_a: Vec3,
    vel_b: Vec3,
    max_t: f32,
    tol: f32,
) -> Option<f32> {
    // If already overlapping at t=0
    if gjk_intersect_3d(a, b) {
        return Some(0.0);
    }

    let rel_vel = vel_b - vel_a;
    let speed = rel_vel.length();
    if speed < crate::EPSILON_F32 {
        return None; // Not approaching
    }

    // Conservative advancement
    let mut t = 0.0;
    for _ in 0..GJK_MAX_ITERATIONS {
        // We approximate distance by running GJK and checking overlap
        // at the current time offset. This is a simplified approach —
        // a full implementation would use the GJK distance query.
        if t > max_t {
            return None;
        }

        // Check if shapes overlap at time t by testing with offset supports
        let offset = rel_vel * t;
        let offset_support = OffsetSupport { shape: b, offset };
        if gjk_intersect_3d(a, &offset_support) {
            return Some(t);
        }

        // Advance conservatively: use the closing speed as upper bound
        let step = tol.max(0.01) / speed;
        t += step;
    }

    None
}

/// Helper: a ConvexSupport3D that offsets another shape by a translation.
struct OffsetSupport<'a> {
    shape: &'a dyn ConvexSupport3D,
    offset: Vec3,
}

impl ConvexSupport3D for OffsetSupport<'_> {
    fn support(&self, direction: Vec3) -> Vec3 {
        self.shape.support(direction) + self.offset
    }
}

// ---------------------------------------------------------------------------
// Constraint solvers (physics)
// ---------------------------------------------------------------------------

/// A contact constraint for sequential impulse solving.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContactConstraint {
    /// Contact normal (from A to B).
    pub normal: Vec3,
    /// Contact point (world space).
    pub point: Vec3,
    /// Penetration depth.
    pub penetration: f32,
    /// Coefficient of restitution.
    pub restitution: f32,
    /// Coefficient of friction.
    pub friction: f32,
    /// Inverse mass of body A.
    pub inv_mass_a: f32,
    /// Inverse mass of body B.
    pub inv_mass_b: f32,
}

/// Result of sequential impulse solving: normal and friction impulses per contact.
#[derive(Debug, Clone, PartialEq)]
pub struct ImpulseResult {
    /// Normal impulse magnitudes (one per constraint, always >= 0).
    pub normal: Vec<f32>,
    /// Tangent friction impulse vectors (one per constraint).
    pub friction: Vec<Vec3>,
}

/// Solve contact constraints using sequential impulse iteration.
///
/// Given a set of contact constraints and relative velocities, computes
/// impulses that resolve penetration and apply Coulomb friction.
///
/// Returns [`ImpulseResult`] with normal and friction impulses per constraint.
///
/// This is equivalent to calling [`sequential_impulse_warm`] with
/// `warm_start: None`.
#[must_use]
pub fn sequential_impulse(
    constraints: &[ContactConstraint],
    rel_velocities: &[Vec3],
    iterations: usize,
) -> ImpulseResult {
    sequential_impulse_warm(constraints, rel_velocities, iterations, None, 0.0)
}

/// Solve contact constraints with warm-starting from previous frame's impulses.
///
/// `warm_start` is the previous frame's [`ImpulseResult`]. If provided, initial
/// impulses are seeded from it (scaled by `warm_factor` ∈ \[0,1\] to damp).
/// This dramatically reduces iteration count for stable stacking.
///
/// A `warm_factor` of `0.8`–`0.95` is typical for physics at 60 Hz. Pass
/// `warm_start: None` (or use [`sequential_impulse`]) to start from rest.
#[must_use]
pub fn sequential_impulse_warm(
    constraints: &[ContactConstraint],
    rel_velocities: &[Vec3],
    iterations: usize,
    warm_start: Option<&ImpulseResult>,
    warm_factor: f32,
) -> ImpulseResult {
    let n = constraints.len();

    // Seed initial impulses from the previous frame, scaled by warm_factor.
    let warm_factor = warm_factor.clamp(0.0, 1.0);
    let mut normal_impulses: Vec<f32> = if let Some(ws) = warm_start {
        ws.normal
            .iter()
            .take(n)
            .map(|&j| (j * warm_factor).max(0.0))
            .chain(std::iter::repeat(0.0))
            .take(n)
            .collect()
    } else {
        vec![0.0f32; n]
    };
    let mut friction_impulses: Vec<Vec3> = if let Some(ws) = warm_start {
        ws.friction
            .iter()
            .take(n)
            .map(|&f| f * warm_factor)
            .chain(std::iter::repeat(Vec3::ZERO))
            .take(n)
            .collect()
    } else {
        vec![Vec3::ZERO; n]
    };

    for _ in 0..iterations {
        for i in 0..n {
            let c = &constraints[i];
            let inv_mass = c.inv_mass_a + c.inv_mass_b;
            if inv_mass < crate::EPSILON_F32 {
                continue;
            }

            let v_rel = if i < rel_velocities.len() {
                rel_velocities[i]
            } else {
                Vec3::ZERO
            };

            // Normal impulse
            let v_n = v_rel.dot(c.normal);
            let j_n = -(1.0 + c.restitution) * v_n / inv_mass;
            let new_impulse = (normal_impulses[i] + j_n).max(0.0);
            normal_impulses[i] = new_impulse;

            // Tangent friction impulse (Coulomb cone)
            if c.friction > crate::EPSILON_F32 {
                let v_tangent = v_rel - c.normal * v_n;
                let tangent_speed = v_tangent.length();
                if tangent_speed > crate::EPSILON_F32 {
                    let tangent_dir = v_tangent / tangent_speed;
                    let j_t = -tangent_speed / inv_mass;
                    // Clamp to friction cone: |j_t| <= mu * j_n
                    let max_friction = c.friction * new_impulse;
                    let clamped = j_t.clamp(-max_friction, max_friction);
                    friction_impulses[i] = tangent_dir * clamped;
                }
            }
        }
    }

    ImpulseResult {
        normal: normal_impulses,
        friction: friction_impulses,
    }
}

// ---------------------------------------------------------------------------

// 3D GJK / EPA collision detection
// ---------------------------------------------------------------------------

/// A convex 3D shape that can compute a support point in a given direction.
pub trait ConvexSupport3D {
    /// Return the point on the shape farthest in `direction`.
    fn support(&self, direction: Vec3) -> Vec3;
}

impl ConvexSupport3D for Obb {
    #[inline]
    fn support(&self, direction: Vec3) -> Vec3 {
        let axes = self.axes();
        let he = self.half_extents.to_array();
        let mut result = self.center;
        for i in 0..3 {
            let sign = if axes[i].dot(direction) >= 0.0 {
                1.0
            } else {
                -1.0
            };
            result += axes[i] * (sign * he[i]);
        }
        result
    }
}

impl ConvexSupport3D for Capsule {
    #[inline]
    fn support(&self, direction: Vec3) -> Vec3 {
        let len = direction.length();
        let dir_norm = if len > crate::EPSILON_F32 {
            direction / len
        } else {
            Vec3::X
        };
        // Farthest endpoint in direction, plus radius offset
        let dot_start = self.start.dot(direction);
        let dot_end = self.end.dot(direction);
        let base = if dot_start >= dot_end {
            self.start
        } else {
            self.end
        };
        base + dir_norm * self.radius
    }
}

/// A convex polyhedron for 3D GJK/EPA.
#[derive(Debug, Clone)]
#[must_use]
pub struct ConvexHull3D {
    /// Vertices of the convex hull.
    pub vertices: Vec<Vec3>,
}

impl ConvexHull3D {
    /// Create a convex hull from vertices.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HisabError::InvalidInput`] if `vertices` is empty.
    pub fn new(vertices: Vec<Vec3>) -> Result<Self, crate::HisabError> {
        if vertices.is_empty() {
            return Err(crate::HisabError::InvalidInput(
                "convex hull requires at least one vertex".into(),
            ));
        }
        Ok(Self { vertices })
    }
}

impl ConvexSupport3D for ConvexHull3D {
    #[inline]
    fn support(&self, direction: Vec3) -> Vec3 {
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

impl ConvexSupport3D for Sphere {
    #[inline]
    fn support(&self, direction: Vec3) -> Vec3 {
        let len = direction.length();
        if len < crate::EPSILON_F32 {
            return self.center + Vec3::new(self.radius, 0.0, 0.0);
        }
        self.center + direction * (self.radius / len)
    }
}

/// Minkowski difference support for 3D shapes.
#[inline]
fn minkowski_support_3d(a: &dyn ConvexSupport3D, b: &dyn ConvexSupport3D, direction: Vec3) -> Vec3 {
    a.support(direction) - b.support(-direction)
}

/// Penetration result from 3D EPA.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Penetration3D {
    /// Penetration normal (direction to separate).
    pub normal: Vec3,
    /// Penetration depth.
    pub depth: f32,
}

/// 3D GJK collision test.
///
/// Returns `true` if the two convex shapes overlap.
#[must_use]
pub fn gjk_intersect_3d(a: &dyn ConvexSupport3D, b: &dyn ConvexSupport3D) -> bool {
    gjk_core_3d(a, b).is_some()
}

/// Combined 3D GJK + EPA.
///
/// Returns `None` if no overlap, or `Some(Penetration3D)` with separation info.
#[must_use]
pub fn gjk_epa_3d(a: &dyn ConvexSupport3D, b: &dyn ConvexSupport3D) -> Option<Penetration3D> {
    let simplex = gjk_core_3d(a, b)?;
    epa_penetration_3d(a, b, &simplex)
}

/// Core 3D GJK — returns the enclosing simplex (up to 4 points) if origin is contained.
fn gjk_core_3d(a: &dyn ConvexSupport3D, b: &dyn ConvexSupport3D) -> Option<Vec<Vec3>> {
    let mut direction = Vec3::X;
    let mut simplex: Vec<Vec3> = Vec::with_capacity(4);

    let s = minkowski_support_3d(a, b, direction);
    simplex.push(s);
    direction = -s;

    for _ in 0..GJK_MAX_ITERATIONS {
        if direction.length_squared() < crate::EPSILON_F32 {
            // Direction degenerated — pick perpendicular to existing simplex
            if simplex.len() >= 2 {
                let edge = simplex[simplex.len() - 1] - simplex[0];
                direction = if edge.x.abs() < 0.9 {
                    edge.cross(Vec3::X)
                } else {
                    edge.cross(Vec3::Y)
                };
            } else {
                direction = Vec3::Y;
            }
            if direction.length_squared() < crate::EPSILON_F32 {
                direction = Vec3::Z;
            }
        }
        let new_point = minkowski_support_3d(a, b, direction);
        if new_point.dot(direction) < 0.0 {
            return None;
        }
        simplex.push(new_point);

        if do_simplex_3d(&mut simplex, &mut direction) {
            return Some(simplex);
        }
    }
    None
}

/// Evolve the 3D simplex. Returns true if origin is enclosed.
fn do_simplex_3d(simplex: &mut Vec<Vec3>, direction: &mut Vec3) -> bool {
    match simplex.len() {
        2 => {
            // Line case
            let a = simplex[1];
            let b = simplex[0];
            let ab = b - a;
            let ao = -a;
            if ab.dot(ao) > 0.0 {
                *direction = ab.cross(ao).cross(ab);
            } else {
                simplex.clear();
                simplex.push(a);
                *direction = ao;
            }
            false
        }
        3 => {
            // Triangle case
            let a = simplex[2];
            let b = simplex[1];
            let c = simplex[0];
            let ab = b - a;
            let ac = c - a;
            let ao = -a;
            let abc = ab.cross(ac);

            if abc.cross(ac).dot(ao) > 0.0 {
                if ac.dot(ao) > 0.0 {
                    simplex.clear();
                    simplex.push(c);
                    simplex.push(a);
                    *direction = ac.cross(ao).cross(ac);
                } else {
                    simplex.clear();
                    simplex.push(b);
                    simplex.push(a);
                    return do_simplex_3d(simplex, direction); // line case
                }
            } else if ab.cross(abc).dot(ao) > 0.0 {
                simplex.clear();
                simplex.push(b);
                simplex.push(a);
                return do_simplex_3d(simplex, direction); // line case
            } else if abc.dot(ao) > 0.0 {
                *direction = abc;
            } else {
                // Below triangle
                simplex.swap(0, 1);
                *direction = -abc;
            }
            false
        }
        4 => {
            // Tetrahedron case — a is the newest point (last added)
            let a = simplex[3];
            let b = simplex[2];
            let c = simplex[1];
            let d = simplex[0];
            let ab = b - a;
            let ac = c - a;
            let ad = d - a;
            let ao = -a;

            // Compute face normals, ensuring they point outward (away from the
            // opposite vertex) by flipping if needed.
            let mut abc = ab.cross(ac);
            if abc.dot(ad) > 0.0 {
                abc = -abc;
            }
            let mut acd = ac.cross(ad);
            if acd.dot(ab) > 0.0 {
                acd = -acd;
            }
            let mut adb = ad.cross(ab);
            if adb.dot(ac) > 0.0 {
                adb = -adb;
            }

            if abc.dot(ao) > 0.0 {
                simplex.clear();
                simplex.push(c);
                simplex.push(b);
                simplex.push(a);
                return do_simplex_3d(simplex, direction); // triangle
            }
            if acd.dot(ao) > 0.0 {
                simplex.clear();
                simplex.push(d);
                simplex.push(c);
                simplex.push(a);
                return do_simplex_3d(simplex, direction);
            }
            if adb.dot(ao) > 0.0 {
                simplex.clear();
                simplex.push(b);
                simplex.push(d);
                simplex.push(a);
                return do_simplex_3d(simplex, direction);
            }
            // Origin is inside the tetrahedron
            true
        }
        _ => false,
    }
}

/// 3D EPA: compute penetration depth from the GJK simplex.
fn epa_penetration_3d(
    a: &dyn ConvexSupport3D,
    b: &dyn ConvexSupport3D,
    simplex: &[Vec3],
) -> Option<Penetration3D> {
    if simplex.is_empty() {
        return None;
    }

    // Expand simplex to a tetrahedron if needed
    let mut vertices: Vec<Vec3> = simplex.to_vec();
    // Try to ensure 4 non-coplanar points by adding support in cardinal directions
    let dirs = [
        Vec3::X,
        Vec3::Y,
        Vec3::Z,
        Vec3::NEG_X,
        Vec3::NEG_Y,
        Vec3::NEG_Z,
    ];
    for dir in &dirs {
        if vertices.len() >= 4 {
            break;
        }
        let p = minkowski_support_3d(a, b, *dir);
        let is_dup = vertices
            .iter()
            .any(|v| (*v - p).length_squared() < crate::EPSILON_F32);
        if !is_dup {
            vertices.push(p);
        }
    }
    if vertices.len() < 4 {
        return None; // Truly degenerate
    }

    // Check tetrahedron volume — if flat, try perturbing
    let vol = (vertices[1] - vertices[0])
        .dot((vertices[2] - vertices[0]).cross(vertices[3] - vertices[0]))
        .abs();
    if vol < 1e-10 {
        return None;
    }

    // Build initial polytope as triangular faces of the tetrahedron.
    // Ensure each face normal points away from the centroid (outward).
    let centroid = (vertices[0] + vertices[1] + vertices[2] + vertices[3]) * 0.25;
    let mut faces: Vec<[usize; 3]> = vec![[0, 1, 2], [0, 3, 1], [0, 2, 3], [1, 3, 2]];
    // Fix winding so normals point outward from centroid
    for face in &mut faces {
        let va = vertices[face[0]];
        let vb = vertices[face[1]];
        let vc = vertices[face[2]];
        let normal = (vb - va).cross(vc - va);
        if normal.dot(va - centroid) < 0.0 {
            face.swap(1, 2); // Reverse winding
        }
    }

    for _ in 0..EPA_MAX_ITERATIONS {
        // Find closest face to origin
        let mut closest_dist = f32::INFINITY;
        let mut closest_normal = Vec3::ZERO;

        for face in &faces {
            let va = vertices[face[0]];
            let vb = vertices[face[1]];
            let vc = vertices[face[2]];
            let cross = (vb - va).cross(vc - va);
            let len = cross.length();
            if len < crate::EPSILON_F32 {
                continue; // Skip degenerate faces
            }
            let normal = cross / len;
            let dist = normal.dot(va);

            // Ensure normal points away from origin
            let (normal, dist) = if dist < 0.0 {
                (-normal, -dist)
            } else {
                (normal, dist)
            };

            if dist < closest_dist {
                closest_dist = dist;
                closest_normal = normal;
            }
        }

        if closest_dist.is_infinite() {
            return None; // All faces degenerate this iteration
        }

        let support = minkowski_support_3d(a, b, closest_normal);
        let d = support.dot(closest_normal);

        if (d - closest_dist).abs() < crate::EPSILON_F32 {
            return Some(Penetration3D {
                normal: closest_normal,
                depth: closest_dist.abs(),
            });
        }

        // Add support point and rebuild faces visible from it
        let new_idx = vertices.len();
        vertices.push(support);

        // Find faces visible from the new point
        let mut edges: Vec<[usize; 2]> = Vec::new();
        let mut keep: Vec<bool> = vec![true; faces.len()];

        for (fi, face) in faces.iter().enumerate() {
            let va = vertices[face[0]];
            let vb = vertices[face[1]];
            let vc = vertices[face[2]];
            let normal = (vb - va).cross(vc - va);
            if normal.dot(support - va) > 0.0 {
                keep[fi] = false;
                // Collect edges
                for e in 0..3 {
                    let edge = [face[e], face[(e + 1) % 3]];
                    // Check if this edge is shared with another visible face
                    let reversed = [edge[1], edge[0]];
                    if let Some(pos) = edges.iter().position(|e| *e == reversed) {
                        edges.remove(pos);
                    } else {
                        edges.push(edge);
                    }
                }
            }
        }

        // Remove visible faces
        let mut new_faces: Vec<[usize; 3]> = Vec::new();
        for (fi, face) in faces.iter().enumerate() {
            if keep[fi] {
                new_faces.push(*face);
            }
        }

        // Create new faces from horizon edges to the new point
        for edge in &edges {
            new_faces.push([edge[0], edge[1], new_idx]);
        }

        faces = new_faces;
    }

    // Return best estimate
    let mut closest_dist = f32::INFINITY;
    let mut closest_normal = Vec3::ZERO;
    for face in &faces {
        let va = vertices[face[0]];
        let vb = vertices[face[1]];
        let vc = vertices[face[2]];
        let cross = (vb - va).cross(vc - va);
        let len = cross.length();
        if len < crate::EPSILON_F32 {
            continue;
        }
        let normal = cross / len;
        let dist = normal.dot(va).abs();
        if dist < closest_dist {
            closest_dist = dist;
            closest_normal = if normal.dot(va) >= 0.0 {
                normal
            } else {
                -normal
            };
        }
    }
    if closest_dist.is_infinite() {
        return None; // All faces degenerate
    }
    Some(Penetration3D {
        normal: closest_normal,
        depth: closest_dist,
    })
}

// ---------------------------------------------------------------------------
// MPR (Minkowski Portal Refinement) / XenoCollide
// ---------------------------------------------------------------------------

/// MPR (Minkowski Portal Refinement) collision test for 3D convex shapes.
///
/// An alternative to GJK that finds a "portal" — a triangle on the Minkowski
/// difference surface that separates the origin. Simpler to implement than GJK
/// and often faster for overlap-only queries.
///
/// Returns `true` if the shapes overlap.
#[must_use]
pub fn mpr_intersect(a: &dyn ConvexSupport3D, b: &dyn ConvexSupport3D) -> bool {
    mpr_penetration(a, b).is_some()
}

/// MPR collision with penetration info.
///
/// Returns `None` if shapes don't overlap, or `Some(Penetration3D)` with
/// the separation normal and approximate depth.
#[must_use]
pub fn mpr_penetration(a: &dyn ConvexSupport3D, b: &dyn ConvexSupport3D) -> Option<Penetration3D> {
    // Phase 1: Find the portal (origin ray)
    // v0 = interior point of Minkowski difference (center of A - center of B approximation)
    let v0 =
        minkowski_support_3d(a, b, Vec3::X) * 0.5 + minkowski_support_3d(a, b, Vec3::NEG_X) * 0.5;

    if v0.length_squared() < crate::EPSILON_F32 {
        // Centers coincide — shapes definitely overlap
        // Use a support direction to get penetration info
        let n = Vec3::X;
        let s = minkowski_support_3d(a, b, n);
        return Some(Penetration3D {
            normal: n,
            depth: s.dot(n).abs(),
        });
    }

    // v1 = support in direction from origin toward v0
    let dir1 = -v0.normalize_or_zero();
    let v1 = minkowski_support_3d(a, b, dir1);

    // If v1 doesn't cross the origin ray, no intersection
    if v1.dot(dir1) < 0.0 {
        return None;
    }

    // v2 = support perpendicular to the v0-v1 line
    let dir2 = (v1 - v0).cross(-v0);
    if dir2.length_squared() < crate::EPSILON_F32 {
        // v0 and v1 are on the same line through origin — overlap
        let n = dir1;
        return Some(Penetration3D {
            normal: n,
            depth: v1.dot(n).abs(),
        });
    }
    let dir2 = dir2.normalize();
    let v2 = minkowski_support_3d(a, b, dir2);

    if v2.dot(dir2) < 0.0 {
        return None;
    }

    // v3 = support in the direction of the portal normal
    let mut portal = [v1, v2, Vec3::ZERO];
    let portal_normal = (v2 - v1).cross(v0 - v1);
    let dir3 = if portal_normal.dot(-v0) > 0.0 {
        portal_normal.normalize()
    } else {
        // Flip winding
        portal.swap(0, 1);
        -portal_normal.normalize()
    };
    let v3 = minkowski_support_3d(a, b, dir3);

    if v3.dot(dir3) < 0.0 {
        return None;
    }
    portal[2] = v3;

    // Phase 2: Portal refinement — refine until the portal contains the
    // origin ray or we find the closest feature
    for _ in 0..GJK_MAX_ITERATIONS {
        let normal = (portal[1] - portal[0]).cross(portal[2] - portal[0]);
        let len = normal.length();
        if len < crate::EPSILON_F32 {
            break;
        }
        let n = normal / len;

        // Check if origin is on the correct side of the portal
        let dist = n.dot(portal[0]);
        if dist < 0.0 {
            // Origin is behind the portal — no intersection
            return None;
        }

        // Find new support point beyond the portal
        let v_new = minkowski_support_3d(a, b, n);
        let new_dist = v_new.dot(n);

        // Convergence check
        if (new_dist - dist).abs() < crate::EPSILON_F32 {
            return Some(Penetration3D {
                normal: n,
                depth: dist,
            });
        }

        // Determine which edge of the portal to replace
        // Replace the vertex whose removal keeps the origin on the same side
        let c0 = (portal[1] - v_new).cross(portal[0] - v_new);
        let c1 = (portal[2] - v_new).cross(portal[1] - v_new);

        if c0.dot(-v0) > 0.0 {
            portal[2] = v_new;
        } else if c1.dot(-v0) > 0.0 {
            portal[0] = v_new;
        } else {
            portal[1] = v_new;
        }
    }

    // Return best estimate from final portal
    let normal = (portal[1] - portal[0]).cross(portal[2] - portal[0]);
    let len = normal.length();
    if len < crate::EPSILON_F32 {
        return None;
    }
    let n = normal / len;
    let depth = n.dot(portal[0]).abs();

    Some(Penetration3D { normal: n, depth })
}

// ---------------------------------------------------------------------------
// Point-in-convex-polygon (2D)
// ---------------------------------------------------------------------------

/// Test whether a 2D point lies inside (or on the boundary of) a convex polygon.
///
/// Uses cross-product winding: for each directed edge of the polygon, the point
/// must lie on the same side (consistent sign of the 2D cross product). The
/// polygon vertices must be in counter-clockwise order (as produced by
/// [`convex_hull_2d`]).
///
/// Returns `false` for polygons with fewer than 3 vertices.
#[must_use]
#[inline]
pub fn point_in_convex_polygon(point: glam::Vec2, polygon: &ConvexPolygon) -> bool {
    let verts = &polygon.vertices;
    let n = verts.len();
    if n < 3 {
        return false;
    }
    // For a CCW polygon, the point is inside iff the 2D cross product of each
    // edge vector with the vector from the edge start to the point is >= 0.
    for i in 0..n {
        let a = verts[i];
        let b = verts[(i + 1) % n];
        let edge = b - a;
        let to_point = point - a;
        // 2D cross product: edge.x * to_point.y - edge.y * to_point.x
        let cross = edge.x * to_point.y - edge.y * to_point.x;
        if cross < 0.0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    // --- point_in_convex_polygon tests --------------------------------------

    fn square_polygon() -> ConvexPolygon {
        // Unit square CCW: (0,0), (1,0), (1,1), (0,1)
        ConvexPolygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ])
        .unwrap()
    }

    fn triangle_polygon() -> ConvexPolygon {
        // CCW triangle
        ConvexPolygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(1.0, 2.0),
        ])
        .unwrap()
    }

    #[test]
    fn point_inside_square() {
        let poly = square_polygon();
        assert!(point_in_convex_polygon(Vec2::new(0.5, 0.5), &poly));
    }

    #[test]
    fn point_outside_square() {
        let poly = square_polygon();
        assert!(!point_in_convex_polygon(Vec2::new(1.5, 0.5), &poly));
        assert!(!point_in_convex_polygon(Vec2::new(-0.1, 0.5), &poly));
    }

    #[test]
    fn point_on_edge_square() {
        let poly = square_polygon();
        // On the bottom edge
        assert!(point_in_convex_polygon(Vec2::new(0.5, 0.0), &poly));
    }

    #[test]
    fn point_inside_triangle() {
        let poly = triangle_polygon();
        assert!(point_in_convex_polygon(Vec2::new(1.0, 0.5), &poly));
    }

    #[test]
    fn point_outside_triangle() {
        let poly = triangle_polygon();
        assert!(!point_in_convex_polygon(Vec2::new(0.0, 1.5), &poly));
        assert!(!point_in_convex_polygon(Vec2::new(3.0, 0.0), &poly));
    }

    #[test]
    fn point_at_vertex_triangle() {
        let poly = triangle_polygon();
        assert!(point_in_convex_polygon(Vec2::new(0.0, 0.0), &poly));
    }

    #[test]
    fn degenerate_polygon_returns_false() {
        // Only 2 vertices — not a polygon
        let poly = ConvexPolygon::new(vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0)]).unwrap();
        assert!(!point_in_convex_polygon(Vec2::new(0.5, 0.0), &poly));
    }
}
