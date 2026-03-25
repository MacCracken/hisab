use super::*;

// Signed Distance Fields (SDF)
// ---------------------------------------------------------------------------

/// Signed distance from a point to a sphere surface.
/// Negative inside, positive outside.
#[must_use]
#[inline]
pub fn sdf_sphere(point: Vec3, center: Vec3, radius: f32) -> f32 {
    (point - center).length() - radius
}

/// Signed distance from a point to an axis-aligned box surface.
#[must_use]
#[inline]
pub fn sdf_box(point: Vec3, center: Vec3, half_extents: Vec3) -> f32 {
    let d = (point - center).abs() - half_extents;
    let outside = Vec3::new(d.x.max(0.0), d.y.max(0.0), d.z.max(0.0)).length();
    let inside = d.x.max(d.y.max(d.z)).min(0.0);
    outside + inside
}

/// Signed distance from a point to a capsule.
#[must_use]
#[inline]
pub fn sdf_capsule(point: Vec3, a: Vec3, b: Vec3, radius: f32) -> f32 {
    let ab = b - a;
    let ap = point - a;
    let t = ap.dot(ab) / ab.dot(ab);
    let t = t.clamp(0.0, 1.0);
    let closest = a + ab * t;
    (point - closest).length() - radius
}

/// SDF union (minimum of two SDFs).
#[must_use]
#[inline]
pub fn sdf_union(d1: f32, d2: f32) -> f32 {
    d1.min(d2)
}

/// SDF intersection (maximum of two SDFs).
#[must_use]
#[inline]
pub fn sdf_intersection(d1: f32, d2: f32) -> f32 {
    d1.max(d2)
}

/// SDF subtraction (d1 minus d2).
#[must_use]
#[inline]
pub fn sdf_subtraction(d1: f32, d2: f32) -> f32 {
    d1.max(-d2)
}

/// SDF smooth union (blending two shapes).
#[must_use]
#[inline]
pub fn sdf_smooth_union(d1: f32, d2: f32, k: f32) -> f32 {
    let h = (0.5 + 0.5 * (d2 - d1) / k).clamp(0.0, 1.0);
    d2 * (1.0 - h) + d1 * h - k * h * (1.0 - h)
}

// ---------------------------------------------------------------------------
// Polygon triangulation (ear clipping)
// ---------------------------------------------------------------------------

/// Triangulate a simple polygon using the ear-clipping method.
///
/// Takes vertices in order (CCW or CW). Returns a list of triangle index triples.
///
/// # Errors
///
/// Returns [`crate::HisabError::InvalidInput`] if fewer than 3 vertices.
pub fn triangulate_polygon(vertices: &[glam::Vec2]) -> Result<Vec<[usize; 3]>, crate::HisabError> {
    let n = vertices.len();
    if n < 3 {
        return Err(crate::HisabError::InvalidInput(
            "polygon needs at least 3 vertices".into(),
        ));
    }
    if n == 3 {
        return Ok(vec![[0, 1, 2]]);
    }

    let mut indices: Vec<usize> = (0..n).collect();
    let mut triangles = Vec::with_capacity(n - 2);

    // Determine winding
    let area: f32 = (0..n)
        .map(|i| {
            let j = (i + 1) % n;
            vertices[i].x * vertices[j].y - vertices[j].x * vertices[i].y
        })
        .sum();
    let ccw = area > 0.0;

    while indices.len() > 3 {
        let len = indices.len();
        let mut ear_found = false;

        for i in 0..len {
            let prev = indices[(i + len - 1) % len];
            let curr = indices[i];
            let next = indices[(i + 1) % len];

            let a = vertices[prev];
            let b = vertices[curr];
            let c = vertices[next];

            // Check if this is a convex vertex
            let cross = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
            let is_convex = if ccw { cross > 0.0 } else { cross < 0.0 };
            if !is_convex {
                continue;
            }

            // Check no other vertex is inside this triangle
            let mut is_ear = true;
            for &idx in &indices {
                if idx == prev || idx == curr || idx == next {
                    continue;
                }
                if point_in_triangle_2d(vertices[idx], a, b, c) {
                    is_ear = false;
                    break;
                }
            }

            if is_ear {
                triangles.push([prev, curr, next]);
                indices.remove(i);
                ear_found = true;
                break;
            }
        }

        if !ear_found {
            break; // Degenerate polygon
        }
    }

    if indices.len() == 3 {
        triangles.push([indices[0], indices[1], indices[2]]);
    }

    Ok(triangles)
}

/// Check if a 2D point is inside a triangle (using barycentric coordinates).
#[must_use]
#[inline]
fn point_in_triangle_2d(p: glam::Vec2, a: glam::Vec2, b: glam::Vec2, c: glam::Vec2) -> bool {
    let v0 = c - a;
    let v1 = b - a;
    let v2 = p - a;
    let dot00 = v0.dot(v0);
    let dot01 = v0.dot(v1);
    let dot02 = v0.dot(v2);
    let dot11 = v1.dot(v1);
    let dot12 = v1.dot(v2);
    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
    u >= 0.0 && v >= 0.0 && u + v <= 1.0
}

// ---------------------------------------------------------------------------
