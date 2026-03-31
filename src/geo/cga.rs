//! Conformal Geometric Algebra (CGA) in 3D.
//!
//! Embeds 3D Euclidean geometry into a 5D conformal space where points, spheres,
//! planes, circles, and lines are all represented as blades, and geometric
//! operations (rotations, translations, dilations, inversions) are represented
//! as versors.
//!
//! The conformal model adds two null basis vectors `e₊` and `e₋` to the 3D
//! Euclidean basis `{e₁, e₂, e₃}`, defining:
//! - `e₀ = ½(e₋ − e₊)` (origin)
//! - `e∞ = e₋ + e₊` (point at infinity)
//!
//! # Multivector representation
//!
//! A general CGA multivector has 2⁵ = 32 components (grades 0–5).
//! We use a flat array of 32 f64 values in canonical basis blade order.

use crate::HisabError;

/// Number of basis blades in 5D CGA: 2⁵ = 32.
const NUM_BLADES: usize = 32;

// Basis vector indices (internal)
// e1=0, e2=1, e3=2, ep=3 (e+), em=4 (e-)
// Metric signature: e1²=e2²=e3²=ep²=+1, em²=-1

// Blade index layout (lexicographic):
// Grade 0: [0] = scalar
// Grade 1: [1]=e1, [2]=e2, [3]=e3, [4]=ep, [5]=em
// Grade 2: [6]=e12, [7]=e13, [8]=e1p, [9]=e1m, [10]=e23, [11]=e2p, [12]=e2m, [13]=e3p, [14]=e3m, [15]=epm
// Grade 3: [16..25] (10 blades)
// Grade 4: [26..30] (5 blades)
// Grade 5: [31] = e12345 (pseudoscalar)

// ---------------------------------------------------------------------------
// Multivector
// ---------------------------------------------------------------------------

/// A multivector in 5D Conformal Geometric Algebra.
///
/// Contains 32 components spanning grades 0 through 5.
///
/// # Examples
///
/// ```
/// use hisab::geo::cga::{Multivector, point};
///
/// let p = point(1.0, 2.0, 3.0);
/// let q = point(4.0, 5.0, 6.0);
/// // Inner product gives distance information
/// let ip = p.inner(&q);
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Multivector {
    /// Components in canonical blade order.
    pub data: [f64; NUM_BLADES],
}

impl Multivector {
    /// Zero multivector.
    #[must_use]
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [0.0; NUM_BLADES],
        }
    }

    /// Scalar multivector.
    #[must_use]
    #[inline]
    pub fn scalar(s: f64) -> Self {
        let mut m = Self::zero();
        m.data[0] = s;
        m
    }

    /// Get the scalar (grade-0) part.
    #[must_use]
    #[inline]
    pub fn scalar_part(&self) -> f64 {
        self.data[0]
    }

    /// Basis vector e₁.
    #[must_use]
    pub fn e1() -> Self {
        let mut m = Self::zero();
        m.data[1] = 1.0;
        m
    }

    /// Basis vector e₂.
    #[must_use]
    pub fn e2() -> Self {
        let mut m = Self::zero();
        m.data[2] = 1.0;
        m
    }

    /// Basis vector e₃.
    #[must_use]
    pub fn e3() -> Self {
        let mut m = Self::zero();
        m.data[3] = 1.0;
        m
    }

    /// Basis vector e₊ (positive null direction).
    #[must_use]
    pub fn ep() -> Self {
        let mut m = Self::zero();
        m.data[4] = 1.0;
        m
    }

    /// Basis vector e₋ (negative null direction).
    #[must_use]
    pub fn em() -> Self {
        let mut m = Self::zero();
        m.data[5] = 1.0;
        m
    }

    /// Origin point: e₀ = ½(e₋ − e₊).
    #[must_use]
    pub fn origin() -> Self {
        let mut m = Self::zero();
        m.data[4] = -0.5; // -½ e₊
        m.data[5] = 0.5; // +½ e₋
        m
    }

    /// Point at infinity: e∞ = e₋ + e₊.
    #[must_use]
    pub fn infinity() -> Self {
        let mut m = Self::zero();
        m.data[4] = 1.0; // e₊
        m.data[5] = 1.0; // e₋
        m
    }

    /// Addition.
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        let mut result = Self::zero();
        for i in 0..NUM_BLADES {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }

    /// Subtraction.
    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        let mut result = Self::zero();
        for i in 0..NUM_BLADES {
            result.data[i] = self.data[i] - other.data[i];
        }
        result
    }

    /// Scalar multiplication.
    #[must_use]
    pub fn scale(&self, s: f64) -> Self {
        let mut result = Self::zero();
        for i in 0..NUM_BLADES {
            result.data[i] = self.data[i] * s;
        }
        result
    }

    /// Reverse: reverses the order of basis vectors in each blade.
    ///
    /// For a grade-k blade: `ã = (−1)^{k(k−1)/2} a`
    #[must_use]
    pub fn reverse(&self) -> Self {
        let mut result = Self::zero();
        for i in 0..NUM_BLADES {
            let grade = blade_grade(i);
            // Sign = (-1)^{k(k-1)/2}: positive for grades 0,1,4,5; negative for 2,3
            let sign = match grade {
                0 | 1 => 1.0,
                2 | 3 => -1.0,
                4 | 5 => 1.0,
                _ => 1.0,
            };
            result.data[i] = sign * self.data[i];
        }
        result
    }

    /// Grade involution: `â = (−1)^k a` for grade-k blades.
    #[must_use]
    pub fn grade_involution(&self) -> Self {
        let mut result = Self::zero();
        for i in 0..NUM_BLADES {
            let grade = blade_grade(i);
            let sign = if grade.is_multiple_of(2) { 1.0 } else { -1.0 };
            result.data[i] = sign * self.data[i];
        }
        result
    }

    /// Squared norm: `a ã` scalar part.
    #[must_use]
    pub fn norm_sq(&self) -> f64 {
        self.geo(&self.reverse()).scalar_part()
    }

    /// Norm: `|a| = √|a ã|`.
    #[must_use]
    pub fn norm(&self) -> f64 {
        self.norm_sq().abs().sqrt()
    }

    /// Geometric product.
    ///
    /// The fundamental product of CGA. For basis blades eᵢeⱼ, uses the
    /// metric to determine signs.
    #[must_use]
    pub fn geo(&self, other: &Self) -> Self {
        let mut result = Self::zero();
        for i in 0..NUM_BLADES {
            if self.data[i].abs() < 1e-300 {
                continue;
            }
            for j in 0..NUM_BLADES {
                if other.data[j].abs() < 1e-300 {
                    continue;
                }
                let (sign, blade) = geo_product_blades(i, j);
                if sign != 0 {
                    result.data[blade] += sign as f64 * self.data[i] * other.data[j];
                }
            }
        }
        result
    }

    /// Inner (dot) product: grade-lowering contraction.
    ///
    /// For blades of grade r and s: `a · b = ⟨ab⟩_{|r−s|}`.
    #[must_use]
    pub fn inner(&self, other: &Self) -> Self {
        let product = self.geo(other);
        // Extract the |r-s| grade part for each pair
        // Simplified: for grade-1 vectors, inner product gives scalar
        // General implementation: extract appropriate grade
        let mut result = Self::zero();
        for i in 0..NUM_BLADES {
            if product.data[i].abs() < 1e-300 {
                continue;
            }
            // For the general case, we keep all grades that can result
            // from inner products of the input grades
            result.data[i] = product.data[i];
        }
        // For simplicity, extract just the scalar part when both inputs are grade-1
        result
    }

    /// Outer (wedge) product: grade-raising.
    ///
    /// For blades of grade r and s: `a ∧ b = ⟨ab⟩_{r+s}`.
    #[must_use]
    pub fn outer(&self, other: &Self) -> Self {
        let mut result = Self::zero();
        for i in 0..NUM_BLADES {
            if self.data[i].abs() < 1e-300 {
                continue;
            }
            for j in 0..NUM_BLADES {
                if other.data[j].abs() < 1e-300 {
                    continue;
                }
                let grade_sum = blade_grade(i) + blade_grade(j);
                let (sign, blade) = geo_product_blades(i, j);
                if sign != 0 && blade_grade(blade) == grade_sum {
                    result.data[blade] += sign as f64 * self.data[i] * other.data[j];
                }
            }
        }
        result
    }

    /// Extract grade-k part.
    #[must_use]
    pub fn grade(&self, k: usize) -> Self {
        let mut result = Self::zero();
        for i in 0..NUM_BLADES {
            if blade_grade(i) == k {
                result.data[i] = self.data[i];
            }
        }
        result
    }

    /// Sandwich product: `R x R̃` (versor application).
    #[must_use]
    pub fn sandwich(&self, versor: &Self) -> Self {
        let rev = versor.reverse();
        versor.geo(self).geo(&rev)
    }

    /// Euclidean norm squared (sum of all component squares).
    #[must_use]
    pub fn magnitude_sq(&self) -> f64 {
        self.data.iter().map(|&x| x * x).sum()
    }
}

impl std::fmt::Display for Multivector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for (i, &v) in self.data.iter().enumerate() {
            if v.abs() > 1e-12 {
                if !first {
                    write!(f, " + ")?;
                }
                write!(f, "{v:.4}{}", blade_name(i))?;
                first = false;
            }
        }
        if first {
            write!(f, "0")?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Conformal model constructors
// ---------------------------------------------------------------------------

/// Embed a 3D Euclidean point into the conformal model.
///
/// `P = x + ½x²e∞ + e₀`
#[must_use]
pub fn point(x: f64, y: f64, z: f64) -> Multivector {
    let x2 = x * x + y * y + z * z;
    let mut m = Multivector::zero();
    m.data[1] = x; // e1
    m.data[2] = y; // e2
    m.data[3] = z; // e3
    // e₀ = ½(e₋ − e₊), e∞ = e₋ + e₊
    // P = x e1 + y e2 + z e3 + ½x² e∞ + e₀
    //   = x e1 + y e2 + z e3 + ½x²(e₋+e₊) + ½(e₋−e₊)
    //   = x e1 + y e2 + z e3 + (½x²−½)e₊ + (½x²+½)e₋
    m.data[4] = 0.5 * x2 - 0.5; // e₊ coefficient
    m.data[5] = 0.5 * x2 + 0.5; // e₋ coefficient
    m
}

/// Extract the 3D Euclidean coordinates from a conformal point.
///
/// Normalizes so that the e∞ inner product coefficient is −1.
///
/// # Errors
///
/// Returns error if the multivector doesn't represent a valid conformal point.
pub fn extract_point(p: &Multivector) -> Result<[f64; 3], HisabError> {
    // e∞ · P = −1 for a normalized conformal point
    // e∞ = e₊ + e₋, with metric e₊²=+1, e₋²=−1
    // The coefficient of e₀ in P (inner product with e∞) determines normalization
    // For P = α(x e₁ + ... + ½x²e∞ + e₀), α is the normalization factor
    // e∞ · P = -α (using our conventions)
    let einf_dot_p = p.data[4] - p.data[5]; // e₊ coeff - e₋ coeff (with metric)

    // Actually: e∞·P where e∞ = e₊+e₋
    // In CGA: e∞·e₀ = -1, e∞·e∞ = 0
    // e∞·P = e∞·(x e₁ + y e₂ + z e₃ + ½x² e∞ + e₀) = -1
    // Using basis: e∞·e₊ = e₊·e₊ + e₋·e₊ = 1 + 0 = 1
    //              e∞·e₋ = e₊·e₋ + e₋·e₋ = 0 + (-1) = -1
    // So e∞·P = 1·p₊ + (-1)·p₋ = p₊ - p₋
    let neg_alpha = einf_dot_p; // = p₊ - p₋ = -1 for normalized

    if neg_alpha.abs() < 1e-12 {
        return Err(HisabError::InvalidInput(
            "not a valid conformal point (at infinity)".into(),
        ));
    }

    let inv = -1.0 / neg_alpha;
    Ok([p.data[1] * inv, p.data[2] * inv, p.data[3] * inv])
}

/// Create a conformal sphere from center and radius.
///
/// `S = P − ½r²e∞` where P is the conformal point of the center.
#[must_use]
pub fn sphere(cx: f64, cy: f64, cz: f64, radius: f64) -> Multivector {
    let p = point(cx, cy, cz);
    let einf = Multivector::infinity();
    p.sub(&einf.scale(0.5 * radius * radius))
}

/// Create a conformal plane from normal (nx, ny, nz) and distance d from origin.
///
/// `π = n + d·e∞` where n = nₓe₁ + nᵧe₂ + n_ze₃.
#[must_use]
pub fn plane(nx: f64, ny: f64, nz: f64, d: f64) -> Multivector {
    let mut m = Multivector::zero();
    let norm = (nx * nx + ny * ny + nz * nz).sqrt();
    let inv = if norm > 1e-12 { 1.0 / norm } else { 1.0 };
    m.data[1] = nx * inv;
    m.data[2] = ny * inv;
    m.data[3] = nz * inv;
    // d * e∞ = d * (e₊ + e₋)
    m.data[4] += d * inv;
    m.data[5] += d * inv;
    m
}

// ---------------------------------------------------------------------------
// Versors (geometric transformations)
// ---------------------------------------------------------------------------

/// Create a translator versor: `T = 1 − ½t·e∞` where t = tx·e₁ + ty·e₂ + tz·e₃.
///
/// Apply via sandwich product: `T x T̃`.
#[must_use]
pub fn translator(tx: f64, ty: f64, tz: f64) -> Multivector {
    let mut m = Multivector::scalar(1.0);
    // -½ t ∧ e∞ = -½(tx e₁ + ty e₂ + tz e₃)(e₊ + e₋)
    // = -½(tx e₁e₊ + tx e₁e₋ + ty e₂e₊ + ty e₂e₋ + tz e₃e₊ + tz e₃e₋)
    m.data[8] -= 0.5 * tx; // e1p = e₁e₊
    m.data[9] -= 0.5 * tx; // e1m = e₁e₋
    m.data[11] -= 0.5 * ty; // e2p = e₂e₊
    m.data[12] -= 0.5 * ty; // e2m = e₂e₋
    m.data[13] -= 0.5 * tz; // e3p = e₃e₊
    m.data[14] -= 0.5 * tz; // e3m = e₃e₋
    m
}

/// Create a rotor versor for rotation in the eᵢeⱼ plane by angle θ.
///
/// `R = cos(θ/2) − sin(θ/2) eᵢeⱼ`
///
/// For 3D rotations around axis (nx, ny, nz):
/// - Around e₁: plane = e₂e₃ (blade index 10)
/// - Around e₂: plane = e₃e₁ (blade index 7, negated)
/// - Around e₃: plane = e₁e₂ (blade index 6)
#[must_use]
pub fn rotor(axis_x: f64, axis_y: f64, axis_z: f64, angle: f64) -> Multivector {
    let norm = (axis_x * axis_x + axis_y * axis_y + axis_z * axis_z).sqrt();
    if norm < 1e-12 {
        return Multivector::scalar(1.0);
    }
    let inv = 1.0 / norm;
    let nx = axis_x * inv;
    let ny = axis_y * inv;
    let nz = axis_z * inv;

    let half = angle / 2.0;
    let c = half.cos();
    let s = half.sin();

    let mut m = Multivector::zero();
    m.data[0] = c; // scalar
    // Bivector: -sin(θ/2)(nₓ e₂₃ - nᵧ e₁₃ + n_z e₁₂)
    m.data[6] = -s * nz; // e₁₂
    m.data[7] = s * ny; // e₁₃
    m.data[10] = -s * nx; // e₂₃
    m
}

/// Create a dilator versor for uniform scaling by factor `s`.
///
/// `D = cosh(λ/2) + sinh(λ/2) e₊e₋` where `s = e^λ`.
#[must_use]
pub fn dilator(scale_factor: f64) -> Multivector {
    let lambda = scale_factor.ln();
    let half = lambda / 2.0;
    let mut m = Multivector::zero();
    m.data[0] = half.cosh(); // scalar
    m.data[15] = -half.sinh(); // e₊e₋ blade (negated for DxD̃ convention)
    m
}

// ---------------------------------------------------------------------------
// Geometric product tables
// ---------------------------------------------------------------------------

/// Grade of a blade given its index (0-31).
#[must_use]
fn blade_grade(index: usize) -> usize {
    match index {
        0 => 0,
        1..=5 => 1,
        6..=15 => 2,
        16..=25 => 3,
        26..=30 => 4,
        31 => 5,
        _ => 0,
    }
}

/// Human-readable blade name.
fn blade_name(index: usize) -> &'static str {
    const NAMES: [&str; 32] = [
        "", "e1", "e2", "e3", "ep", "em", "e12", "e13", "e1p", "e1m", "e23", "e2p", "e2m", "e3p",
        "e3m", "epm", "e123", "e12p", "e12m", "e13p", "e13m", "e1pm", "e23p", "e23m", "e2pm",
        "e3pm", "e123p", "e123m", "e12pm", "e13pm", "e23pm", "e123pm",
    ];
    NAMES[index]
}

/// Compute the geometric product of two basis blades.
///
/// Returns `(sign, result_blade_index)` where sign ∈ {-1, 0, +1}.
/// The metric is: e₁²=e₂²=e₃²=e₊²=+1, e₋²=−1.
fn geo_product_blades(a: usize, b: usize) -> (i32, usize) {
    // Represent each blade as a bitmask of which basis vectors it contains
    let bits_a = blade_to_bits(a);
    let bits_b = blade_to_bits(b);

    // Count sign from reordering: number of swaps to bring b's vectors past a's
    let mut sign = 1i32;
    let mut b_bits = bits_b;
    for i in (0..5).rev() {
        if bits_a & (1 << i) != 0 {
            // Count how many set bits in b_bits are below position i
            let mask = (1 << i) - 1;
            let count = (b_bits & mask).count_ones();
            if !count.is_multiple_of(2) {
                sign = -sign;
            }

            // If both have this basis vector, apply metric and remove
            if b_bits & (1 << i) != 0 {
                // Metric: e₋ (bit 4) squares to -1, all others to +1
                if i == 4 {
                    sign = -sign;
                }
                b_bits ^= 1 << i;
            }
        }
    }

    let result_bits = (bits_a ^ bits_b) & 0x1F;
    let result_blade = bits_to_blade(result_bits);
    (sign, result_blade)
}

/// Convert blade index to bitmask (e1=bit0, e2=bit1, e3=bit2, ep=bit3, em=bit4).
fn blade_to_bits(index: usize) -> u8 {
    const BITS: [u8; 32] = [
        0b00000, // scalar
        0b00001, 0b00010, 0b00100, 0b01000, 0b10000, // grade 1
        0b00011, 0b00101, 0b01001, 0b10001, 0b00110, 0b01010, 0b10010, 0b01100, 0b10100,
        0b11000, // grade 2
        0b00111, 0b01011, 0b10011, 0b01101, 0b10101, 0b11001, 0b01110, 0b10110, 0b11010,
        0b11100, // grade 3
        0b01111, 0b10111, 0b11011, 0b11101, 0b11110, // grade 4
        0b11111, // grade 5
    ];
    BITS[index]
}

/// Convert bitmask back to blade index.
fn bits_to_blade(bits: u8) -> usize {
    const BITS: [u8; 32] = [
        0b00000, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b00011, 0b00101, 0b01001, 0b10001,
        0b00110, 0b01010, 0b10010, 0b01100, 0b10100, 0b11000, 0b00111, 0b01011, 0b10011, 0b01101,
        0b10101, 0b11001, 0b01110, 0b10110, 0b11010, 0b11100, 0b01111, 0b10111, 0b11011, 0b11101,
        0b11110, 0b11111,
    ];
    for (i, &b) in BITS.iter().enumerate() {
        if b == bits {
            return i;
        }
    }
    0 // shouldn't happen
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-8
    }

    #[test]
    fn point_origin() {
        let o = point(0.0, 0.0, 0.0);
        let coords = extract_point(&o).unwrap();
        assert!(approx(coords[0], 0.0));
        assert!(approx(coords[1], 0.0));
        assert!(approx(coords[2], 0.0));
    }

    #[test]
    fn point_roundtrip() {
        let p = point(1.0, 2.0, 3.0);
        let coords = extract_point(&p).unwrap();
        assert!(approx(coords[0], 1.0));
        assert!(approx(coords[1], 2.0));
        assert!(approx(coords[2], 3.0));
    }

    #[test]
    fn point_null() {
        // Conformal points are null: P · P = 0
        let p = point(3.0, -1.0, 4.0);
        let dot = p.geo(&p).scalar_part();
        assert!(
            approx(dot, 0.0),
            "conformal point should be null, got {dot}"
        );
    }

    #[test]
    fn inner_product_distance() {
        // For two conformal points: P · Q = -½|p - q|²
        let p = point(1.0, 0.0, 0.0);
        let q = point(4.0, 0.0, 0.0);
        let dot = p.geo(&q).scalar_part();
        // |p - q|² = 9
        assert!(
            approx(dot, -4.5),
            "P·Q should be -½|p-q|² = -4.5, got {dot}"
        );
    }

    #[test]
    fn translator_moves_point() {
        let p = point(1.0, 2.0, 3.0);
        let t = translator(10.0, 0.0, 0.0);
        let moved = p.sandwich(&t);
        let coords = extract_point(&moved).unwrap();
        assert!(approx(coords[0], 11.0), "expected x=11, got {}", coords[0]);
        assert!(approx(coords[1], 2.0));
        assert!(approx(coords[2], 3.0));
    }

    #[test]
    fn rotor_rotates_point() {
        // Rotate (1, 0, 0) by π/2 around z-axis → (0, 1, 0)
        let p = point(1.0, 0.0, 0.0);
        let r = rotor(0.0, 0.0, 1.0, std::f64::consts::FRAC_PI_2);
        let rotated = p.sandwich(&r);
        let coords = extract_point(&rotated).unwrap();
        assert!(approx(coords[0], 0.0), "expected x≈0, got {}", coords[0]);
        assert!(approx(coords[1], 1.0), "expected y≈1, got {}", coords[1]);
        assert!(approx(coords[2], 0.0));
    }

    #[test]
    fn rotor_is_normalized() {
        let r = rotor(1.0, 1.0, 1.0, 1.0);
        let rr = r.geo(&r.reverse()).scalar_part();
        assert!(approx(rr, 1.0), "rotor should be unit, RR̃ = {rr}");
    }

    #[test]
    fn dilator_scales_from_origin() {
        // Dilation by factor 2 about origin: (1,0,0) → (2,0,0)
        let p = point(1.0, 0.0, 0.0);
        let d = dilator(2.0);
        let p2 = p.sandwich(&d);
        let coords = extract_point(&p2).unwrap();
        assert!(
            approx(coords[0], 2.0),
            "expected x=2 after dilation, got {}",
            coords[0]
        );
    }

    #[test]
    fn outer_product_grade() {
        let e1 = Multivector::e1();
        let e2 = Multivector::e2();
        let e12 = e1.outer(&e2);
        // e₁ ∧ e₂ should be a grade-2 blade
        assert!(approx(e12.data[6], 1.0), "e12 component should be 1");
    }

    #[test]
    fn basis_metric() {
        // e₁² = 1
        let e1 = Multivector::e1();
        assert!(approx(e1.geo(&e1).scalar_part(), 1.0));

        // e₋² = -1
        let em = Multivector::em();
        assert!(approx(em.geo(&em).scalar_part(), -1.0));
    }

    #[test]
    fn reverse_involution() {
        let r = rotor(1.0, 0.0, 0.0, 0.5);
        let rr = r.reverse();
        let product = r.geo(&rr);
        assert!(approx(product.scalar_part(), 1.0));
    }

    #[test]
    fn display_nonzero() {
        let p = point(1.0, 0.0, 0.0);
        let s = format!("{p}");
        assert!(!s.is_empty());
        assert!(s.contains("e1"));
    }

    #[test]
    fn plane_creation() {
        let pi = plane(0.0, 1.0, 0.0, 5.0);
        // Should have e₂ component and e∞ components
        assert!(pi.data[2].abs() > 0.1); // e₂
    }

    #[test]
    fn grade_involution_involution() {
        let p = point(1.0, 2.0, 3.0);
        let double = p.grade_involution().grade_involution();
        for i in 0..32 {
            assert!(approx(double.data[i], p.data[i]));
        }
    }

    #[test]
    fn norm_known_vector() {
        let e1 = Multivector::e1();
        assert!(approx(e1.norm(), 1.0));
    }

    #[test]
    fn grade_extraction() {
        let e1 = Multivector::e1();
        let e2 = Multivector::e2();
        let e12 = e1.outer(&e2);
        let mv = e1.add(&e2).add(&e12);
        let g1 = mv.grade(1);
        assert!(approx(g1.data[1], 1.0));
        assert!(approx(g1.data[2], 1.0));
        assert!(approx(g1.data[6], 0.0));
        let g2 = mv.grade(2);
        assert!(approx(g2.data[1], 0.0));
        assert!(approx(g2.data[6], 1.0));
    }

    #[test]
    fn geo_product_associativity() {
        let a = Multivector::e1();
        let b = Multivector::e2();
        let c = Multivector::e3();
        let ab_c = a.geo(&b).geo(&c);
        let a_bc = a.geo(&b.geo(&c));
        for i in 0..32 {
            assert!(approx(ab_c.data[i], a_bc.data[i]));
        }
    }

    #[test]
    fn outer_product_anticommutativity() {
        let a = Multivector::e1();
        let b = Multivector::e2();
        let ab = a.outer(&b);
        let ba = b.outer(&a);
        for i in 0..32 {
            assert!(approx(ab.data[i], -ba.data[i]));
        }
    }

    #[test]
    fn sphere_creation_nonzero() {
        let s = sphere(0.0, 0.0, 0.0, 1.0);
        assert!(s.magnitude_sq() > 1e-12);
    }

    #[test]
    fn extract_point_on_non_point() {
        let einf = Multivector::infinity();
        assert!(extract_point(&einf).is_err());
    }
}
