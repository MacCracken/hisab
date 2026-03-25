use super::*;

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

