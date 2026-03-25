use super::*;

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


