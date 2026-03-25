use super::*;

// ---------------------------------------------------------------------------

/// Permutation table for noise functions (256 entries, doubled for wrapping).
const PERM: [u8; 512] = {
    let base: [u8; 256] = [
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30,
        69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94,
        252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171,
        168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60,
        211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1,
        216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
        164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
        126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
        213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39,
        253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
        242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
        192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
        138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
    ];
    let mut table = [0u8; 512];
    let mut i = 0;
    while i < 512 {
        table[i] = base[i & 255];
        i += 1;
    }
    table
};

#[inline]
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline]
fn grad2(hash: u8, x: f64, y: f64) -> f64 {
    match hash & 3 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        _ => -x - y,
    }
}

#[inline]
fn grad3(hash: u8, x: f64, y: f64, z: f64) -> f64 {
    match hash & 15 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        3 => -x - y,
        4 => x + z,
        5 => -x + z,
        6 => x - z,
        7 => -x - z,
        8 => y + z,
        9 => -y + z,
        10 => y - z,
        11 => -y - z,
        12 => y + x,
        13 => -y + z,
        14 => y - x,
        _ => -y - z,
    }
}

/// 2D Perlin noise. Returns a value in approximately [-1, 1].
#[must_use]
#[inline]
pub fn perlin_2d(x: f64, y: f64) -> f64 {
    let xi = x.floor() as i32 as usize & 255;
    let yi = y.floor() as i32 as usize & 255;
    let xf = x - x.floor();
    let yf = y - y.floor();
    let u = fade(xf);
    let v = fade(yf);

    let aa = PERM[PERM[xi] as usize + yi];
    let ab = PERM[PERM[xi] as usize + yi + 1];
    let ba = PERM[PERM[xi + 1] as usize + yi];
    let bb = PERM[PERM[xi + 1] as usize + yi + 1];

    let x1 = lerp(grad2(aa, xf, yf), grad2(ba, xf - 1.0, yf), u);
    let x2 = lerp(grad2(ab, xf, yf - 1.0), grad2(bb, xf - 1.0, yf - 1.0), u);
    lerp(x1, x2, v)
}

/// 3D Perlin noise. Returns a value in approximately [-1, 1].
#[must_use]
#[inline]
pub fn perlin_3d(x: f64, y: f64, z: f64) -> f64 {
    let xi = x.floor() as i32 as usize & 255;
    let yi = y.floor() as i32 as usize & 255;
    let zi = z.floor() as i32 as usize & 255;
    let xf = x - x.floor();
    let yf = y - y.floor();
    let zf = z - z.floor();
    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let aaa = PERM[PERM[PERM[xi] as usize + yi] as usize + zi];
    let aba = PERM[PERM[PERM[xi] as usize + yi + 1] as usize + zi];
    let aab = PERM[PERM[PERM[xi] as usize + yi] as usize + zi + 1];
    let abb = PERM[PERM[PERM[xi] as usize + yi + 1] as usize + zi + 1];
    let baa = PERM[PERM[PERM[xi + 1] as usize + yi] as usize + zi];
    let bba = PERM[PERM[PERM[xi + 1] as usize + yi + 1] as usize + zi];
    let bab = PERM[PERM[PERM[xi + 1] as usize + yi] as usize + zi + 1];
    let bbb = PERM[PERM[PERM[xi + 1] as usize + yi + 1] as usize + zi + 1];

    let x1 = lerp(grad3(aaa, xf, yf, zf), grad3(baa, xf - 1.0, yf, zf), u);
    let x2 = lerp(
        grad3(aba, xf, yf - 1.0, zf),
        grad3(bba, xf - 1.0, yf - 1.0, zf),
        u,
    );
    let y1 = lerp(x1, x2, v);

    let x1 = lerp(
        grad3(aab, xf, yf, zf - 1.0),
        grad3(bab, xf - 1.0, yf, zf - 1.0),
        u,
    );
    let x2 = lerp(
        grad3(abb, xf, yf - 1.0, zf - 1.0),
        grad3(bbb, xf - 1.0, yf - 1.0, zf - 1.0),
        u,
    );
    let y2 = lerp(x1, x2, v);

    lerp(y1, y2, w)
}

/// Fractal Brownian Motion — layered noise with octaves.
///
/// - `noise_fn`: base noise function (e.g., `perlin_2d` wrapped to take `(x, y)`).
/// - `x`, `y`: coordinates.
/// - `octaves`: number of noise layers (typically 4-8).
/// - `lacunarity`: frequency multiplier per octave (typically 2.0).
/// - `gain`: amplitude multiplier per octave (typically 0.5).
#[must_use]
#[inline]
pub fn fbm_2d(
    noise_fn: impl Fn(f64, f64) -> f64,
    x: f64,
    y: f64,
    octaves: usize,
    lacunarity: f64,
    gain: f64,
) -> f64 {
    let mut sum = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_amplitude = 0.0;

    for _ in 0..octaves {
        sum += amplitude * noise_fn(x * frequency, y * frequency);
        max_amplitude += amplitude;
        amplitude *= gain;
        frequency *= lacunarity;
    }

    sum / max_amplitude
}

// ---------------------------------------------------------------------------
// Spring dynamics

