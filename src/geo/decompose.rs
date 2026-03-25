use super::*;

// Convex decomposition
// ---------------------------------------------------------------------------

/// A triangle mesh for convex decomposition.
#[derive(Debug, Clone)]
pub struct TriMesh {
    /// Vertex positions.
    pub vertices: Vec<Vec3>,
    /// Triangle indices (each triple references vertices).
    pub indices: Vec<[usize; 3]>,
}

/// Result of approximate convex decomposition.
#[derive(Debug, Clone)]
pub struct ConvexDecomposition {
    /// Convex parts, each as a set of vertex indices into the original mesh.
    pub parts: Vec<Vec<usize>>,
}

/// Configuration for approximate convex decomposition.
pub struct AcdConfig {
    /// Maximum concavity threshold. Parts with concavity below this are kept.
    pub max_concavity: f32,
    /// Maximum number of output parts.
    pub max_parts: usize,
}

impl Default for AcdConfig {
    fn default() -> Self {
        Self {
            max_concavity: 0.05,
            max_parts: 32,
        }
    }
}

/// Approximate convex decomposition of a triangle mesh.
///
/// Hierarchically splits the mesh along PCA-derived cutting planes
/// until all parts are within the concavity threshold.
///
/// # Errors
///
/// Returns [`crate::HisabError::InvalidInput`] if the mesh has no triangles.
pub fn convex_decompose(
    mesh: &TriMesh,
    config: &AcdConfig,
) -> Result<ConvexDecomposition, crate::HisabError> {
    if mesh.indices.is_empty() {
        return Err(crate::HisabError::InvalidInput(
            "mesh has no triangles".into(),
        ));
    }

    // Start with all vertex indices as one part
    let all_verts: Vec<usize> = (0..mesh.vertices.len()).collect();
    let mut parts: Vec<Vec<usize>> = vec![all_verts];

    // Iteratively split the most concave part
    for _ in 0..config.max_parts {
        if parts.len() >= config.max_parts {
            break;
        }

        // Find the part with highest concavity
        let mut worst_idx = 0;
        let mut worst_concavity = 0.0f32;
        for (i, part) in parts.iter().enumerate() {
            let c = compute_concavity(mesh, part);
            if c > worst_concavity {
                worst_concavity = c;
                worst_idx = i;
            }
        }

        if worst_concavity <= config.max_concavity {
            break; // All parts are convex enough
        }

        // Split the worst part using PCA
        let part = parts.remove(worst_idx);
        let (left, right) = split_part_pca(mesh, &part);
        if !left.is_empty() {
            parts.push(left);
        }
        if !right.is_empty() {
            parts.push(right);
        }
    }

    Ok(ConvexDecomposition { parts })
}

/// Compute concavity of a set of vertices (max distance from convex hull).
fn compute_concavity(mesh: &TriMesh, part: &[usize]) -> f32 {
    if part.len() < 4 {
        return 0.0;
    }

    // Compute centroid
    let mut centroid = Vec3::ZERO;
    for &idx in part {
        centroid += mesh.vertices[idx];
    }
    centroid /= part.len() as f32;

    // Compute average distance from centroid
    let avg_dist: f32 = part
        .iter()
        .map(|&idx| (mesh.vertices[idx] - centroid).length())
        .sum::<f32>()
        / part.len() as f32;

    // Concavity ≈ variance of distances / avg_dist
    let variance: f32 = part
        .iter()
        .map(|&idx| {
            let d = (mesh.vertices[idx] - centroid).length() - avg_dist;
            d * d
        })
        .sum::<f32>()
        / part.len() as f32;

    variance.sqrt() / avg_dist.max(crate::EPSILON_F32)
}

/// Split a set of vertices along their principal axis.
fn split_part_pca(mesh: &TriMesh, part: &[usize]) -> (Vec<usize>, Vec<usize>) {
    if part.len() < 2 {
        return (part.to_vec(), Vec::new());
    }

    // Centroid
    let mut centroid = Vec3::ZERO;
    for &idx in part {
        centroid += mesh.vertices[idx];
    }
    centroid /= part.len() as f32;

    // Find the axis of greatest variance (simplified PCA: just pick the widest axis)
    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    for &idx in part {
        let v = mesh.vertices[idx];
        min = min.min(v);
        max = max.max(v);
    }
    let extent = max - min;
    let split_axis = if extent.x >= extent.y && extent.x >= extent.z {
        0
    } else if extent.y >= extent.z {
        1
    } else {
        2
    };

    let split_val = centroid.to_array()[split_axis];

    let mut left = Vec::new();
    let mut right = Vec::new();
    for &idx in part {
        if mesh.vertices[idx].to_array()[split_axis] <= split_val {
            left.push(idx);
        } else {
            right.push(idx);
        }
    }

    // Ensure both sides have vertices
    if left.is_empty() || right.is_empty() {
        let mid = part.len() / 2;
        left = part[..mid].to_vec();
        right = part[mid..].to_vec();
    }

    (left, right)
}

// ---------------------------------------------------------------------------
