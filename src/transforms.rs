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
// ---------------------------------------------------------------------------

/// Euler rotation order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum EulerOrder {
    /// X → Y → Z (roll, pitch, yaw)
    XYZ,
    /// X → Z → Y
    XZY,
    /// Y → X → Z
    YXZ,
    /// Y → Z → X
    YZX,
    /// Z → X → Y
    ZXY,
    /// Z → Y → X
    ZYX,
}

/// Create a quaternion from Euler angles (in radians) with the given rotation order.
#[must_use]
#[inline]
pub fn quat_from_euler(x: f32, y: f32, z: f32, order: EulerOrder) -> Quat {
    match order {
        EulerOrder::XYZ => Quat::from_euler(glam::EulerRot::XYZ, x, y, z),
        EulerOrder::XZY => Quat::from_euler(glam::EulerRot::XZY, x, z, y),
        EulerOrder::YXZ => Quat::from_euler(glam::EulerRot::YXZ, y, x, z),
        EulerOrder::YZX => Quat::from_euler(glam::EulerRot::YZX, y, z, x),
        EulerOrder::ZXY => Quat::from_euler(glam::EulerRot::ZXY, z, x, y),
        EulerOrder::ZYX => Quat::from_euler(glam::EulerRot::ZYX, z, y, x),
    }
}

/// Extract Euler angles (in radians) from a quaternion with the given rotation order.
///
/// Returns `(x, y, z)` angles. Subject to gimbal lock at ±90° on the middle axis.
#[must_use]
#[inline]
pub fn quat_to_euler(q: Quat, order: EulerOrder) -> (f32, f32, f32) {
    match order {
        EulerOrder::XYZ => {
            let (x, y, z) = q.to_euler(glam::EulerRot::XYZ);
            (x, y, z)
        }
        EulerOrder::XZY => {
            let (x, z, y) = q.to_euler(glam::EulerRot::XZY);
            (x, y, z)
        }
        EulerOrder::YXZ => {
            let (y, x, z) = q.to_euler(glam::EulerRot::YXZ);
            (x, y, z)
        }
        EulerOrder::YZX => {
            let (y, z, x) = q.to_euler(glam::EulerRot::YZX);
            (x, y, z)
        }
        EulerOrder::ZXY => {
            let (z, x, y) = q.to_euler(glam::EulerRot::ZXY);
            (x, y, z)
        }
        EulerOrder::ZYX => {
            let (z, y, x) = q.to_euler(glam::EulerRot::ZYX);
            (x, y, z)
        }
    }
}

/// Construct a quaternion that rotates `forward` to look at the given direction.
///
/// `forward` is the desired look direction (will be normalized).
/// `up` is the world up vector (typically `Vec3::Y`).
#[must_use]
#[inline]
pub fn quat_look_at(forward: Vec3, up: Vec3) -> Quat {
    let f = forward.normalize();
    let r = up.cross(f).normalize();
    let u = f.cross(r);
    Quat::from_mat3(&Mat3::from_cols(r, u, f))
}

/// Construct a right-handed view matrix (camera look-at).
#[must_use]
#[inline]
pub fn look_at_rh(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    Mat4::look_at_rh(eye, target, up)
}

// ---------------------------------------------------------------------------
// Screen-space projection
// ---------------------------------------------------------------------------

/// Project a 3D world point to 2D screen coordinates.
///
/// `mvp` is the combined model-view-projection matrix.
/// Returns `(screen_x, screen_y, depth)` where depth is in [0, 1] (near to far).
#[must_use]
#[inline]
pub fn world_to_screen(
    point: Vec3,
    mvp: Mat4,
    viewport_width: f32,
    viewport_height: f32,
) -> (f32, f32, f32) {
    let clip = mvp * glam::Vec4::new(point.x, point.y, point.z, 1.0);
    let ndc = Vec3::new(clip.x / clip.w, clip.y / clip.w, clip.z / clip.w);
    let screen_x = (ndc.x * 0.5 + 0.5) * viewport_width;
    let screen_y = (1.0 - (ndc.y * 0.5 + 0.5)) * viewport_height; // Y flipped
    let depth = ndc.z * 0.5 + 0.5;
    (screen_x, screen_y, depth)
}

/// Unproject a 2D screen point to a 3D world-space ray.
///
/// Returns `(origin, direction)` where origin is on the near plane.
#[must_use]
#[inline]
pub fn screen_to_world_ray(
    screen_x: f32,
    screen_y: f32,
    inverse_vp: Mat4,
    viewport_width: f32,
    viewport_height: f32,
) -> (Vec3, Vec3) {
    let ndc_x = (screen_x / viewport_width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (screen_y / viewport_height) * 2.0;

    let near = inverse_vp * glam::Vec4::new(ndc_x, ndc_y, -1.0, 1.0);
    let far = inverse_vp * glam::Vec4::new(ndc_x, ndc_y, 1.0, 1.0);

    let near_pt = Vec3::new(near.x / near.w, near.y / near.w, near.z / near.w);
    let far_pt = Vec3::new(far.x / far.w, far.y / far.w, far.z / far.w);

    let dir = (far_pt - near_pt).normalize();
    (near_pt, dir)
}

// ---------------------------------------------------------------------------
// Color space conversions
// ---------------------------------------------------------------------------

/// Convert a single sRGB component to linear.
///
/// Uses the official piecewise sRGB transfer function.
#[must_use]
#[inline]
pub fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert a single linear component to sRGB.
///
/// Uses the official piecewise sRGB transfer function.
#[must_use]
#[inline]
pub fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// Convert an sRGB color (Vec3, components in [0,1]) to linear.
#[must_use]
#[inline]
pub fn srgb_to_linear_vec3(color: Vec3) -> Vec3 {
    Vec3::new(
        srgb_to_linear(color.x),
        srgb_to_linear(color.y),
        srgb_to_linear(color.z),
    )
}

/// Convert a linear color (Vec3) to sRGB.
#[must_use]
#[inline]
pub fn linear_to_srgb_vec3(color: Vec3) -> Vec3 {
    Vec3::new(
        linear_to_srgb(color.x),
        linear_to_srgb(color.y),
        linear_to_srgb(color.z),
    )
}

// ---------------------------------------------------------------------------
// Dual quaternions
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
// ---------------------------------------------------------------------------

/// Build a 4×4 saturation matrix.
///
/// `s = 0` is grayscale, `s = 1` is identity, `s > 1` is oversaturated.
#[must_use]
#[inline]
pub fn color_matrix_saturation(s: f32) -> Mat4 {
    let lr = 0.2126 * (1.0 - s);
    let lg = 0.7152 * (1.0 - s);
    let lb = 0.0722 * (1.0 - s);
    Mat4::from_cols_array_2d(&[
        [lr + s, lr, lr, 0.0],
        [lg, lg + s, lg, 0.0],
        [lb, lb, lb + s, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// Build a 4×4 hue rotation matrix (angle in radians).
#[must_use]
#[inline]
pub fn color_matrix_hue_rotate(angle: f32) -> Mat4 {
    let cos = angle.cos();
    let sin = angle.sin();
    // ITU-R BT.709 luma weights
    let r = Vec3::new(0.2126, 0.7152, 0.0722);
    Mat4::from_cols_array_2d(&[
        [
            r.x + cos * (1.0 - r.x) + sin * (-r.x),
            r.x + cos * (-r.x) + sin * 0.143,
            r.x + cos * (-r.x) + sin * (-(1.0 - r.x)),
            0.0,
        ],
        [
            r.y + cos * (-r.y) + sin * (-r.y),
            r.y + cos * (1.0 - r.y) + sin * 0.140,
            r.y + cos * (-r.y) + sin * r.y,
            0.0,
        ],
        [
            r.z + cos * (-r.z) + sin * (1.0 - r.z),
            r.z + cos * (-r.z) + sin * (-0.283),
            r.z + cos * (1.0 - r.z) + sin * r.z,
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// Convert linear RGB to Oklab perceptual color space.
///
/// These coefficients are from Bjorn Ottosson's Oklab specification.
/// Returns `(L, a, b)` where L is lightness in [0, 1].
#[must_use]
#[inline]
#[allow(clippy::excessive_precision)]
pub fn linear_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    let lab_l = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
    let lab_a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
    let lab_b = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

    (lab_l, lab_a, lab_b)
}

/// Convert Oklab to linear RGB.
#[must_use]
#[inline]
#[allow(clippy::excessive_precision)]
pub fn oklab_to_linear(lab_l: f32, lab_a: f32, lab_b: f32) -> (f32, f32, f32) {
    let l_ = lab_l + 0.3963377774 * lab_a + 0.2158037573 * lab_b;
    let m_ = lab_l - 0.1055613458 * lab_a - 0.0638541728 * lab_b;
    let s_ = lab_l - 0.0894841775 * lab_a - 1.2914855480 * lab_b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
    let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
    let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

    (r, g, b)
}

// ---------------------------------------------------------------------------
// Spherical harmonics (L0-L2)
// ---------------------------------------------------------------------------

/// Evaluate real spherical harmonics basis functions up to L2 (9 coefficients).
///
/// `dir` must be normalized. Returns `[Y00, Y1-1, Y10, Y11, Y2-2, Y2-1, Y20, Y21, Y22]`.
#[must_use]
#[inline]
#[allow(clippy::excessive_precision)]
pub fn sh_eval_l2(dir: Vec3) -> [f32; 9] {
    let x = dir.x;
    let y = dir.y;
    let z = dir.z;
    [
        0.282094792,                       // Y00
        0.488602512 * y,                   // Y1-1
        0.488602512 * z,                   // Y10
        0.488602512 * x,                   // Y11
        1.092548431 * x * y,               // Y2-2
        1.092548431 * y * z,               // Y2-1
        0.315391565 * (3.0 * z * z - 1.0), // Y20
        1.092548431 * x * z,               // Y21
        0.546274215 * (x * x - y * y),     // Y22
    ]
}

/// Project a function (sampled as directional values) onto L2 spherical harmonics.
///
/// `samples` is a list of `(direction, value)` pairs.
/// Returns 9 SH coefficients.
#[must_use]
pub fn sh_project_l2(samples: &[(Vec3, f32)]) -> [f32; 9] {
    let mut coeffs = [0.0f32; 9];
    let weight = 4.0 * std::f32::consts::PI / samples.len() as f32;
    for &(dir, val) in samples {
        let basis = sh_eval_l2(dir);
        for i in 0..9 {
            coeffs[i] += val * basis[i] * weight;
        }
    }
    coeffs
}

/// Evaluate SH lighting at a direction given L2 coefficients.
#[must_use]
#[inline]
pub fn sh_evaluate_l2(coeffs: &[f32; 9], dir: Vec3) -> f32 {
    let basis = sh_eval_l2(dir);
    let mut sum = 0.0;
    for i in 0..9 {
        sum += coeffs[i] * basis[i];
    }
    sum
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
        let t = Transform3D::new(Vec3::new(10.0, 0.0, 0.0), Quat::IDENTITY, Vec3::splat(2.0));
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
        use crate::HisabError;
        assert_eq!(
            HisabError::InvalidTransform("bad".to_string()).to_string(),
            "invalid transform: bad"
        );
        assert_eq!(
            HisabError::OutOfRange("too big".to_string()).to_string(),
            "value out of range: too big"
        );
        assert_eq!(HisabError::DivisionByZero.to_string(), "division by zero");
        assert_eq!(
            HisabError::SingularMatrix.to_string(),
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

    #[test]
    fn transform2d_apply_matches_matrix() {
        // Verify optimized apply_to_point matches matrix multiplication
        let t = Transform2D::new(Vec2::new(5.0, -3.0), 1.2, Vec2::new(0.5, 2.0));
        let point = Vec2::new(7.0, -2.0);
        let via_apply = t.apply_to_point(point);
        let via_matrix = t.to_matrix() * Vec3::new(point.x, point.y, 1.0);
        assert!(approx_eq(via_apply.x, via_matrix.x));
        assert!(approx_eq(via_apply.y, via_matrix.y));
        assert!(approx_eq(via_apply.z, 1.0));
    }

    #[test]
    fn transform3d_apply_matches_matrix() {
        // Verify optimized apply_to_point matches matrix multiplication
        let t = Transform3D::new(
            Vec3::new(5.0, -3.0, 1.0),
            Quat::from_rotation_y(0.7),
            Vec3::new(2.0, 0.5, 3.0),
        );
        let point = Vec3::new(1.0, -2.0, 3.0);
        let via_apply = t.apply_to_point(point);
        let m = t.to_matrix();
        let v = m * glam::Vec4::new(point.x, point.y, point.z, 1.0);
        let via_matrix = Vec3::new(v.x, v.y, v.z);
        assert!(vec3_approx_eq(via_apply, via_matrix));
    }

    #[test]
    fn transform3d_apply_combined_rotation_scale_translate() {
        let rot = Quat::from_rotation_z(FRAC_PI_2);
        let t = Transform3D::new(Vec3::new(10.0, 20.0, 30.0), rot, Vec3::new(2.0, 3.0, 4.0));
        let result = t.apply_to_point(Vec3::new(1.0, 0.0, 0.0));
        assert!(vec3_approx_eq(result, Vec3::new(10.0, 22.0, 30.0)));
    }

    // --- V0.2 tests ---

    #[test]
    fn transform3d_inverse_matrix_identity() {
        let inv = Transform3D::IDENTITY.inverse_matrix();
        assert_eq!(inv, Mat4::IDENTITY);
    }

    #[test]
    fn transform3d_inverse_matrix_roundtrip() {
        let t = Transform3D::new(
            Vec3::new(3.0, -5.0, 7.0),
            Quat::from_rotation_y(1.2),
            Vec3::new(2.0, 0.5, 3.0),
        );
        let inv = t.inverse_matrix();
        let p = glam::Vec4::new(1.0, 2.0, 3.0, 1.0);
        let q = t.to_matrix() * p;
        let result = inv * q;
        assert!(approx_eq(result.x, p.x));
        assert!(approx_eq(result.y, p.y));
        assert!(approx_eq(result.z, p.z));
    }

    #[test]
    fn transform3d_inverse_matrix_translation_only() {
        let t = Transform3D::new(Vec3::new(10.0, 20.0, 30.0), Quat::IDENTITY, Vec3::ONE);
        let inv = t.inverse_matrix();
        let p = glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let result = inv * (t.to_matrix() * p);
        assert!(approx_eq(result.x, 0.0));
        assert!(approx_eq(result.y, 0.0));
        assert!(approx_eq(result.z, 0.0));
    }

    #[test]
    fn slerp_endpoints() {
        let a = Quat::IDENTITY;
        let b = Quat::from_rotation_y(FRAC_PI_2);
        let at_0 = slerp(a, b, 0.0);
        let at_1 = slerp(a, b, 1.0);
        assert!(approx_eq(at_0.x, a.x) && approx_eq(at_0.w, a.w));
        assert!(approx_eq(at_1.x, b.x) && approx_eq(at_1.w, b.w));
    }

    #[test]
    fn slerp_midpoint() {
        let a = Quat::IDENTITY;
        let b = Quat::from_rotation_y(FRAC_PI_2);
        let mid = slerp(a, b, 0.5);
        // Midpoint should be a 45-degree rotation
        let expected = Quat::from_rotation_y(FRAC_PI_4);
        assert!(approx_eq(mid.x, expected.x));
        assert!(approx_eq(mid.y, expected.y));
        assert!(approx_eq(mid.z, expected.z));
        assert!(approx_eq(mid.w, expected.w));
    }

    #[test]
    fn transform3d_lerp_endpoints() {
        let a = Transform3D::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
        let b = Transform3D::new(
            Vec3::new(10.0, 0.0, 0.0),
            Quat::from_rotation_y(FRAC_PI_2),
            Vec3::splat(2.0),
        );
        let at_0 = transform3d_lerp(&a, &b, 0.0);
        let at_1 = transform3d_lerp(&a, &b, 1.0);
        assert!(vec3_approx_eq(at_0.position, a.position));
        assert!(vec3_approx_eq(at_0.scale, a.scale));
        assert!(vec3_approx_eq(at_1.position, b.position));
        assert!(vec3_approx_eq(at_1.scale, b.scale));
    }

    #[test]
    fn transform3d_lerp_midpoint() {
        let a = Transform3D::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
        let b = Transform3D::new(Vec3::new(10.0, 0.0, 0.0), Quat::IDENTITY, Vec3::splat(3.0));
        let mid = transform3d_lerp(&a, &b, 0.5);
        assert!(vec3_approx_eq(mid.position, Vec3::new(5.0, 0.0, 0.0)));
        assert!(vec3_approx_eq(mid.scale, Vec3::splat(2.0)));
    }

    #[test]
    fn flip_handedness_z_double_flip() {
        let m = projection_perspective(FRAC_PI_4, 1.0, 0.1, 100.0);
        let flipped = flip_handedness_z(m);
        let restored = flip_handedness_z(flipped);
        // Double flip should restore original
        let a = m.to_cols_array();
        let b = restored.to_cols_array();
        for i in 0..16 {
            assert!(approx_eq(a[i], b[i]));
        }
    }

    #[test]
    fn flip_handedness_z_negates_z_column() {
        let m = Mat4::IDENTITY;
        let f = flip_handedness_z(m);
        let cols = f.to_cols_array_2d();
        assert!(approx_eq(cols[2][2], -1.0));
        assert!(approx_eq(cols[0][0], 1.0)); // X unchanged
        assert!(approx_eq(cols[1][1], 1.0)); // Y unchanged
    }

    // --- V1.0b: Transform2D::inverse_matrix ---

    #[test]
    fn transform2d_inverse_matrix_identity() {
        let t = Transform2D::IDENTITY;
        let inv = t.inverse_matrix();
        let cols = inv.to_cols_array_2d();
        assert!(approx_eq(cols[0][0], 1.0));
        assert!(approx_eq(cols[1][1], 1.0));
        assert!(approx_eq(cols[2][2], 1.0));
    }

    #[test]
    fn transform2d_inverse_matrix_roundtrip() {
        let t = Transform2D::new(Vec2::new(3.0, -7.0), 0.8, Vec2::new(2.0, 0.5));
        let m = t.to_matrix();
        let inv = t.inverse_matrix();
        let product = m * inv;
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(approx_eq(product.to_cols_array_2d()[i][j], expected));
            }
        }
    }

    #[test]
    fn transform2d_inverse_matrix_undo_point() {
        let t = Transform2D::new(Vec2::new(5.0, 3.0), 1.2, Vec2::new(1.5, 2.0));
        let original = Vec2::new(4.0, -2.0);
        let transformed = t.apply_to_point(original);
        let inv = t.inverse_matrix();
        let recovered = inv * transformed;
        assert!(approx_eq(recovered.x, original.x));
        assert!(approx_eq(recovered.y, original.y));
    }

    // --- Quaternion utilities ---

    #[test]
    fn euler_roundtrip_xyz() {
        let (x, y, z) = (0.3, 0.5, 0.7);
        let q = quat_from_euler(x, y, z, EulerOrder::XYZ);
        let (rx, ry, rz) = quat_to_euler(q, EulerOrder::XYZ);
        assert!(approx_eq(rx, x));
        assert!(approx_eq(ry, y));
        assert!(approx_eq(rz, z));
    }

    #[test]
    fn euler_roundtrip_zyx() {
        let (x, y, z) = (0.1, -0.2, 0.4);
        let q = quat_from_euler(x, y, z, EulerOrder::ZYX);
        let (rx, ry, rz) = quat_to_euler(q, EulerOrder::ZYX);
        assert!(approx_eq(rx, x));
        assert!(approx_eq(ry, y));
        assert!(approx_eq(rz, z));
    }

    #[test]
    fn quat_look_at_forward_z() {
        let q = quat_look_at(Vec3::Z, Vec3::Y);
        let forward = q * Vec3::Z;
        assert!(vec3_approx_eq(forward, Vec3::Z));
    }

    #[test]
    fn look_at_rh_basic() {
        let m = look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
        // Camera at (0,0,5) looking at origin — origin should map to (0,0,-5) in view space
        let p = m * glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
        assert!(approx_eq(p.z, -5.0));
    }

    // --- Screen projection ---

    #[test]
    fn world_to_screen_center() {
        let proj = projection_perspective(FRAC_PI_4, 1.0, 0.1, 100.0);
        let (sx, sy, _) = world_to_screen(Vec3::new(0.0, 0.0, -5.0), proj, 800.0, 600.0);
        // Center of screen
        assert!((sx - 400.0).abs() < 1.0);
        assert!((sy - 300.0).abs() < 1.0);
    }

    #[test]
    fn screen_to_world_ray_center() {
        let proj = projection_perspective(FRAC_PI_4, 1.0, 0.1, 100.0);
        let inv = proj.inverse();
        let (_, dir) = screen_to_world_ray(400.0, 300.0, inv, 800.0, 600.0);
        // Center ray should point down -Z
        assert!(dir.z < -0.9);
    }

    // --- sRGB conversions ---

    #[test]
    fn srgb_linear_roundtrip() {
        for i in 0..=10 {
            let c = i as f32 / 10.0;
            let linear = srgb_to_linear(c);
            let back = linear_to_srgb(linear);
            assert!(
                (back - c).abs() < 1e-4,
                "roundtrip failed for {c}: got {back}"
            );
        }
    }

    #[test]
    fn srgb_endpoints() {
        assert!(approx_eq(srgb_to_linear(0.0), 0.0));
        assert!(approx_eq(srgb_to_linear(1.0), 1.0));
        assert!(approx_eq(linear_to_srgb(0.0), 0.0));
        assert!(approx_eq(linear_to_srgb(1.0), 1.0));
    }

    #[test]
    fn srgb_midpoint_gamma() {
        let l = srgb_to_linear(0.5);
        assert!(l > 0.2 && l < 0.23);
    }

    // --- Dual quaternion tests ---

    #[test]
    fn dualquat_identity() {
        let dq = DualQuat::IDENTITY;
        let p = dq.transform_point(Vec3::new(1.0, 2.0, 3.0));
        assert!(vec3_approx_eq(p, Vec3::new(1.0, 2.0, 3.0)));
    }

    #[test]
    fn dualquat_translation() {
        let dq = DualQuat::from_rotation_translation(Quat::IDENTITY, Vec3::new(5.0, 0.0, 0.0));
        let p = dq.transform_point(Vec3::ZERO);
        assert!(vec3_approx_eq(p, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn dualquat_roundtrip() {
        let rot = Quat::from_rotation_y(0.5);
        let trans = Vec3::new(1.0, 2.0, 3.0);
        let dq = DualQuat::from_rotation_translation(rot, trans);
        let t_back = dq.translation();
        assert!(vec3_approx_eq(t_back, trans));
    }

    #[test]
    fn dualquat_blend_endpoints() {
        let a = DualQuat::from_rotation_translation(Quat::IDENTITY, Vec3::ZERO);
        let b = DualQuat::from_rotation_translation(Quat::IDENTITY, Vec3::new(10.0, 0.0, 0.0));
        let mid = DualQuat::blend(&a, &b, 0.5);
        let t = mid.translation();
        assert!(approx_eq(t.x, 5.0));
    }

    // --- CSS decompose tests ---

    #[test]
    fn decompose_identity() {
        let d = decompose_mat4(Mat4::IDENTITY).unwrap();
        assert!(vec3_approx_eq(d.translation, Vec3::ZERO));
        assert!(vec3_approx_eq(d.scale, Vec3::ONE));
    }

    #[test]
    fn decompose_recompose_roundtrip() {
        let m = Mat4::from_scale_rotation_translation(
            Vec3::new(2.0, 3.0, 4.0),
            Quat::from_rotation_y(0.5),
            Vec3::new(10.0, 20.0, 30.0),
        );
        let d = decompose_mat4(m).unwrap();
        let r = recompose_mat4(&d);
        let cols_a = m.to_cols_array();
        let cols_b = r.to_cols_array();
        for i in 0..16 {
            assert!(approx_eq(cols_a[i], cols_b[i]));
        }
    }

    // --- Oklab tests ---

    #[test]
    fn oklab_roundtrip() {
        let (r, g, b) = (0.5, 0.3, 0.1);
        let (l, a, ob) = linear_to_oklab(r, g, b);
        let (rr, gg, bb) = oklab_to_linear(l, a, ob);
        assert!((rr - r).abs() < 0.01);
        assert!((gg - g).abs() < 0.01);
        assert!((bb - b).abs() < 0.01);
    }

    // --- Spherical harmonics tests ---

    #[test]
    fn sh_eval_l2_normalization() {
        // Y00 at any direction should be constant
        let a = sh_eval_l2(Vec3::X);
        let b = sh_eval_l2(Vec3::Y);
        assert!(approx_eq(a[0], b[0])); // Y00 is constant
    }

    #[test]
    fn sh_project_evaluate_constant() {
        // Project a constant function → only Y00 should be nonzero
        let samples: Vec<(Vec3, f32)> = [
            Vec3::X,
            Vec3::Y,
            Vec3::Z,
            Vec3::NEG_X,
            Vec3::NEG_Y,
            Vec3::NEG_Z,
        ]
        .iter()
        .map(|&d| (d, 1.0))
        .collect();
        let coeffs = sh_project_l2(&samples);
        // Y00 coefficient should be ~√(4π) ≈ 3.545 * 0.282 ≈ 1.0
        assert!(coeffs[0] > 0.5);
        // Evaluate at any direction should give ~1.0
        let val = sh_evaluate_l2(&coeffs, Vec3::new(1.0, 1.0, 1.0).normalize());
        assert!((val - 1.0).abs() < 0.5); // Approximate with few samples
    }

    // --- Color matrix tests ---

    #[test]
    fn saturation_zero_is_grayscale() {
        let m = color_matrix_saturation(0.0);
        let color = m * glam::Vec4::new(1.0, 0.0, 0.0, 1.0);
        // Pure red → grayscale, all channels should be equal
        assert!((color.x - color.y).abs() < 0.01);
        assert!((color.y - color.z).abs() < 0.01);
    }

    #[test]
    fn saturation_one_is_identity() {
        let m = color_matrix_saturation(1.0);
        let color = m * glam::Vec4::new(0.5, 0.3, 0.1, 1.0);
        assert!(approx_eq(color.x, 0.5));
        assert!(approx_eq(color.y, 0.3));
        assert!(approx_eq(color.z, 0.1));
    }
}
