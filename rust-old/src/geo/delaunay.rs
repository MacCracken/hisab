use glam::Vec2;

// ---------------------------------------------------------------------------
// Delaunay triangulation (Bowyer-Watson)
// ---------------------------------------------------------------------------

/// A triangle represented by indices into a point array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DelaunayTriangle {
    /// Vertex indices (counter-clockwise).
    pub indices: [usize; 3],
}

/// Result of a 2D Delaunay triangulation.
#[derive(Debug, Clone)]
pub struct Triangulation {
    /// Input points.
    pub points: Vec<Vec2>,
    /// Triangles (indices into `points`).
    pub triangles: Vec<DelaunayTriangle>,
}

/// Compute the 2D Delaunay triangulation of a point set using the
/// Bowyer-Watson incremental algorithm.
///
/// Returns `None` if fewer than 3 points or all points are collinear.
#[must_use]
pub fn delaunay_2d(points: &[Vec2]) -> Option<Triangulation> {
    let n = points.len();
    if n < 3 {
        return None;
    }

    // Find bounding box
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for p in points {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }
    let dx = (max_x - min_x).max(1.0);
    let dy = (max_y - min_y).max(1.0);
    let mid_x = (min_x + max_x) * 0.5;
    let mid_y = (min_y + max_y) * 0.5;
    let d_max = dx.max(dy) * 10.0;

    // Super-triangle vertices (indices n, n+1, n+2)
    let st0 = Vec2::new(mid_x - d_max, mid_y - d_max);
    let st1 = Vec2::new(mid_x + d_max, mid_y - d_max);
    let st2 = Vec2::new(mid_x, mid_y + d_max);

    let mut all_points: Vec<Vec2> = points.to_vec();
    all_points.push(st0);
    all_points.push(st1);
    all_points.push(st2);

    let st_a = n;
    let st_b = n + 1;
    let st_c = n + 2;

    let mut triangles: Vec<[usize; 3]> = vec![[st_a, st_b, st_c]];

    // Incrementally insert each point
    for i in 0..n {
        let p = all_points[i];
        let mut bad_triangles = Vec::new();

        // Find all triangles whose circumcircle contains the point
        for (ti, tri) in triangles.iter().enumerate() {
            if in_circumcircle(&all_points, tri, p) {
                bad_triangles.push(ti);
            }
        }

        // Find the boundary polygon (edges of the "hole")
        let mut polygon: Vec<[usize; 2]> = Vec::new();
        for &ti in &bad_triangles {
            let tri = triangles[ti];
            for edge_idx in 0..3 {
                let edge = [tri[edge_idx], tri[(edge_idx + 1) % 3]];
                // Check if this edge is shared with another bad triangle
                let shared = bad_triangles.iter().any(|&other_ti| {
                    other_ti != ti && triangle_has_edge(&triangles[other_ti], edge)
                });
                if !shared {
                    polygon.push(edge);
                }
            }
        }

        // Remove bad triangles (in reverse order to preserve indices)
        let mut sorted_bad = bad_triangles;
        sorted_bad.sort_unstable();
        for &ti in sorted_bad.iter().rev() {
            triangles.swap_remove(ti);
        }

        // Create new triangles from the polygon edges to the new point
        for edge in &polygon {
            triangles.push([edge[0], edge[1], i]);
        }
    }

    // Remove triangles that reference super-triangle vertices
    triangles.retain(|tri| tri[0] < n && tri[1] < n && tri[2] < n);

    if triangles.is_empty() {
        return None;
    }

    // Ensure CCW winding
    for tri in &mut triangles {
        let a = all_points[tri[0]];
        let b = all_points[tri[1]];
        let c = all_points[tri[2]];
        let cross = (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y);
        if cross < 0.0 {
            tri.swap(0, 1);
        }
    }

    Some(Triangulation {
        points: points.to_vec(),
        triangles: triangles
            .into_iter()
            .map(|idx| DelaunayTriangle { indices: idx })
            .collect(),
    })
}

/// Check if point `p` is inside the circumcircle of triangle `tri`.
fn in_circumcircle(points: &[Vec2], tri: &[usize; 3], p: Vec2) -> bool {
    let a = points[tri[0]];
    let b = points[tri[1]];
    let c = points[tri[2]];

    let ax = a.x - p.x;
    let ay = a.y - p.y;
    let bx = b.x - p.x;
    let by = b.y - p.y;
    let cx = c.x - p.x;
    let cy = c.y - p.y;

    let det = ax * (by * (cx * cx + cy * cy) - cy * (bx * bx + by * by))
        - ay * (bx * (cx * cx + cy * cy) - cx * (bx * bx + by * by))
        + (ax * ax + ay * ay) * (bx * cy - by * cx);

    // The sign depends on the winding of the triangle
    let cross = (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y);
    if cross > 0.0 { det > 0.0 } else { det < 0.0 }
}

/// Check if a triangle contains a specific edge (in either direction).
fn triangle_has_edge(tri: &[usize; 3], edge: [usize; 2]) -> bool {
    for i in 0..3 {
        let e = [tri[i], tri[(i + 1) % 3]];
        if (e[0] == edge[0] && e[1] == edge[1]) || (e[0] == edge[1] && e[1] == edge[0]) {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Voronoi diagram (dual of Delaunay)
// ---------------------------------------------------------------------------

/// A Voronoi edge connecting two circumcenters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoronoiEdge {
    /// Start point (circumcenter of one triangle).
    pub start: Vec2,
    /// End point (circumcenter of adjacent triangle).
    pub end: Vec2,
}

/// A Voronoi diagram derived from a Delaunay triangulation.
#[derive(Debug, Clone)]
pub struct VoronoiDiagram {
    /// The site points (input to Delaunay).
    pub sites: Vec<Vec2>,
    /// Voronoi edges (finite edges between circumcenters).
    pub edges: Vec<VoronoiEdge>,
}

/// Compute the 2D Voronoi diagram as the dual of the Delaunay triangulation.
///
/// Returns finite Voronoi edges only (edges to infinity are omitted).
/// Returns `None` if the Delaunay triangulation fails.
#[must_use]
pub fn voronoi_2d(points: &[Vec2]) -> Option<VoronoiDiagram> {
    let triangulation = delaunay_2d(points)?;
    let tris = &triangulation.triangles;
    let pts = &triangulation.points;

    // Compute circumcenters for each triangle
    let circumcenters: Vec<Vec2> = tris
        .iter()
        .map(|t| circumcenter(pts[t.indices[0]], pts[t.indices[1]], pts[t.indices[2]]))
        .collect();

    // Build adjacency: for each pair of triangles sharing an edge,
    // create a Voronoi edge between their circumcenters
    let mut edges = Vec::new();
    for i in 0..tris.len() {
        for j in (i + 1)..tris.len() {
            if triangles_share_edge(&tris[i].indices, &tris[j].indices) {
                edges.push(VoronoiEdge {
                    start: circumcenters[i],
                    end: circumcenters[j],
                });
            }
        }
    }

    Some(VoronoiDiagram {
        sites: points.to_vec(),
        edges,
    })
}

/// Circumcenter of a triangle.
fn circumcenter(a: Vec2, b: Vec2, c: Vec2) -> Vec2 {
    let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));
    if d.abs() < crate::EPSILON_F32 {
        // Degenerate — return centroid
        return (a + b + c) / 3.0;
    }
    let a_sq = a.x * a.x + a.y * a.y;
    let b_sq = b.x * b.x + b.y * b.y;
    let c_sq = c.x * c.x + c.y * c.y;
    let ux = (a_sq * (b.y - c.y) + b_sq * (c.y - a.y) + c_sq * (a.y - b.y)) / d;
    let uy = (a_sq * (c.x - b.x) + b_sq * (a.x - c.x) + c_sq * (b.x - a.x)) / d;
    Vec2::new(ux, uy)
}

/// Check if two triangles share an edge.
fn triangles_share_edge(a: &[usize; 3], b: &[usize; 3]) -> bool {
    let mut shared = 0;
    for &ai in a {
        for &bi in b {
            if ai == bi {
                shared += 1;
            }
        }
    }
    shared >= 2
}
