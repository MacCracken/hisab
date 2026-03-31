mod core;
pub use core::*;

mod quat;
pub use quat::*;
mod screen;
pub use screen::*;
mod color;
pub use color::*;
mod dualquat;
pub use dualquat::*;
mod decompose;
pub use decompose::*;
mod sh;
pub use sh::*;
mod lie;
pub use lie::*;

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

    // --- inverse_lerp / remap ---

    #[test]
    fn inverse_lerp_basic() {
        assert!(approx_eq(inverse_lerp(0.0, 10.0, 5.0), 0.5));
        assert!(approx_eq(inverse_lerp(0.0, 10.0, 0.0), 0.0));
        assert!(approx_eq(inverse_lerp(0.0, 10.0, 10.0), 1.0));
    }

    #[test]
    fn inverse_lerp_degenerate() {
        assert!(approx_eq(inverse_lerp(5.0, 5.0, 5.0), 0.0));
    }

    #[test]
    fn remap_basic() {
        assert!(approx_eq(remap(5.0, 0.0, 10.0, 0.0, 100.0), 50.0));
        assert!(approx_eq(remap(0.0, 0.0, 10.0, 20.0, 40.0), 20.0));
        assert!(approx_eq(remap(10.0, 0.0, 10.0, 20.0, 40.0), 40.0));
    }

    #[test]
    fn lerp_inverse_lerp_roundtrip() {
        let a = 3.0;
        let b = 17.0;
        let t = 0.7;
        let v = lerp_f32(a, b, t);
        let t_back = inverse_lerp(a, b, v);
        assert!(approx_eq(t_back, t));
    }

    // --- reverse-Z projection ---

    #[test]
    fn reverse_z_near_maps_to_one() {
        let m = projection_perspective_reverse_z(FRAC_PI_4, 1.0, 0.1);
        // Near plane point: z = -0.1
        let p = m * glam::Vec4::new(0.0, 0.0, -0.1, 1.0);
        let ndc_z = p.z / p.w;
        assert!(approx_eq(ndc_z, 1.0));
    }

    #[test]
    fn reverse_z_far_approaches_zero() {
        let m = projection_perspective_reverse_z(FRAC_PI_4, 1.0, 0.1);
        // Very far point: z = -10000
        let p = m * glam::Vec4::new(0.0, 0.0, -10000.0, 1.0);
        let ndc_z = p.z / p.w;
        assert!((0.0..0.001).contains(&ndc_z));
    }

    // --- HSV roundtrip ---

    #[test]
    fn hsv_roundtrip() {
        let cases = [(0.8, 0.3, 0.1), (0.1, 0.9, 0.5), (0.5, 0.5, 0.5)];
        for (r, g, b) in cases {
            let (h, s, v) = linear_to_hsv(r, g, b);
            let (rr, gg, bb) = hsv_to_linear(h, s, v);
            assert!((rr - r).abs() < 1e-4, "R: {r} -> {rr}");
            assert!((gg - g).abs() < 1e-4, "G: {g} -> {gg}");
            assert!((bb - b).abs() < 1e-4, "B: {b} -> {bb}");
        }
    }

    #[test]
    fn hsv_grayscale() {
        let (_, s, _) = linear_to_hsv(0.5, 0.5, 0.5);
        assert!(approx_eq(s, 0.0));
    }

    // --- HSL roundtrip ---

    #[test]
    fn hsl_roundtrip() {
        let cases = [(0.8, 0.3, 0.1), (0.1, 0.9, 0.5), (0.5, 0.5, 0.5)];
        for (r, g, b) in cases {
            let (h, s, l) = linear_to_hsl(r, g, b);
            let (rr, gg, bb) = hsl_to_linear(h, s, l);
            assert!((rr - r).abs() < 1e-4, "R: {r} -> {rr}");
            assert!((gg - g).abs() < 1e-4, "G: {g} -> {gg}");
            assert!((bb - b).abs() < 1e-4, "B: {b} -> {bb}");
        }
    }

    // --- Premultiplied alpha ---

    #[test]
    fn premultiply_roundtrip() {
        let (r, g, b, a) = (0.8, 0.6, 0.4, 0.5);
        let (pr, pg, pb, pa) = premultiply_alpha(r, g, b, a);
        assert!(approx_eq(pr, 0.4));
        assert!(approx_eq(pa, 0.5));
        let (ur, ug, ub, ua) = unpremultiply_alpha(pr, pg, pb, pa);
        assert!(approx_eq(ur, r));
        assert!(approx_eq(ug, g));
        assert!(approx_eq(ub, b));
        assert!(approx_eq(ua, a));
    }

    #[test]
    fn unpremultiply_transparent() {
        let (r, _g, _b, a) = unpremultiply_alpha(0.0, 0.0, 0.0, 0.0);
        assert!(approx_eq(r, 0.0));
        assert!(approx_eq(a, 0.0));
    }

    // --- Transform composition ---

    #[test]
    fn transform2d_compose_identity() {
        let t = Transform2D::new(Vec2::new(3.0, 4.0), 0.5, Vec2::new(2.0, 1.0));
        let composed = t.compose(&Transform2D::IDENTITY);
        let p = Vec2::new(1.0, 1.0);
        let a = t.apply_to_point(p);
        let b = composed.apply_to_point(p);
        assert!(approx_eq(a.x, b.x));
        assert!(approx_eq(a.y, b.y));
    }

    #[test]
    fn transform2d_compose_matches_matrix() {
        let a = Transform2D::new(Vec2::new(1.0, 2.0), 0.3, Vec2::ONE);
        let b = Transform2D::new(Vec2::new(3.0, -1.0), 0.7, Vec2::new(2.0, 2.0));
        let composed = a.compose(&b);
        let p = Vec2::new(5.0, -3.0);
        // Matrix path: b.matrix * a.matrix * point
        let m = b.to_matrix() * a.to_matrix();
        let via_matrix = m * Vec3::new(p.x, p.y, 1.0);
        let via_compose = composed.apply_to_point(p);
        assert!((via_compose.x - via_matrix.x).abs() < 1e-3);
        assert!((via_compose.y - via_matrix.y).abs() < 1e-3);
    }

    #[test]
    fn transform3d_compose_identity() {
        let t = Transform3D::new(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::from_rotation_y(0.5),
            Vec3::ONE,
        );
        let composed = t.compose(&Transform3D::IDENTITY);
        let p = Vec3::new(1.0, 1.0, 1.0);
        let a = t.apply_to_point(p);
        let b = composed.apply_to_point(p);
        assert!(vec3_approx_eq(a, b));
    }

    #[test]
    fn transform3d_compose_matches_matrix() {
        let a = Transform3D::new(
            Vec3::new(1.0, 0.0, 0.0),
            Quat::from_rotation_y(0.3),
            Vec3::ONE,
        );
        let b = Transform3D::new(
            Vec3::new(0.0, 5.0, 0.0),
            Quat::from_rotation_x(0.5),
            Vec3::splat(2.0),
        );
        let composed = a.compose(&b);
        let p = Vec3::new(1.0, -1.0, 2.0);
        let m = b.to_matrix() * a.to_matrix();
        let v = m * glam::Vec4::new(p.x, p.y, p.z, 1.0);
        let via_matrix = Vec3::new(v.x, v.y, v.z);
        let via_compose = composed.apply_to_point(p);
        assert!(vec3_approx_eq(via_compose, via_matrix));
    }

    // --- Porter-Duff compositing ---

    #[test]
    fn composite_src_over_opaque() {
        // Fully opaque source should replace destination
        let (r, g, _b, a) = composite_src_over(1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        assert!(approx_eq(r, 1.0));
        assert!(approx_eq(g, 0.0));
        assert!(approx_eq(a, 1.0));
    }

    #[test]
    fn composite_src_over_transparent() {
        // Fully transparent source should pass through destination
        let (r, _g, _b, a) = composite_src_over(0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0);
        assert!(approx_eq(r, 0.5));
        assert!(approx_eq(a, 1.0));
    }

    #[test]
    fn composite_src_in_masks() {
        // src-in: source clipped by destination alpha
        let (r, _g, _b, a) = composite_src_in(1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.5);
        assert!(approx_eq(r, 0.5));
        assert!(approx_eq(a, 0.5));
    }

    #[test]
    fn composite_plus_clamped() {
        let (r, _g, _b, a) = composite_plus(0.8, 0.0, 0.0, 1.0, 0.5, 0.0, 0.0, 1.0);
        assert!(approx_eq(r, 1.0)); // Clamped
        assert!(approx_eq(a, 1.0));
    }

    // --- HDR tone mapping ---

    #[test]
    fn tonemap_reinhard_zero() {
        let (r, _g, _b) = tonemap_reinhard(0.0, 0.0, 0.0);
        assert!(approx_eq(r, 0.0));
    }

    #[test]
    fn tonemap_reinhard_convergence() {
        let (r, _, _) = tonemap_reinhard(1000.0, 0.0, 0.0);
        assert!(r > 0.99 && r <= 1.0); // Approaches 1.0
    }

    #[test]
    fn tonemap_aces_range() {
        let (r, _g, _b) = tonemap_aces(1.0, 1.0, 1.0);
        assert!((0.0..=1.0).contains(&r));
    }

    // --- Depth linearization ---

    #[test]
    fn linearize_depth_near_far() {
        let near = 0.1;
        let far = 100.0;
        let d_near = linearize_depth(0.0, near, far);
        let d_far = linearize_depth(1.0, near, far);
        assert!((d_near - near).abs() < 0.01);
        assert!((d_far - far).abs() < 0.01);
    }

    #[test]
    fn linearize_depth_reverse_z_near() {
        let d = linearize_depth_reverse_z(1.0, 0.1);
        assert!((d - 0.1).abs() < 0.001);
    }

    // --- lerp_srgb ---

    #[test]
    fn lerp_srgb_endpoints() {
        let black = (0.0f32, 0.0, 0.0);
        let white = (1.0f32, 1.0, 1.0);
        let at_zero = lerp_srgb(black, white, 0.0);
        let at_one = lerp_srgb(black, white, 1.0);
        assert!((at_zero.0 - 0.0).abs() < 1e-6);
        assert!((at_one.0 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn lerp_srgb_midpoint_brighter_than_naive() {
        // Gamma-correct midpoint of black/white should be > 0.5 in sRGB
        let mid = lerp_srgb((0.0, 0.0, 0.0), (1.0, 1.0, 1.0), 0.5);
        assert!(
            mid.0 > 0.5,
            "gamma-aware mid ({}) should exceed naive 0.5",
            mid.0
        );
        assert!((mid.0 - 0.735).abs() < 0.01);
    }

    #[test]
    fn lerp_srgb_symmetry() {
        let a = (0.2f32, 0.4, 0.8);
        let b = (0.8f32, 0.2, 0.3);
        let ab = lerp_srgb(a, b, 0.3);
        let ba = lerp_srgb(b, a, 0.7);
        assert!((ab.0 - ba.0).abs() < 1e-5);
        assert!((ab.1 - ba.1).abs() < 1e-5);
        assert!((ab.2 - ba.2).abs() < 1e-5);
    }

    #[test]
    fn lerp_srgb_vec3_matches_tuple() {
        use glam::Vec3;
        let a = (0.3f32, 0.5, 0.7);
        let b = (0.7f32, 0.3, 0.1);
        let t = 0.4;
        let tuple_result = lerp_srgb(a, b, t);
        let vec_result = lerp_srgb_vec3(Vec3::new(a.0, a.1, a.2), Vec3::new(b.0, b.1, b.2), t);
        assert!((tuple_result.0 - vec_result.x).abs() < 1e-6);
        assert!((tuple_result.1 - vec_result.y).abs() < 1e-6);
        assert!((tuple_result.2 - vec_result.z).abs() < 1e-6);
    }

    #[test]
    fn lerp_srgb_same_color() {
        let color = (0.5f32, 0.3, 0.8);
        let result = lerp_srgb(color, color, 0.5);
        assert!((result.0 - color.0).abs() < 1e-5);
        assert!((result.1 - color.1).abs() < 1e-5);
        assert!((result.2 - color.2).abs() < 1e-5);
    }

    // --- ev100_to_luminance / luminance_to_ev100 / ev100_to_exposure ---

    #[test]
    fn ev100_luminance_known_value() {
        use std::f32::consts::PI;
        // EV=3: L = 2^0 * 12.5 / π = 12.5 / π
        let lum = ev100_to_luminance(3.0);
        assert!((lum - 12.5 / PI).abs() < 0.001);
    }

    #[test]
    fn ev100_luminance_roundtrip() {
        for ev in [-2.0f32, 0.0, 3.0, 6.0, 12.0] {
            let lum = ev100_to_luminance(ev);
            let ev_back = luminance_to_ev100(lum);
            assert!(
                (ev_back - ev).abs() < 1e-4,
                "roundtrip failed for EV={ev}: got {ev_back}"
            );
        }
    }

    #[test]
    fn ev100_exposure_monotone_decreasing() {
        let e0 = ev100_to_exposure(0.0);
        let e3 = ev100_to_exposure(3.0);
        let e6 = ev100_to_exposure(6.0);
        assert!(e0 > e3, "exposure should decrease with higher EV");
        assert!(e3 > e6, "exposure should decrease with higher EV");
    }

    #[test]
    fn ev100_exposure_formula() {
        // ev=0: 1 / (1.2 * 1.0) = 1/1.2
        let e = ev100_to_exposure(0.0);
        assert!((e - 1.0 / 1.2).abs() < 1e-6);
    }

    #[test]
    fn ev100_luminance_doubling() {
        // Each +1 EV doubles the luminance
        let l0 = ev100_to_luminance(4.0);
        let l1 = ev100_to_luminance(5.0);
        assert!((l1 / l0 - 2.0).abs() < 1e-4);
    }
}
