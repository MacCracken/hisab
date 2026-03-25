use super::*;

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

