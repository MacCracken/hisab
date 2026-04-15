use super::*;

// ---------------------------------------------------------------------------

/// A dual quaternion for rigid body transforms (rotation + translation).
///
/// Avoids the "candy wrapper" artifact of linear blend skinning.
/// Stored as `real + dual * ε` where both are quaternions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DualQuat {
    /// Real part (rotation).
    pub real: Quat,
    /// Dual part (encodes translation).
    pub dual: Quat,
}

impl DualQuat {
    /// Identity dual quaternion (no rotation, no translation).
    pub const IDENTITY: Self = Self {
        real: Quat::IDENTITY,
        dual: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
    };

    /// Create from rotation and translation.
    #[must_use]
    #[inline]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        let t = Quat::from_xyzw(
            translation.x * 0.5,
            translation.y * 0.5,
            translation.z * 0.5,
            0.0,
        );
        Self {
            real: rotation,
            dual: t * rotation,
        }
    }

    /// Extract translation.
    #[must_use]
    #[inline]
    pub fn translation(&self) -> Vec3 {
        let t = self.dual * self.real.conjugate();
        Vec3::new(t.x * 2.0, t.y * 2.0, t.z * 2.0)
    }

    /// Extract rotation.
    #[must_use]
    #[inline]
    pub fn rotation(&self) -> Quat {
        self.real
    }

    /// Convert to a 4×4 matrix.
    #[must_use]
    #[inline]
    pub fn to_matrix(&self) -> Mat4 {
        let t = self.translation();
        Mat4::from_rotation_translation(self.real, t)
    }

    /// Transform a point.
    #[must_use]
    #[inline]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.real * point + self.translation()
    }

    /// Normalize the dual quaternion.
    #[must_use]
    #[inline]
    pub fn normalize(self) -> Self {
        let norm = self.real.length();
        if norm < crate::EPSILON_F32 {
            return Self::IDENTITY;
        }
        let inv = 1.0 / norm;
        Self {
            real: self.real * inv,
            dual: self.dual * inv,
        }
    }

    /// Dual quaternion linear blend (DLB) — blend two transforms.
    #[must_use]
    #[inline]
    pub fn blend(a: &DualQuat, b: &DualQuat, t: f32) -> DualQuat {
        // Ensure shortest path
        let dot = a.real.dot(b.real);
        let sign = if dot < 0.0 { -1.0 } else { 1.0 };
        let u = 1.0 - t;
        DualQuat {
            real: Quat::from_xyzw(
                u * a.real.x + t * sign * b.real.x,
                u * a.real.y + t * sign * b.real.y,
                u * a.real.z + t * sign * b.real.z,
                u * a.real.w + t * sign * b.real.w,
            ),
            dual: Quat::from_xyzw(
                u * a.dual.x + t * sign * b.dual.x,
                u * a.dual.y + t * sign * b.dual.y,
                u * a.dual.z + t * sign * b.dual.z,
                u * a.dual.w + t * sign * b.dual.w,
            ),
        }
        .normalize()
    }
}

// ---------------------------------------------------------------------------
// CSS transform decomposition
