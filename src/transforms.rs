//! Transforms, projections, and interpolation.
//!
//! Built on [glam](https://docs.rs/glam) and re-exports its fundamental types.
//! Provides 2D/3D affine transforms, orthographic and perspective projections,
//! and linear interpolation helpers.

use serde::{Deserialize, Serialize};

// Re-export glam types for consumers
pub use glam;
pub use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

/// A 2D affine transform (position, rotation in radians, non-uniform scale).
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
    pub fn new(position: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    /// Convert to a 3x3 affine matrix (scale * rotate * translate).
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
    pub fn apply_to_point(&self, point: Vec2) -> Vec3 {
        let m = self.to_matrix();
        m * Vec3::new(point.x, point.y, 1.0)
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
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    /// Convert to a 4x4 affine matrix (scale * rotate * translate).
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    /// Apply this transform to a 3D point.
    pub fn apply_to_point(&self, point: Vec3) -> Vec3 {
        let m = self.to_matrix();
        let v = m * glam::Vec4::new(point.x, point.y, point.z, 1.0);
        Vec3::new(v.x, v.y, v.z)
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Create an orthographic projection matrix (OpenGL convention, right-handed).
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
pub fn projection_perspective(fov_y_radians: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    Mat4::perspective_rh_gl(fov_y_radians, aspect, near, far)
}

/// Linearly interpolate between two f32 values.
#[inline]
pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Linearly interpolate between two Vec3 values.
#[inline]
pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

    const EPSILON: f32 = 1e-5;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    fn vec3_approx_eq(a: Vec3, b: Vec3) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    #[test]
    fn transform2d_identity() {
        let t = Transform2D::IDENTITY;
        let m = t.to_matrix();
        assert_eq!(m, Mat3::IDENTITY);
    }

    #[test]
    fn transform2d_translation() {
        let t = Transform2D::new(Vec2::new(3.0, 4.0), 0.0, Vec2::ONE);
        let result = t.apply_to_point(Vec2::ZERO);
        assert!(approx_eq(result.x, 3.0));
        assert!(approx_eq(result.y, 4.0));
    }

    #[test]
    fn transform2d_rotation_90() {
        let t = Transform2D::new(Vec2::ZERO, FRAC_PI_2, Vec2::ONE);
        let result = t.apply_to_point(Vec2::new(1.0, 0.0));
        assert!(approx_eq(result.x, 0.0));
        assert!(approx_eq(result.y, 1.0));
    }

    #[test]
    fn transform2d_scale() {
        let t = Transform2D::new(Vec2::ZERO, 0.0, Vec2::new(2.0, 3.0));
        let result = t.apply_to_point(Vec2::new(1.0, 1.0));
        assert!(approx_eq(result.x, 2.0));
        assert!(approx_eq(result.y, 3.0));
    }

    #[test]
    fn transform2d_default_is_identity() {
        assert_eq!(Transform2D::default(), Transform2D::IDENTITY);
    }

    #[test]
    fn transform3d_identity() {
        let t = Transform3D::IDENTITY;
        let m = t.to_matrix();
        assert_eq!(m, Mat4::IDENTITY);
    }

    #[test]
    fn transform3d_translation() {
        let t = Transform3D::new(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::ONE);
        let result = t.apply_to_point(Vec3::ZERO);
        assert!(vec3_approx_eq(result, Vec3::new(1.0, 2.0, 3.0)));
    }

    #[test]
    fn transform3d_scale() {
        let t = Transform3D::new(Vec3::ZERO, Quat::IDENTITY, Vec3::new(2.0, 3.0, 4.0));
        let result = t.apply_to_point(Vec3::ONE);
        assert!(vec3_approx_eq(result, Vec3::new(2.0, 3.0, 4.0)));
    }

    #[test]
    fn transform3d_rotation_90_y() {
        let rot = Quat::from_rotation_y(FRAC_PI_2);
        let t = Transform3D::new(Vec3::ZERO, rot, Vec3::ONE);
        let result = t.apply_to_point(Vec3::new(1.0, 0.0, 0.0));
        assert!(vec3_approx_eq(result, Vec3::new(0.0, 0.0, -1.0)));
    }

    #[test]
    fn transform3d_default_is_identity() {
        assert_eq!(Transform3D::default(), Transform3D::IDENTITY);
    }

    #[test]
    fn projection_orthographic_identity_like() {
        let m = projection_orthographic(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
        let p = m * glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
        assert!(approx_eq(p.x, 0.0));
        assert!(approx_eq(p.y, 0.0));
    }

    #[test]
    fn projection_perspective_basic() {
        let m = projection_perspective(FRAC_PI_4, 1.0, 0.1, 100.0);
        let p = m * glam::Vec4::new(0.0, 0.0, -0.1, 1.0);
        let ndc_z = p.z / p.w;
        assert!(approx_eq(ndc_z, -1.0));
    }

    #[test]
    fn lerp_f32_endpoints() {
        assert!(approx_eq(lerp_f32(0.0, 10.0, 0.0), 0.0));
        assert!(approx_eq(lerp_f32(0.0, 10.0, 1.0), 10.0));
        assert!(approx_eq(lerp_f32(0.0, 10.0, 0.5), 5.0));
    }

    #[test]
    fn lerp_vec3_midpoint() {
        let a = Vec3::ZERO;
        let b = Vec3::new(10.0, 20.0, 30.0);
        let mid = lerp_vec3(a, b, 0.5);
        assert!(vec3_approx_eq(mid, Vec3::new(5.0, 10.0, 15.0)));
    }

    #[test]
    fn transform3d_combined() {
        let t = Transform3D::new(
            Vec3::new(10.0, 0.0, 0.0),
            Quat::IDENTITY,
            Vec3::splat(2.0),
        );
        let result = t.apply_to_point(Vec3::ONE);
        assert!(vec3_approx_eq(result, Vec3::new(12.0, 2.0, 2.0)));
    }

    #[test]
    fn transform2d_combined_scale_rotation_translation() {
        let t = Transform2D::new(Vec2::new(5.0, 0.0), FRAC_PI_2, Vec2::splat(2.0));
        let result = t.apply_to_point(Vec2::new(1.0, 0.0));
        assert!(approx_eq(result.x, 5.0));
        assert!(approx_eq(result.y, 2.0));
    }

    #[test]
    fn transform2d_to_matrix_roundtrip() {
        let t = Transform2D::new(Vec2::new(1.0, 2.0), 0.5, Vec2::new(3.0, 4.0));
        let m = t.to_matrix();
        let point = Vec2::new(7.0, -3.0);
        let via_method = t.apply_to_point(point);
        let via_matrix = m * Vec3::new(point.x, point.y, 1.0);
        assert!(approx_eq(via_method.x, via_matrix.x));
        assert!(approx_eq(via_method.y, via_matrix.y));
    }

    #[test]
    fn transform2d_apply_origin() {
        let result = Transform2D::IDENTITY.apply_to_point(Vec2::new(42.0, -7.0));
        assert!(approx_eq(result.x, 42.0));
        assert!(approx_eq(result.y, -7.0));
    }

    #[test]
    fn transform3d_rotation_x() {
        let rot = Quat::from_rotation_x(FRAC_PI_2);
        let t = Transform3D::new(Vec3::ZERO, rot, Vec3::ONE);
        let result = t.apply_to_point(Vec3::new(0.0, 1.0, 0.0));
        assert!(vec3_approx_eq(result, Vec3::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn transform3d_rotation_z() {
        let rot = Quat::from_rotation_z(FRAC_PI_2);
        let t = Transform3D::new(Vec3::ZERO, rot, Vec3::ONE);
        let result = t.apply_to_point(Vec3::new(1.0, 0.0, 0.0));
        assert!(vec3_approx_eq(result, Vec3::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn transform3d_non_uniform_scale() {
        let t = Transform3D::new(Vec3::ZERO, Quat::IDENTITY, Vec3::new(1.0, 2.0, 3.0));
        let result = t.apply_to_point(Vec3::new(1.0, 1.0, 1.0));
        assert!(vec3_approx_eq(result, Vec3::new(1.0, 2.0, 3.0)));
    }

    #[test]
    fn transform3d_to_matrix_identity() {
        let m = Transform3D::IDENTITY.to_matrix();
        assert_eq!(m, Mat4::IDENTITY);
    }

    #[test]
    fn projection_orthographic_maps_corners() {
        let m = projection_orthographic(-10.0, 10.0, -5.0, 5.0, 0.1, 100.0);
        let left = m * glam::Vec4::new(-10.0, 0.0, -0.1, 1.0);
        assert!(approx_eq(left.x, -1.0));
        let right = m * glam::Vec4::new(10.0, 0.0, -0.1, 1.0);
        assert!(approx_eq(right.x, 1.0));
    }

    #[test]
    fn projection_perspective_far_plane() {
        let m = projection_perspective(FRAC_PI_4, 1.0, 0.1, 100.0);
        let p = m * glam::Vec4::new(0.0, 0.0, -100.0, 1.0);
        let ndc_z = p.z / p.w;
        assert!(approx_eq(ndc_z, 1.0));
    }

    #[test]
    fn lerp_f32_extrapolation() {
        assert!(approx_eq(lerp_f32(0.0, 10.0, 2.0), 20.0));
        assert!(approx_eq(lerp_f32(0.0, 10.0, -1.0), -10.0));
    }

    #[test]
    fn lerp_f32_negative_values() {
        assert!(approx_eq(lerp_f32(-10.0, -20.0, 0.5), -15.0));
    }

    #[test]
    fn lerp_vec3_endpoints() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert!(vec3_approx_eq(lerp_vec3(a, b, 0.0), a));
        assert!(vec3_approx_eq(lerp_vec3(a, b, 1.0), b));
    }

    #[test]
    fn error_display_all_variants() {
        use crate::GanitError;
        assert_eq!(
            GanitError::InvalidTransform("bad".to_string()).to_string(),
            "invalid transform: bad"
        );
        assert_eq!(
            GanitError::OutOfRange("too big".to_string()).to_string(),
            "value out of range: too big"
        );
        assert_eq!(
            GanitError::DivisionByZero.to_string(),
            "division by zero"
        );
        assert_eq!(
            GanitError::SingularMatrix.to_string(),
            "singular matrix — cannot invert"
        );
    }

    #[test]
    fn transform2d_serde_roundtrip() {
        let t = Transform2D::new(Vec2::new(1.0, 2.0), 0.5, Vec2::new(3.0, 4.0));
        let json = serde_json::to_string(&t).unwrap();
        let t2: Transform2D = serde_json::from_str(&json).unwrap();
        assert_eq!(t, t2);
    }

    #[test]
    fn transform3d_serde_roundtrip() {
        let t = Transform3D::new(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::ONE);
        let json = serde_json::to_string(&t).unwrap();
        let t2: Transform3D = serde_json::from_str(&json).unwrap();
        assert_eq!(t, t2);
    }

    #[test]
    fn transform2d_negative_scale() {
        let t = Transform2D::new(Vec2::ZERO, 0.0, Vec2::new(-1.0, 1.0));
        let result = t.apply_to_point(Vec2::new(1.0, 0.0));
        assert!(approx_eq(result.x, -1.0));
        assert!(approx_eq(result.y, 0.0));
    }

    #[test]
    fn transform2d_rotation_360() {
        let t = Transform2D::new(Vec2::ZERO, std::f32::consts::TAU, Vec2::ONE);
        let result = t.apply_to_point(Vec2::new(1.0, 0.0));
        assert!(approx_eq(result.x, 1.0));
        assert!(approx_eq(result.y, 0.0));
    }
}
