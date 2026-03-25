//! Transforms, projections, and interpolation.
//!
//! Built on [glam](https://docs.rs/glam) and re-exports its fundamental types.
//! Provides 2D/3D affine transforms, orthographic and perspective projections,
//! and linear interpolation helpers.

use serde::{Deserialize, Serialize};

// Re-export glam types for consumers
pub use glam;
pub use glam::{DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
pub use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

/// A 2D affine transform (position, rotation in radians, non-uniform scale).
///
/// # Examples
///
/// ```
/// use hisab::transforms::{Transform2D, Vec2};
///
/// let t = Transform2D::new(Vec2::new(1.0, 2.0), 0.0, Vec2::ONE);
/// let p = t.apply_to_point(Vec2::ZERO);
/// assert!((p.x - 1.0).abs() < 1e-5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    pub position: Vec2,
    /// Rotation in radians (counter-clockwise).
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform2D {
    /// Identity transform.
    pub const IDENTITY: Self = Self {
        position: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
    };

    /// Create a new 2D transform.
    #[must_use]
    pub fn new(position: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    /// Convert to a 3x3 affine matrix (scale * rotate * translate).
    #[must_use]
    #[inline]
    pub fn to_matrix(&self) -> Mat3 {
        let (sin, cos) = self.rotation.sin_cos();
        // Column-major: each column is a basis vector
        Mat3::from_cols(
            Vec3::new(cos * self.scale.x, sin * self.scale.x, 0.0),
            Vec3::new(-sin * self.scale.y, cos * self.scale.y, 0.0),
            Vec3::new(self.position.x, self.position.y, 1.0),
        )
    }

    /// Apply this transform to a 2D point.
    #[must_use]
    #[inline]
    pub fn apply_to_point(&self, point: Vec2) -> Vec3 {
        let (sin, cos) = self.rotation.sin_cos();
        let x = cos * self.scale.x * point.x - sin * self.scale.y * point.y + self.position.x;
        let y = sin * self.scale.x * point.x + cos * self.scale.y * point.y + self.position.y;
        Vec3::new(x, y, 1.0)
    }

    /// Compute the inverse 3x3 matrix of this transform.
    ///
    /// Use this to undo the transform: `t.inverse_matrix() * point3 ≈ original`.
    #[must_use]
    #[inline]
    pub fn inverse_matrix(&self) -> Mat3 {
        self.to_matrix().inverse()
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// A 3D transform (position, quaternion rotation, non-uniform scale).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform3D {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform3D {
    /// Identity transform.
    pub const IDENTITY: Self = Self {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    /// Create a new 3D transform.
    #[must_use]
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    /// Convert to a 4x4 affine matrix (scale * rotate * translate).
    #[must_use]
    #[inline]
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    /// Apply this transform to a 3D point.
    #[must_use]
    #[inline]
    pub fn apply_to_point(&self, point: Vec3) -> Vec3 {
        self.rotation * (self.scale * point) + self.position
    }

    /// Compute the inverse 4x4 matrix of this transform.
    ///
    /// Use this to undo the transform: `t.inverse_matrix() * point4 ≈ original`.
    /// For non-uniform scale with rotation, the SRT decomposition doesn't
    /// round-trip cleanly, so we provide the inverse as a matrix directly.
    #[must_use]
    #[inline]
    pub fn inverse_matrix(&self) -> Mat4 {
        self.to_matrix().inverse()
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Spherical linear interpolation between two quaternions.
#[must_use]
#[inline]
pub fn slerp(a: Quat, b: Quat, t: f32) -> Quat {
    a.slerp(b, t)
}

/// Interpolate between two 3D transforms.
///
/// Position and scale are linearly interpolated; rotation uses slerp.
#[must_use]
#[inline]
pub fn transform3d_lerp(a: &Transform3D, b: &Transform3D, t: f32) -> Transform3D {
    Transform3D {
        position: lerp_vec3(a.position, b.position, t),
        rotation: a.rotation.slerp(b.rotation, t),
        scale: lerp_vec3(a.scale, b.scale, t),
    }
}

/// Flip the handedness of a matrix by negating the Z column.
///
/// Converts between left-handed and right-handed coordinate systems.
#[must_use]
#[inline]
pub fn flip_handedness_z(mat: Mat4) -> Mat4 {
    let cols = mat.to_cols_array_2d();
    Mat4::from_cols_array_2d(&[
        cols[0],
        cols[1],
        [-cols[2][0], -cols[2][1], -cols[2][2], -cols[2][3]],
        cols[3],
    ])
}

/// Create an orthographic projection matrix (OpenGL convention, right-handed).
#[must_use]
#[inline]
pub fn projection_orthographic(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> Mat4 {
    Mat4::orthographic_rh_gl(left, right, bottom, top, near, far)
}

/// Create a perspective projection matrix.
///
/// `fov_y_radians`: vertical field of view in radians.
/// `aspect`: width / height.
/// `near`, `far`: near and far clipping planes (must be positive).
#[must_use]
#[inline]
pub fn projection_perspective(fov_y_radians: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    Mat4::perspective_rh_gl(fov_y_radians, aspect, near, far)
}

/// Linearly interpolate between two f32 values.
#[must_use]
#[inline]
pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Linearly interpolate between two Vec3 values.
#[must_use]
#[inline]
pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}

// ---------------------------------------------------------------------------
// Quaternion utilities

