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

/// Convert an sRGB color (Vec3, components in \[0,1\]) to linear.
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
// HSV / HSL conversions
// ---------------------------------------------------------------------------

/// Convert linear RGB to HSV (hue in radians 0..TAU, saturation 0..1, value 0..1).
#[must_use]
#[inline]
pub fn linear_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let v = max;
    let s = if max > crate::EPSILON_F32 {
        delta / max
    } else {
        0.0
    };

    let h = if delta < crate::EPSILON_F32 {
        0.0
    } else if (max - r).abs() < crate::EPSILON_F32 {
        std::f32::consts::FRAC_PI_3 * ((g - b) / delta).rem_euclid(6.0)
    } else if (max - g).abs() < crate::EPSILON_F32 {
        std::f32::consts::FRAC_PI_3 * ((b - r) / delta + 2.0)
    } else {
        std::f32::consts::FRAC_PI_3 * ((r - g) / delta + 4.0)
    };

    (h, s, v)
}

/// Convert HSV to linear RGB (hue in radians 0..TAU).
#[must_use]
#[inline]
pub fn hsv_to_linear(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s < crate::EPSILON_F32 {
        return (v, v, v);
    }
    let h_deg = h / std::f32::consts::FRAC_PI_3; // 0..6
    let sector = h_deg.floor() as i32;
    let f = h_deg - sector as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    match sector.rem_euclid(6) {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

/// Convert linear RGB to HSL (hue in radians 0..TAU, saturation 0..1, lightness 0..1).
#[must_use]
#[inline]
pub fn linear_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let l = (max + min) * 0.5;

    let s = if delta < crate::EPSILON_F32 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta < crate::EPSILON_F32 {
        0.0
    } else if (max - r).abs() < crate::EPSILON_F32 {
        std::f32::consts::FRAC_PI_3 * ((g - b) / delta).rem_euclid(6.0)
    } else if (max - g).abs() < crate::EPSILON_F32 {
        std::f32::consts::FRAC_PI_3 * ((b - r) / delta + 2.0)
    } else {
        std::f32::consts::FRAC_PI_3 * ((r - g) / delta + 4.0)
    };

    (h, s, l)
}

/// Convert HSL to linear RGB (hue in radians 0..TAU).
#[must_use]
#[inline]
pub fn hsl_to_linear(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s < crate::EPSILON_F32 {
        return (l, l, l);
    }
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_deg = h / std::f32::consts::FRAC_PI_3; // 0..6
    let x = c * (1.0 - (h_deg.rem_euclid(2.0) - 1.0).abs());
    let m = l - c * 0.5;
    let sector = h_deg.floor() as i32;
    let (r, g, b) = match sector.rem_euclid(6) {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (r + m, g + m, b + m)
}

// ---------------------------------------------------------------------------
// Premultiplied alpha
// ---------------------------------------------------------------------------

/// Convert a straight-alpha RGBA color to premultiplied alpha.
///
/// `(r, g, b)` are multiplied by `a`.
#[must_use]
#[inline]
pub fn premultiply_alpha(r: f32, g: f32, b: f32, a: f32) -> (f32, f32, f32, f32) {
    (r * a, g * a, b * a, a)
}

/// Convert a premultiplied-alpha RGBA color back to straight alpha.
///
/// Divides `(r, g, b)` by `a`. Returns `(0, 0, 0, 0)` for fully transparent pixels.
#[must_use]
#[inline]
pub fn unpremultiply_alpha(r: f32, g: f32, b: f32, a: f32) -> (f32, f32, f32, f32) {
    if a < crate::EPSILON_F32 {
        (0.0, 0.0, 0.0, 0.0)
    } else {
        let inv_a = 1.0 / a;
        (r * inv_a, g * inv_a, b * inv_a, a)
    }
}

// ---------------------------------------------------------------------------
// Porter-Duff compositing operators (premultiplied alpha)
// ---------------------------------------------------------------------------

/// Porter-Duff compositing: source over destination.
///
/// All inputs and outputs are in **premultiplied alpha** space.
/// Each function takes `(src_r, src_g, src_b, src_a, dst_r, dst_g, dst_b, dst_a)`
/// and returns `(out_r, out_g, out_b, out_a)`.
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn composite_src_over(
    sr: f32,
    sg: f32,
    sb: f32,
    sa: f32,
    dr: f32,
    dg: f32,
    db: f32,
    da: f32,
) -> (f32, f32, f32, f32) {
    let inv_sa = 1.0 - sa;
    (
        sr + dr * inv_sa,
        sg + dg * inv_sa,
        sb + db * inv_sa,
        sa + da * inv_sa,
    )
}

/// Porter-Duff: destination over source.
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn composite_dst_over(
    sr: f32,
    sg: f32,
    sb: f32,
    sa: f32,
    dr: f32,
    dg: f32,
    db: f32,
    da: f32,
) -> (f32, f32, f32, f32) {
    let inv_da = 1.0 - da;
    (
        dr + sr * inv_da,
        dg + sg * inv_da,
        db + sb * inv_da,
        da + sa * inv_da,
    )
}

/// Porter-Duff: source in destination.
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn composite_src_in(
    sr: f32,
    sg: f32,
    sb: f32,
    sa: f32,
    _dr: f32,
    _dg: f32,
    _db: f32,
    da: f32,
) -> (f32, f32, f32, f32) {
    (sr * da, sg * da, sb * da, sa * da)
}

/// Porter-Duff: destination in source.
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn composite_dst_in(
    _sr: f32,
    _sg: f32,
    _sb: f32,
    sa: f32,
    dr: f32,
    dg: f32,
    db: f32,
    da: f32,
) -> (f32, f32, f32, f32) {
    (dr * sa, dg * sa, db * sa, da * sa)
}

/// Porter-Duff: source out (source held out by destination).
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn composite_src_out(
    sr: f32,
    sg: f32,
    sb: f32,
    sa: f32,
    _dr: f32,
    _dg: f32,
    _db: f32,
    da: f32,
) -> (f32, f32, f32, f32) {
    let inv_da = 1.0 - da;
    (sr * inv_da, sg * inv_da, sb * inv_da, sa * inv_da)
}

/// Porter-Duff: destination out (destination held out by source).
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn composite_dst_out(
    _sr: f32,
    _sg: f32,
    _sb: f32,
    sa: f32,
    dr: f32,
    dg: f32,
    db: f32,
    da: f32,
) -> (f32, f32, f32, f32) {
    let inv_sa = 1.0 - sa;
    (dr * inv_sa, dg * inv_sa, db * inv_sa, da * inv_sa)
}

/// Porter-Duff: source atop destination.
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn composite_src_atop(
    sr: f32,
    sg: f32,
    sb: f32,
    sa: f32,
    dr: f32,
    dg: f32,
    db: f32,
    da: f32,
) -> (f32, f32, f32, f32) {
    let inv_sa = 1.0 - sa;
    (
        sr * da + dr * inv_sa,
        sg * da + dg * inv_sa,
        sb * da + db * inv_sa,
        da,
    )
}

/// Porter-Duff: destination atop source.
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn composite_dst_atop(
    sr: f32,
    sg: f32,
    sb: f32,
    sa: f32,
    dr: f32,
    dg: f32,
    db: f32,
    da: f32,
) -> (f32, f32, f32, f32) {
    let inv_da = 1.0 - da;
    (
        dr * sa + sr * inv_da,
        dg * sa + sg * inv_da,
        db * sa + sb * inv_da,
        sa,
    )
}

/// Porter-Duff: XOR (exclusive or).
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn composite_xor(
    sr: f32,
    sg: f32,
    sb: f32,
    sa: f32,
    dr: f32,
    dg: f32,
    db: f32,
    da: f32,
) -> (f32, f32, f32, f32) {
    let inv_sa = 1.0 - sa;
    let inv_da = 1.0 - da;
    (
        sr * inv_da + dr * inv_sa,
        sg * inv_da + dg * inv_sa,
        sb * inv_da + db * inv_sa,
        sa * inv_da + da * inv_sa,
    )
}

/// Porter-Duff: additive blend (plus / lighter).
#[must_use]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn composite_plus(
    sr: f32,
    sg: f32,
    sb: f32,
    sa: f32,
    dr: f32,
    dg: f32,
    db: f32,
    da: f32,
) -> (f32, f32, f32, f32) {
    (
        (sr + dr).min(1.0),
        (sg + dg).min(1.0),
        (sb + db).min(1.0),
        (sa + da).min(1.0),
    )
}

// ---------------------------------------------------------------------------
// HDR tone mapping
// ---------------------------------------------------------------------------

/// Reinhard tone mapping operator (per-channel).
///
/// Maps HDR luminance to \[0, 1\] via `L / (1 + L)`.
#[must_use]
#[inline]
pub fn tonemap_reinhard(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    (r / (1.0 + r), g / (1.0 + g), b / (1.0 + b))
}

/// Extended Reinhard with adjustable white point.
///
/// `white` is the luminance value that maps to 1.0 (controls highlight rolloff).
#[must_use]
#[inline]
pub fn tonemap_reinhard_extended(r: f32, g: f32, b: f32, white: f32) -> (f32, f32, f32) {
    let w2 = white * white;
    let map = |c: f32| c * (1.0 + c / w2) / (1.0 + c);
    (map(r), map(g), map(b))
}

/// ACES filmic tone mapping (Narkowicz approximation).
///
/// Industry-standard cinematic tone curve from the Academy Color Encoding System.
#[must_use]
#[inline]
pub fn tonemap_aces(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    // Narkowicz 2015, "ACES Filmic Tone Mapping Curve"
    let map = |x: f32| {
        let a = 2.51;
        let b = 0.03;
        let c = 2.43;
        let d = 0.59;
        let e = 0.14;
        ((x * (a * x + b)) / (x * (c * x + d) + e)).clamp(0.0, 1.0)
    };
    (map(r), map(g), map(b))
}

// ---------------------------------------------------------------------------
// Depth buffer utilities
// ---------------------------------------------------------------------------

/// Linearize a depth buffer value from NDC to view-space depth.
///
/// For standard (non-reverse-Z) perspective projection where near maps to 0
/// and far maps to 1 in NDC.
#[must_use]
#[inline]
pub fn linearize_depth(ndc_depth: f32, near: f32, far: f32) -> f32 {
    near * far / (far - ndc_depth * (far - near))
}

/// Linearize a reverse-Z depth buffer value to view-space depth.
///
/// For reverse-Z projection where near maps to 1 and far maps to 0.
#[must_use]
#[inline]
pub fn linearize_depth_reverse_z(ndc_depth: f32, near: f32) -> f32 {
    near / ndc_depth
}

// ---------------------------------------------------------------------------
// Gamma-aware interpolation
// ---------------------------------------------------------------------------

/// Gamma-aware linear interpolation: decode sRGB → lerp in linear light → encode sRGB.
///
/// Interpolating directly in sRGB space produces perceptually dark midpoints.
/// This function converts both endpoints to linear light, lerps each channel,
/// then encodes the result back to sRGB.
///
/// # Examples
///
/// ```
/// use hisab::transforms::lerp_srgb;
///
/// // Midpoint of black and white in linear-aware sRGB
/// let mid = lerp_srgb((0.0, 0.0, 0.0), (1.0, 1.0, 1.0), 0.5);
/// // Result is approximately (0.735, 0.735, 0.735) — brighter than 0.5
/// assert!((mid.0 - 0.735).abs() < 0.01);
/// ```
#[must_use]
#[inline]
pub fn lerp_srgb(a: (f32, f32, f32), b: (f32, f32, f32), t: f32) -> (f32, f32, f32) {
    let al = (
        srgb_to_linear(a.0),
        srgb_to_linear(a.1),
        srgb_to_linear(a.2),
    );
    let bl = (
        srgb_to_linear(b.0),
        srgb_to_linear(b.1),
        srgb_to_linear(b.2),
    );
    let r = al.0 + (bl.0 - al.0) * t;
    let g = al.1 + (bl.1 - al.1) * t;
    let b_ch = al.2 + (bl.2 - al.2) * t;
    (linear_to_srgb(r), linear_to_srgb(g), linear_to_srgb(b_ch))
}

/// Gamma-aware linear interpolation for [`Vec3`] sRGB colors.
///
/// Equivalent to [`lerp_srgb`] but operates on `Vec3` (x=R, y=G, z=B).
///
/// # Examples
///
/// ```
/// use hisab::transforms::lerp_srgb_vec3;
/// use glam::Vec3;
///
/// let black = Vec3::ZERO;
/// let white = Vec3::ONE;
/// let mid = lerp_srgb_vec3(black, white, 0.5);
/// assert!((mid.x - 0.735).abs() < 0.01);
/// ```
#[must_use]
#[inline]
pub fn lerp_srgb_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    let al = srgb_to_linear_vec3(a);
    let bl = srgb_to_linear_vec3(b);
    let lerped = al + (bl - al) * t;
    linear_to_srgb_vec3(lerped)
}

// ---------------------------------------------------------------------------
// Exposure / EV ↔ luminance conversion
// ---------------------------------------------------------------------------

/// Convert exposure value (EV100) to luminance (cd/m²).
///
/// Formula: `L = 2^(EV - 3) * 12.5 / π`
///
/// Reference: Lagarde & de Rousiers 2014, "Moving Frostbite to PBR".
///
/// # Examples
///
/// ```
/// use hisab::transforms::ev100_to_luminance;
/// use std::f32::consts::PI;
///
/// // EV100 = 3 → L = 1.0 * 12.5 / π ≈ 3.979
/// let lum = ev100_to_luminance(3.0);
/// assert!((lum - 12.5 / PI).abs() < 0.001);
/// ```
#[must_use]
#[inline]
pub fn ev100_to_luminance(ev: f32) -> f32 {
    2.0_f32.powf(ev - 3.0) * 12.5 / std::f32::consts::PI
}

/// Convert luminance (cd/m²) to exposure value (EV100).
///
/// Formula: `EV = log2(L * π / 12.5) + 3`
///
/// This is the inverse of [`ev100_to_luminance`].
///
/// # Examples
///
/// ```
/// use hisab::transforms::luminance_to_ev100;
///
/// // Round-trip: EV → luminance → EV
/// let ev_orig = 6.0_f32;
/// let lum = hisab::transforms::ev100_to_luminance(ev_orig);
/// let ev_back = luminance_to_ev100(lum);
/// assert!((ev_back - ev_orig).abs() < 1e-5);
/// ```
#[must_use]
#[inline]
pub fn luminance_to_ev100(luminance: f32) -> f32 {
    (luminance * std::f32::consts::PI / 12.5).log2() + 3.0
}

/// Compute an exposure multiplier from EV100.
///
/// Formula: `exposure = 1 / (1.2 * 2^EV)`
///
/// Suitable for use as a linear scale factor applied to HDR scene values
/// before tone mapping.
///
/// # Examples
///
/// ```
/// use hisab::transforms::ev100_to_exposure;
///
/// // Higher EV → smaller multiplier (darker image)
/// let e0 = ev100_to_exposure(0.0);
/// let e6 = ev100_to_exposure(6.0);
/// assert!(e6 < e0);
/// ```
#[must_use]
#[inline]
pub fn ev100_to_exposure(ev: f32) -> f32 {
    1.0 / (1.2 * 2.0_f32.powf(ev))
}

// ---------------------------------------------------------------------------
// Spherical harmonics (L0-L2)
