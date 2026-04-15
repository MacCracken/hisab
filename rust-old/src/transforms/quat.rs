use super::*;

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
