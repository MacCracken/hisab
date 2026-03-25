use super::*;

// ---------------------------------------------------------------------------

/// Decomposed transform components (CSS Transforms spec).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecomposedTransform {
    /// Translation (x, y, z).
    pub translation: Vec3,
    /// Rotation as quaternion.
    pub rotation: Quat,
    /// Scale (x, y, z).
    pub scale: Vec3,
}

/// Decompose a 4×4 matrix into translation, rotation, and scale.
///
/// Based on the W3C CSS Transforms decomposition algorithm.
/// Returns `None` if the matrix is degenerate (zero determinant).
#[must_use]
pub fn decompose_mat4(m: Mat4) -> Option<DecomposedTransform> {
    let cols = m.to_cols_array_2d();

    // Extract translation
    let translation = Vec3::new(cols[3][0], cols[3][1], cols[3][2]);

    // Extract upper 3×3
    let mut row0 = Vec3::new(cols[0][0], cols[0][1], cols[0][2]);
    let mut row1 = Vec3::new(cols[1][0], cols[1][1], cols[1][2]);
    let mut row2 = Vec3::new(cols[2][0], cols[2][1], cols[2][2]);

    // Scale
    let sx = row0.length();
    let sy = row1.length();
    let sz = row2.length();

    if sx < crate::EPSILON_F32 || sy < crate::EPSILON_F32 || sz < crate::EPSILON_F32 {
        return None;
    }

    // Normalize
    row0 /= sx;
    row1 /= sy;
    row2 /= sz;

    // Fix handedness
    let det = row0.cross(row1).dot(row2);
    let (scale, row0, row1, row2) = if det < 0.0 {
        (Vec3::new(-sx, sy, sz), -row0, row1, row2)
    } else {
        (Vec3::new(sx, sy, sz), row0, row1, row2)
    };

    // Extract rotation from the normalized 3×3
    let rot_mat = Mat3::from_cols(row0, row1, row2);
    let rotation = Quat::from_mat3(&rot_mat);

    Some(DecomposedTransform {
        translation,
        rotation,
        scale,
    })
}

/// Recompose a 4×4 matrix from decomposed components.
#[must_use]
#[inline]
pub fn recompose_mat4(d: &DecomposedTransform) -> Mat4 {
    Mat4::from_scale_rotation_translation(d.scale, d.rotation, d.translation)
}

// ---------------------------------------------------------------------------
// Color matrix operations

