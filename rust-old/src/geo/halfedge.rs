//! Half-edge mesh for triangle adjacency queries.
//!
//! A half-edge mesh stores directed half-edges (one per directed edge of each
//! triangle). Each half-edge knows its twin (the half-edge running in the
//! opposite direction along the same undirected edge, or `usize::MAX` for
//! boundary edges), the next half-edge around the same face, and the face it
//! belongs to.

use std::collections::HashMap;

/// A single directed half-edge in a triangle mesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HalfEdge {
    /// The vertex index this half-edge points **to**.
    pub vertex: usize,
    /// The twin half-edge (opposite direction). [`usize::MAX`] for boundary edges.
    pub twin: usize,
    /// The next half-edge in this face (CCW winding).
    pub next: usize,
    /// The face this half-edge belongs to.
    pub face: usize,
}

/// Half-edge mesh for triangle adjacency queries.
///
/// Stores directed half-edges for a triangle mesh. Each undirected edge
/// `(a, b)` is represented by two half-edges: one going `a → b` and one going
/// `b → a`. Boundary edges have `twin == usize::MAX`.
#[derive(Debug, Clone)]
pub struct HalfEdgeMesh {
    /// All half-edges in the mesh.
    pub half_edges: Vec<HalfEdge>,
    /// Per-vertex: index of one outgoing half-edge from that vertex.
    pub vertex_edge: Vec<usize>,
    /// Per-face: index of one half-edge belonging to that face.
    pub face_edge: Vec<usize>,
}

impl HalfEdgeMesh {
    /// Build a half-edge mesh from a triangle list.
    ///
    /// `num_vertices` is the total number of vertices referenced by `triangles`.
    /// `triangles` is a slice of `[a, b, c]` index triples (CCW winding
    /// convention, though winding only affects normal direction — the topology
    /// is correct for either convention).
    ///
    /// # Errors
    /// Returns [`crate::HisabError::InvalidInput`] if any vertex index in
    /// `triangles` is >= `num_vertices`, or if `triangles` is empty.
    #[tracing::instrument(skip(triangles), level = "debug")]
    pub fn from_triangles(
        num_vertices: usize,
        triangles: &[[usize; 3]],
    ) -> Result<Self, crate::HisabError> {
        if triangles.is_empty() {
            return Err(crate::HisabError::InvalidInput(
                "half-edge mesh requires at least one triangle".into(),
            ));
        }

        let num_faces = triangles.len();
        // 3 half-edges per triangle
        let num_he = num_faces * 3;

        let mut half_edges: Vec<HalfEdge> = Vec::with_capacity(num_he);
        let mut vertex_edge: Vec<usize> = vec![usize::MAX; num_vertices];
        let mut face_edge: Vec<usize> = Vec::with_capacity(num_faces);

        // Validate indices
        for tri in triangles {
            for &v in tri {
                if v >= num_vertices {
                    return Err(crate::HisabError::InvalidInput(format!(
                        "vertex index {v} out of range (num_vertices={num_vertices})"
                    )));
                }
            }
        }

        // Allocate half-edges: for face f, half-edges are at indices 3f, 3f+1, 3f+2.
        // HE 3f+0: tri[0] → tri[1]
        // HE 3f+1: tri[1] → tri[2]
        // HE 3f+2: tri[2] → tri[0]
        for (f, tri) in triangles.iter().enumerate() {
            let base = f * 3;
            face_edge.push(base);
            for k in 0..3 {
                let dst = tri[(k + 1) % 3];
                let next_he = base + (k + 1) % 3;
                half_edges.push(HalfEdge {
                    vertex: dst,
                    twin: usize::MAX, // filled in below
                    next: next_he,
                    face: f,
                });
                // Record one outgoing half-edge per vertex (from tri[k])
                let src = tri[k];
                if vertex_edge[src] == usize::MAX {
                    vertex_edge[src] = base + k;
                }
            }
        }

        // Build twin map: directed edge (src → dst) → half-edge index.
        // We need to infer src from the half-edge. src of HE at `base + k` is
        // `triangles[f][k]`.
        let mut edge_map: HashMap<(usize, usize), usize> = HashMap::with_capacity(num_he);
        for (f, tri) in triangles.iter().enumerate() {
            for k in 0..3 {
                let src = tri[k];
                let dst = tri[(k + 1) % 3];
                edge_map.insert((src, dst), f * 3 + k);
            }
        }

        // Assign twins
        for (f, tri) in triangles.iter().enumerate() {
            for k in 0..3 {
                let src = tri[k];
                let dst = tri[(k + 1) % 3];
                let he_idx = f * 3 + k;
                if let Some(&twin_idx) = edge_map.get(&(dst, src)) {
                    half_edges[he_idx].twin = twin_idx;
                }
                // else: boundary edge; twin stays usize::MAX
            }
        }

        Ok(Self {
            half_edges,
            vertex_edge,
            face_edge,
        })
    }

    /// Returns the indices of all faces adjacent to `face` (sharing an edge).
    ///
    /// A face is adjacent if it shares a directed edge with the twin of one of
    /// `face`'s half-edges. Boundary edges (no twin) are skipped.
    #[must_use]
    pub fn adjacent_faces(&self, face: usize) -> Vec<usize> {
        let base = self.face_edge[face];
        let mut result = Vec::with_capacity(3);
        for k in 0..3 {
            let he = &self.half_edges[base + k];
            if he.twin != usize::MAX {
                result.push(self.half_edges[he.twin].face);
            }
        }
        result
    }

    /// Returns the indices of all faces in the one-ring around `vertex`
    /// (the vertex fan).
    ///
    /// Walks the outgoing half-edges around the vertex using the twin/next
    /// chain. Stops when it returns to the start or reaches a boundary.
    #[must_use]
    pub fn vertex_faces(&self, vertex: usize) -> Vec<usize> {
        let start = self.vertex_edge[vertex];
        if start == usize::MAX {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut he_idx = start;
        loop {
            let he = &self.half_edges[he_idx];
            result.push(he.face);

            // Move to the next outgoing half-edge from `vertex`:
            // take `next` twice (to the incoming HE at the vertex), then twin.
            let he_next = &self.half_edges[he.next];
            let he_next2 = &self.half_edges[he_next.next];
            let incoming = he_next2; // points TO `vertex`
            if incoming.twin == usize::MAX {
                break; // boundary — cannot continue
            }
            he_idx = incoming.twin;
            if he_idx == start {
                break; // completed the loop
            }
        }
        result
    }

    /// Returns `true` if `vertex` is on the mesh boundary.
    ///
    /// A vertex is on the boundary if any of its outgoing half-edges has no
    /// twin (i.e., twin == `usize::MAX`).
    #[must_use]
    pub fn is_boundary_vertex(&self, vertex: usize) -> bool {
        let start = self.vertex_edge[vertex];
        if start == usize::MAX {
            return false;
        }

        let mut he_idx = start;
        loop {
            let he = &self.half_edges[he_idx];
            if he.twin == usize::MAX {
                return true;
            }
            let he_next = &self.half_edges[he.next];
            let he_next2 = &self.half_edges[he_next.next];
            let incoming_twin = he_next2.twin;
            if incoming_twin == usize::MAX {
                return true;
            }
            he_idx = incoming_twin;
            if he_idx == start {
                break;
            }
        }
        false
    }

    /// Returns the indices of all boundary half-edges (those with no twin).
    #[must_use]
    pub fn boundary_edges(&self) -> Vec<usize> {
        self.half_edges
            .iter()
            .enumerate()
            .filter_map(|(i, he)| if he.twin == usize::MAX { Some(i) } else { None })
            .collect()
    }
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Two triangles sharing one edge:
    ///   v0---v1
    ///   | \  |
    ///   |  \ |
    ///   v2---v3
    /// tri0: [0, 1, 2], tri1: [1, 3, 2]
    fn two_tri_mesh() -> HalfEdgeMesh {
        HalfEdgeMesh::from_triangles(4, &[[0, 1, 2], [1, 3, 2]]).unwrap()
    }

    /// Single triangle.
    fn single_tri_mesh() -> HalfEdgeMesh {
        HalfEdgeMesh::from_triangles(3, &[[0, 1, 2]]).unwrap()
    }

    #[test]
    fn from_triangles_half_edge_count() {
        let mesh = two_tri_mesh();
        // 2 faces × 3 half-edges = 6
        assert_eq!(mesh.half_edges.len(), 6);
        assert_eq!(mesh.face_edge.len(), 2);
        assert_eq!(mesh.vertex_edge.len(), 4);
    }

    #[test]
    fn twin_assignment_shared_edge() {
        let mesh = two_tri_mesh();
        // Shared edge: tri0 has HE 1→2 (index 1), tri1 has HE 2→1.
        // tri0: HE0(0→1), HE1(1→2), HE2(2→0)
        // tri1: HE3(1→3), HE4(3→2), HE5(2→1)
        // HE1 (1→2) should twin with HE5 (2→1)
        assert_eq!(mesh.half_edges[1].twin, 5, "HE1 twin should be HE5");
        assert_eq!(mesh.half_edges[5].twin, 1, "HE5 twin should be HE1");
    }

    #[test]
    fn boundary_edges_single_triangle() {
        let mesh = single_tri_mesh();
        let boundary = mesh.boundary_edges();
        // A lone triangle has 3 boundary edges
        assert_eq!(boundary.len(), 3);
    }

    #[test]
    fn adjacent_faces_two_triangles() {
        let mesh = two_tri_mesh();
        let adj0 = mesh.adjacent_faces(0);
        let adj1 = mesh.adjacent_faces(1);
        // Each face is adjacent to the other exactly once
        assert!(adj0.contains(&1), "face 0 should be adjacent to face 1");
        assert!(adj1.contains(&0), "face 1 should be adjacent to face 0");
        // Only 1 shared edge → exactly 1 adjacent face each
        assert_eq!(adj0.len(), 1);
        assert_eq!(adj1.len(), 1);
    }

    #[test]
    fn is_boundary_vertex_two_triangles() {
        let mesh = two_tri_mesh();
        // v0 and v3 only appear in one triangle each — boundary
        assert!(mesh.is_boundary_vertex(0));
        assert!(mesh.is_boundary_vertex(3));
        // v1 and v2 are shared but on boundary half-edges in this open mesh
        // (the mesh is not closed, so all vertices are boundary)
        // At minimum, v0 and v3 must be boundary.
    }

    #[test]
    fn vertex_faces_single_triangle() {
        let mesh = single_tri_mesh();
        // Every vertex of a lone triangle is in exactly 1 face
        for v in 0..3 {
            let faces = mesh.vertex_faces(v);
            assert_eq!(faces.len(), 1, "vertex {v} should be in 1 face");
            assert_eq!(faces[0], 0);
        }
    }

    #[test]
    fn invalid_vertex_index_errors() {
        let err = HalfEdgeMesh::from_triangles(3, &[[0, 1, 5]]);
        assert!(
            err.is_err(),
            "out-of-range vertex index should return error"
        );
    }

    #[test]
    fn empty_triangles_errors() {
        let err = HalfEdgeMesh::from_triangles(3, &[]);
        assert!(err.is_err(), "empty triangle list should return error");
    }

    /// A closed tetrahedron: 4 vertices, 4 faces — no boundary edges.
    #[test]
    fn tetrahedron_no_boundary() {
        // Vertices: 0,1,2,3
        // Faces (CCW from outside):
        let tris = [[0, 1, 2], [0, 2, 3], [0, 3, 1], [1, 3, 2]];
        let mesh = HalfEdgeMesh::from_triangles(4, &tris).unwrap();
        let boundary = mesh.boundary_edges();
        assert!(
            boundary.is_empty(),
            "closed tetrahedron should have no boundary edges, got {boundary:?}"
        );
    }
}
