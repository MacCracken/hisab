//! Lie groups and algebras for physics transformations.
//!
//! Provides:
//! - **SO(3,1)** — Lorentz group: boosts + rotations in Minkowski spacetime
//! - **SU(2)** — spinor rotations, Pauli algebra
//! - **SU(3)** — color charge algebra, Gell-Mann matrices
//! - **U(1)** — phase transformations
//! - Exponential map, commutators, structure constants, Casimir operators

use crate::HisabError;
use crate::num::{Complex, ComplexMatrix};

// ---------------------------------------------------------------------------
// U(1) — Phase group
// ---------------------------------------------------------------------------

/// U(1) phase element: `e^{iθ}`.
///
/// Represents a point on the unit circle in the complex plane.
/// Used for electromagnetic gauge transformations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct U1 {
    /// Phase angle in radians.
    pub theta: f64,
}

impl U1 {
    /// Create a U(1) element from a phase angle.
    #[must_use]
    #[inline]
    pub fn new(theta: f64) -> Self {
        Self { theta }
    }

    /// Identity element (zero phase).
    #[must_use]
    #[inline]
    pub fn identity() -> Self {
        Self { theta: 0.0 }
    }

    /// Group multiplication: `e^{iα} · e^{iβ} = e^{i(α+β)}`.
    #[must_use]
    #[inline]
    pub fn compose(self, other: Self) -> Self {
        Self {
            theta: self.theta + other.theta,
        }
    }

    /// Inverse: `e^{-iθ}`.
    #[must_use]
    #[inline]
    pub fn inv(self) -> Self {
        Self { theta: -self.theta }
    }

    /// As a complex number: `e^{iθ} = cos θ + i sin θ`.
    #[must_use]
    #[inline]
    pub fn to_complex(self) -> Complex {
        Complex::from_polar(1.0, self.theta)
    }

    /// 1×1 unitary matrix representation.
    #[must_use]
    pub fn to_matrix(self) -> ComplexMatrix {
        let mut m = ComplexMatrix::zeros(1, 1);
        m.set(0, 0, self.to_complex());
        m
    }

    /// Exponential map from Lie algebra (scalar) to group.
    #[must_use]
    #[inline]
    pub fn exp(generator: f64) -> Self {
        Self { theta: generator }
    }

    /// Logarithmic map from group to Lie algebra.
    #[must_use]
    #[inline]
    pub fn log(self) -> f64 {
        self.theta
    }
}

// ---------------------------------------------------------------------------
// SU(2) — Spin group
// ---------------------------------------------------------------------------

/// SU(2) group element as a unit quaternion `(a, b, c, d)` where `a² + b² + c² + d² = 1`.
///
/// `U = a·I + i(b·σ₁ + c·σ₂ + d·σ₃)` where σᵢ are Pauli matrices.
/// Double cover of SO(3): SU(2) → SO(3) is 2:1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Su2 {
    /// Scalar part (cos(θ/2)).
    pub w: f64,
    /// σ₁ coefficient.
    pub x: f64,
    /// σ₂ coefficient.
    pub y: f64,
    /// σ₃ coefficient.
    pub z: f64,
}

impl Su2 {
    /// Create an SU(2) element (automatically normalizes).
    #[must_use]
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        let norm = (w * w + x * x + y * y + z * z).sqrt();
        if norm < 1e-300 {
            return Self::identity();
        }
        let inv = 1.0 / norm;
        Self {
            w: w * inv,
            x: x * inv,
            y: y * inv,
            z: z * inv,
        }
    }

    /// Identity element.
    #[must_use]
    #[inline]
    pub fn identity() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Group multiplication (quaternion product).
    #[must_use]
    pub fn compose(self, other: Self) -> Self {
        let w = self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z;
        let x = self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y;
        let y = self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x;
        let z = self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w;
        Self { w, x, y, z }
    }

    /// Inverse (conjugate for unit quaternion).
    #[must_use]
    #[inline]
    pub fn inv(self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Exponential map: Lie algebra su(2) → SU(2).
    ///
    /// Input is a 3-vector `(ω₁, ω₂, ω₃)` in the Lie algebra.
    /// `exp(ω) = cos(|ω|) + sin(|ω|)/|ω| · (ω₁σ₁ + ω₂σ₂ + ω₃σ₃)`.
    #[must_use]
    pub fn exp(omega: [f64; 3]) -> Self {
        let norm = (omega[0] * omega[0] + omega[1] * omega[1] + omega[2] * omega[2]).sqrt();
        if norm < crate::EPSILON_F64 {
            return Self::identity();
        }
        let half = norm;
        let c = half.cos();
        let s = half.sin() / norm;
        Self {
            w: c,
            x: s * omega[0],
            y: s * omega[1],
            z: s * omega[2],
        }
    }

    /// Logarithmic map: SU(2) → su(2) Lie algebra.
    ///
    /// Returns the 3-vector `(ω₁, ω₂, ω₃)`.
    #[must_use]
    pub fn log(self) -> [f64; 3] {
        let w_clamped = self.w.clamp(-1.0, 1.0);
        let theta = w_clamped.acos();
        let sin_theta = theta.sin();
        if sin_theta.abs() < crate::EPSILON_F64 {
            return [0.0, 0.0, 0.0];
        }
        let s = theta / sin_theta;
        [s * self.x, s * self.y, s * self.z]
    }

    /// Convert to 2×2 unitary matrix.
    #[must_use]
    pub fn to_matrix(self) -> ComplexMatrix {
        let mut m = ComplexMatrix::zeros(2, 2);
        // U = [[a + id, -c + ib], [c + ib, a - id]]
        // where q = (w, x, y, z) maps to a=w, b=x, c=y, d=z
        m.set(0, 0, Complex::new(self.w, self.z));
        m.set(0, 1, Complex::new(-self.y, self.x));
        m.set(1, 0, Complex::new(self.y, self.x));
        m.set(1, 1, Complex::new(self.w, -self.z));
        m
    }

    /// Convert to 3×3 rotation matrix (SO(3) representation).
    #[must_use]
    pub fn to_rotation_matrix(self) -> [[f64; 3]; 3] {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        [
            [
                1.0 - 2.0 * (y * y + z * z),
                2.0 * (x * y - w * z),
                2.0 * (x * z + w * y),
            ],
            [
                2.0 * (x * y + w * z),
                1.0 - 2.0 * (x * x + z * z),
                2.0 * (y * z - w * x),
            ],
            [
                2.0 * (x * z - w * y),
                2.0 * (y * z + w * x),
                1.0 - 2.0 * (x * x + y * y),
            ],
        ]
    }

    /// Rotation angle (total rotation in radians).
    #[must_use]
    pub fn angle(self) -> f64 {
        2.0 * self.w.clamp(-1.0, 1.0).acos()
    }

    /// Rotation axis (unit vector). Returns `[0, 0, 1]` for zero rotation.
    #[must_use]
    pub fn axis(self) -> [f64; 3] {
        let norm = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if norm < crate::EPSILON_F64 {
            return [0.0, 0.0, 1.0];
        }
        let inv = 1.0 / norm;
        [self.x * inv, self.y * inv, self.z * inv]
    }
}

// ---------------------------------------------------------------------------
// SU(3) — Color group
// ---------------------------------------------------------------------------

/// Gell-Mann matrices λ₁ through λ₈ (generators of SU(3)).
///
/// The 3×3 Hermitian traceless matrices satisfying
/// `[λₐ, λ_b] = 2i fₐbc λ_c` where fₐbc are the structure constants.
///
/// # Errors
///
/// Returns error if `index` is not 1–8.
pub fn gell_mann(index: usize) -> Result<ComplexMatrix, HisabError> {
    let one = Complex::from_real(1.0);
    let neg = Complex::from_real(-1.0);
    let i = Complex::new(0.0, 1.0);
    let neg_i = Complex::new(0.0, -1.0);
    let inv_sqrt3 = Complex::from_real(1.0 / 3.0_f64.sqrt());
    let neg2_inv_sqrt3 = Complex::from_real(-2.0 / 3.0_f64.sqrt());

    let mut m = ComplexMatrix::zeros(3, 3);
    match index {
        1 => {
            // λ₁: off-diagonal 01/10
            m.set(0, 1, one);
            m.set(1, 0, one);
        }
        2 => {
            // λ₂: off-diagonal 01/10 with i
            m.set(0, 1, neg_i);
            m.set(1, 0, i);
        }
        3 => {
            // λ₃: diagonal (1, -1, 0)
            m.set(0, 0, one);
            m.set(1, 1, neg);
        }
        4 => {
            // λ₄: off-diagonal 02/20
            m.set(0, 2, one);
            m.set(2, 0, one);
        }
        5 => {
            // λ₅: off-diagonal 02/20 with i
            m.set(0, 2, neg_i);
            m.set(2, 0, i);
        }
        6 => {
            // λ₆: off-diagonal 12/21
            m.set(1, 2, one);
            m.set(2, 1, one);
        }
        7 => {
            // λ₇: off-diagonal 12/21 with i
            m.set(1, 2, neg_i);
            m.set(2, 1, i);
        }
        8 => {
            // λ₈: diagonal (1, 1, -2)/√3
            m.set(0, 0, inv_sqrt3);
            m.set(1, 1, inv_sqrt3);
            m.set(2, 2, neg2_inv_sqrt3);
        }
        _ => {
            return Err(HisabError::InvalidInput(
                "Gell-Mann index must be 1-8".into(),
            ));
        }
    }
    Ok(m)
}

/// All eight Gell-Mann matrices.
#[must_use]
pub fn gell_mann_matrices() -> [ComplexMatrix; 8] {
    std::array::from_fn(|i| gell_mann(i + 1).expect("valid index"))
}

/// SU(3) structure constants f_{abc}.
///
/// Returns the totally antisymmetric structure constants where
/// `[λₐ, λ_b] = 2i Σ_c f_{abc} λ_c`.
///
/// Only non-zero values are stored. Returns `f_{abc}` for 1-indexed a, b, c.
///
/// # Errors
///
/// Returns error if indices are not 1–8.
pub fn su3_structure_constant(a: usize, b: usize, c: usize) -> Result<f64, HisabError> {
    if a == 0 || a > 8 || b == 0 || b > 8 || c == 0 || c > 8 {
        return Err(HisabError::InvalidInput("SU(3) indices must be 1-8".into()));
    }

    // Non-zero structure constants (and antisymmetric permutations)
    let val = match (a, b, c) {
        (1, 2, 3) => 1.0,
        (1, 4, 7) => 0.5,
        (1, 5, 6) => -0.5,
        (2, 4, 6) => 0.5,
        (2, 5, 7) => 0.5,
        (3, 4, 5) => 0.5,
        (3, 6, 7) => -0.5,
        (4, 5, 8) => 3.0_f64.sqrt() / 2.0,
        (6, 7, 8) => 3.0_f64.sqrt() / 2.0,
        _ => {
            // Check antisymmetric permutations
            return su3_structure_antisymmetric(a, b, c);
        }
    };
    Ok(val)
}

/// Helper: compute structure constant via antisymmetry.
fn su3_structure_antisymmetric(a: usize, b: usize, c: usize) -> Result<f64, HisabError> {
    // Try all permutations and apply sign
    let perms = [
        (a, b, c, 1i32),
        (b, c, a, 1),
        (c, a, b, 1),
        (b, a, c, -1),
        (a, c, b, -1),
        (c, b, a, -1),
    ];

    for (pa, pb, pc, sign) in perms {
        let val = match (pa, pb, pc) {
            (1, 2, 3) => 1.0,
            (1, 4, 7) => 0.5,
            (1, 5, 6) => -0.5,
            (2, 4, 6) => 0.5,
            (2, 5, 7) => 0.5,
            (3, 4, 5) => 0.5,
            (3, 6, 7) => -0.5,
            (4, 5, 8) => 3.0_f64.sqrt() / 2.0,
            (6, 7, 8) => 3.0_f64.sqrt() / 2.0,
            _ => continue,
        };
        return Ok(val * sign as f64);
    }
    Ok(0.0)
}

// ---------------------------------------------------------------------------
// SO(3,1) — Lorentz group
// ---------------------------------------------------------------------------

/// A Lorentz transformation (element of SO(3,1)).
///
/// Stored as a 4×4 real matrix satisfying `Λᵀ η Λ = η` where η is the
/// Minkowski metric.
#[derive(Debug, Clone, PartialEq)]
pub struct Lorentz {
    /// 4×4 transformation matrix (row-major flat storage).
    data: [f64; 16],
}

impl Lorentz {
    /// Identity transformation.
    #[must_use]
    pub fn identity() -> Self {
        let mut data = [0.0; 16];
        data[0] = 1.0;
        data[5] = 1.0;
        data[10] = 1.0;
        data[15] = 1.0;
        Self { data }
    }

    /// Pure boost along x-axis with rapidity η.
    ///
    /// `Λ = [[cosh η, sinh η, 0, 0], [sinh η, cosh η, 0, 0], [0,0,1,0], [0,0,0,1]]`
    #[must_use]
    pub fn boost_x(rapidity: f64) -> Self {
        let ch = rapidity.cosh();
        let sh = rapidity.sinh();
        let mut data = [0.0; 16];
        data[0] = ch;
        data[1] = sh;
        data[4] = sh;
        data[5] = ch;
        data[10] = 1.0;
        data[15] = 1.0;
        Self { data }
    }

    /// Pure boost along y-axis with rapidity η.
    #[must_use]
    pub fn boost_y(rapidity: f64) -> Self {
        let ch = rapidity.cosh();
        let sh = rapidity.sinh();
        let mut data = [0.0; 16];
        data[0] = ch;
        data[2] = sh;
        data[8] = sh;
        data[10] = ch;
        data[5] = 1.0;
        data[15] = 1.0;
        Self { data }
    }

    /// Pure boost along z-axis with rapidity η.
    #[must_use]
    pub fn boost_z(rapidity: f64) -> Self {
        let ch = rapidity.cosh();
        let sh = rapidity.sinh();
        let mut data = [0.0; 16];
        data[0] = ch;
        data[3] = sh;
        data[12] = sh;
        data[15] = ch;
        data[5] = 1.0;
        data[10] = 1.0;
        Self { data }
    }

    /// Boost along an arbitrary unit direction with rapidity η.
    ///
    /// # Errors
    ///
    /// Returns error if direction is zero.
    pub fn boost(direction: [f64; 3], rapidity: f64) -> Result<Self, HisabError> {
        let norm = (direction[0] * direction[0]
            + direction[1] * direction[1]
            + direction[2] * direction[2])
            .sqrt();
        if norm < crate::EPSILON_F64 {
            return Err(HisabError::InvalidInput("boost direction is zero".into()));
        }
        let n = [
            direction[0] / norm,
            direction[1] / norm,
            direction[2] / norm,
        ];
        let ch = rapidity.cosh();
        let sh = rapidity.sinh();
        let gamma_m1 = ch - 1.0;

        let mut data = [0.0; 16];
        // Λ⁰₀ = cosh η
        data[0] = ch;
        // Λ⁰ᵢ = Λⁱ₀ = nᵢ sinh η
        for i in 0..3 {
            data[i + 1] = sh * n[i];
            data[(i + 1) * 4] = sh * n[i];
        }
        // Λⁱⱼ = δᵢⱼ + (cosh η - 1) nᵢ nⱼ
        for i in 0..3 {
            for j in 0..3 {
                let dij = if i == j { 1.0 } else { 0.0 };
                data[(i + 1) * 4 + (j + 1)] = dij + gamma_m1 * n[i] * n[j];
            }
        }
        Ok(Self { data })
    }

    /// Spatial rotation about axis `(nx, ny, nz)` by angle θ.
    ///
    /// # Errors
    ///
    /// Returns error if axis is zero.
    pub fn rotation(axis: [f64; 3], angle: f64) -> Result<Self, HisabError> {
        let norm = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        if norm < crate::EPSILON_F64 {
            return Err(HisabError::InvalidInput("rotation axis is zero".into()));
        }
        let n = [axis[0] / norm, axis[1] / norm, axis[2] / norm];
        let c = angle.cos();
        let s = angle.sin();

        let mut data = [0.0; 16];
        data[0] = 1.0; // time component unchanged
        for i in 0..3 {
            for j in 0..3 {
                let dij = if i == j { 1.0 } else { 0.0 };
                // Rodrigues' rotation formula
                let cross = match (i, j) {
                    (0, 1) => -n[2],
                    (0, 2) => n[1],
                    (1, 0) => n[2],
                    (1, 2) => -n[0],
                    (2, 0) => -n[1],
                    (2, 1) => n[0],
                    _ => 0.0,
                };
                data[(i + 1) * 4 + (j + 1)] = c * dij + (1.0 - c) * n[i] * n[j] + s * cross;
            }
        }
        Ok(Self { data })
    }

    /// Get matrix element Λ^μ_ν.
    #[must_use]
    #[inline]
    pub fn get(&self, mu: usize, nu: usize) -> f64 {
        debug_assert!(mu < 4 && nu < 4);
        self.data[mu * 4 + nu]
    }

    /// Group multiplication: `Λ₁ · Λ₂`.
    #[must_use]
    pub fn compose(&self, other: &Self) -> Self {
        let mut data = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.data[i * 4 + k] * other.data[k * 4 + j];
                }
                data[i * 4 + j] = sum;
            }
        }
        Self { data }
    }

    /// Inverse transformation.
    ///
    /// For Lorentz transformations: `Λ⁻¹ = η Λᵀ η`.
    #[must_use]
    pub fn inv(&self) -> Self {
        // η = diag(1, -1, -1, -1)
        let eta = [1.0, -1.0, -1.0, -1.0];
        let mut data = [0.0; 16];
        for mu in 0..4 {
            for nu in 0..4 {
                data[mu * 4 + nu] = eta[mu] * self.data[nu * 4 + mu] * eta[nu];
            }
        }
        Self { data }
    }

    /// Transform a 4-vector: `x'ᵘ = Λᵘ_ν xᵛ`.
    #[must_use]
    #[allow(clippy::needless_range_loop)]
    pub fn transform(&self, x: [f64; 4]) -> [f64; 4] {
        let mut out = [0.0; 4];
        for mu in 0..4 {
            for nu in 0..4 {
                out[mu] += self.data[mu * 4 + nu] * x[nu];
            }
        }
        out
    }

    /// The flat 4×4 matrix data (row-major).
    #[must_use]
    #[inline]
    pub fn as_matrix(&self) -> &[f64; 16] {
        &self.data
    }

    /// Verify that this is a valid Lorentz transformation: `Λᵀ η Λ ≈ η`.
    #[must_use]
    #[allow(clippy::needless_range_loop)]
    pub fn is_valid(&self, tol: f64) -> bool {
        let eta = [1.0, -1.0, -1.0, -1.0];
        for mu in 0..4 {
            for nu in 0..4 {
                let mut sum = 0.0;
                for rho in 0..4 {
                    sum += self.data[rho * 4 + mu] * eta[rho] * self.data[rho * 4 + nu];
                }
                let expected = if mu == nu { eta[mu] } else { 0.0 };
                if (sum - expected).abs() > tol {
                    return false;
                }
            }
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Lorentz Lie algebra generators
// ---------------------------------------------------------------------------

/// The six generators of the Lorentz algebra so(3,1).
///
/// Returns `Jᵢ` (rotations, i=1..3) and `Kᵢ` (boosts, i=1..3) as 4×4 real
/// matrices. Convention: `(Jᵢ)ᵘᵥ` and `(Kᵢ)ᵘᵥ`.
///
/// Index: 0–2 are rotations J₁, J₂, J₃; 3–5 are boosts K₁, K₂, K₃.
///
/// # Errors
///
/// Returns error if index is not 0–5.
pub fn lorentz_generator(index: usize) -> Result<[[f64; 4]; 4], HisabError> {
    let mut g = [[0.0; 4]; 4];
    match index {
        // J₁ = rotation around x: affects y,z components
        0 => {
            g[2][3] = -1.0;
            g[3][2] = 1.0;
        }
        // J₂ = rotation around y: affects x,z
        1 => {
            g[1][3] = 1.0;
            g[3][1] = -1.0;
        }
        // J₃ = rotation around z: affects x,y
        2 => {
            g[1][2] = -1.0;
            g[2][1] = 1.0;
        }
        // K₁ = boost along x
        3 => {
            g[0][1] = 1.0;
            g[1][0] = 1.0;
        }
        // K₂ = boost along y
        4 => {
            g[0][2] = 1.0;
            g[2][0] = 1.0;
        }
        // K₃ = boost along z
        5 => {
            g[0][3] = 1.0;
            g[3][0] = 1.0;
        }
        _ => {
            return Err(HisabError::InvalidInput(
                "Lorentz generator index must be 0-5".into(),
            ));
        }
    }
    Ok(g)
}

/// Exponential map for the Lorentz group.
///
/// Given a 6-vector `(ω₁, ω₂, ω₃, ξ₁, ξ₂, ξ₃)` where ωᵢ are rotation
/// parameters and ξᵢ are boost parameters, compute `exp(ωᵢJᵢ + ξᵢKᵢ)`.
///
/// # Errors
///
/// Returns error on internal computation failure.
#[allow(clippy::needless_range_loop)]
pub fn lorentz_exp(params: [f64; 6]) -> Result<Lorentz, HisabError> {
    // Build the 4×4 generator matrix
    let mut generator = [[0.0; 4]; 4];
    for (i, &p) in params.iter().enumerate() {
        let g = lorentz_generator(i)?;
        for mu in 0..4 {
            for nu in 0..4 {
                generator[mu][nu] += p * g[mu][nu];
            }
        }
    }

    // Matrix exponential via Taylor series (4×4 is small enough)
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        result[i][i] = 1.0;
    }
    let mut term = result;
    for k in 1..=30_u64 {
        let mut next = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for m in 0..4 {
                    next[i][j] += term[i][m] * generator[m][j];
                }
                next[i][j] /= k as f64;
            }
        }
        term = next;

        for i in 0..4 {
            for j in 0..4 {
                result[i][j] += term[i][j];
            }
        }

        // Check convergence
        let mut max = 0.0_f64;
        for row in &term {
            for &v in row {
                max = max.max(v.abs());
            }
        }
        if max < 1e-16 {
            break;
        }
    }

    let mut data = [0.0; 16];
    for i in 0..4 {
        for j in 0..4 {
            data[i * 4 + j] = result[i][j];
        }
    }
    Ok(Lorentz { data })
}

// ---------------------------------------------------------------------------
// Casimir operators
// ---------------------------------------------------------------------------

/// Compute the quadratic Casimir operator `C₂ = Σ Tₐ Tₐ` for a set of
/// generators represented as complex matrices.
///
/// For SU(N): `C₂ = (N²-1)/(2N) · I` in the fundamental representation.
///
/// # Errors
///
/// Returns error if generators list is empty or matrices are inconsistent.
pub fn casimir_quadratic(generators: &[ComplexMatrix]) -> Result<ComplexMatrix, HisabError> {
    if generators.is_empty() {
        return Err(HisabError::InvalidInput(
            "need at least one generator".into(),
        ));
    }
    let n = generators[0].rows();
    let mut c2 = ComplexMatrix::zeros(n, n);
    for g in generators {
        let g2 = g.mul_mat(g)?;
        c2 = c2.add(&g2)?;
    }
    Ok(c2)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::{commutator, pauli_matrices};
    use std::f64::consts::PI;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-8
    }

    // -- U(1) --

    #[test]
    fn u1_group_laws() {
        let a = U1::new(0.5);
        let b = U1::new(0.3);
        let ab = a.compose(b);
        assert!(approx(ab.theta, 0.8));
        let id = a.compose(a.inv());
        assert!(approx(id.theta, 0.0));
    }

    #[test]
    fn u1_exp_log() {
        let g = U1::exp(1.5);
        assert!(approx(g.log(), 1.5));
    }

    #[test]
    fn u1_unitary() {
        let u = U1::new(PI / 3.0);
        let m = u.to_matrix();
        assert!(m.is_unitary(1e-10));
    }

    // -- SU(2) --

    #[test]
    fn su2_identity() {
        let id = Su2::identity();
        let r = id.to_rotation_matrix();
        for (i, row) in r.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(approx(val, expected));
            }
        }
    }

    #[test]
    fn su2_exp_log_roundtrip() {
        let omega = [0.3, -0.5, 0.7];
        let g = Su2::exp(omega);
        let recovered = g.log();
        for i in 0..3 {
            assert!(approx(omega[i], recovered[i]));
        }
    }

    #[test]
    fn su2_group_closure() {
        let a = Su2::exp([0.1, 0.2, 0.3]);
        let b = Su2::exp([0.4, -0.1, 0.2]);
        let ab = a.compose(b);
        // Should be unit quaternion
        let norm = (ab.w * ab.w + ab.x * ab.x + ab.y * ab.y + ab.z * ab.z).sqrt();
        assert!(approx(norm, 1.0));
    }

    #[test]
    fn su2_inverse() {
        let g = Su2::exp([0.5, -0.3, 0.8]);
        let id = g.compose(g.inv());
        assert!(approx(id.w, 1.0));
        assert!(approx(id.x, 0.0));
        assert!(approx(id.y, 0.0));
        assert!(approx(id.z, 0.0));
    }

    #[test]
    fn su2_matrix_is_unitary() {
        let g = Su2::exp([0.5, -0.3, 0.8]);
        let m = g.to_matrix();
        assert!(m.is_unitary(1e-10));
    }

    #[test]
    fn su2_z_rotation_pi() {
        // Rotate by π around z: should negate x and y components
        let g = Su2::exp([0.0, 0.0, PI / 2.0]);
        let r = g.to_rotation_matrix();
        // R should be approximately [[-1,0,0],[0,-1,0],[0,0,1]]
        assert!(approx(r[0][0], -1.0));
        assert!(approx(r[1][1], -1.0));
        assert!(approx(r[2][2], 1.0));
    }

    // -- SU(3) / Gell-Mann --

    #[test]
    fn gell_mann_hermitian() {
        for m in &gell_mann_matrices() {
            assert!(m.is_hermitian(1e-10), "Gell-Mann matrix not Hermitian");
        }
    }

    #[test]
    fn gell_mann_traceless() {
        for m in &gell_mann_matrices() {
            let tr = m.trace().unwrap();
            assert!(tr.norm_sq() < 1e-20, "Gell-Mann matrix not traceless");
        }
    }

    #[test]
    fn su3_commutator_structure_constants() {
        // [λ₁, λ₂] = 2i f_{123} λ₃ = 2i · 1 · λ₃
        let gm = gell_mann_matrices();
        let comm = commutator(&gm[0], &gm[1]).unwrap();
        let expected = gm[2].scale(Complex::new(0.0, 2.0));
        let diff = comm.sub(&expected).unwrap();
        assert!(diff.frobenius_norm() < 1e-10, "[λ₁, λ₂] ≠ 2iλ₃");
    }

    #[test]
    fn su3_structure_antisymmetric_test() {
        // f_{123} = 1
        assert!(approx(su3_structure_constant(1, 2, 3).unwrap(), 1.0));
        // f_{213} = -1 (antisymmetric)
        assert!(approx(su3_structure_constant(2, 1, 3).unwrap(), -1.0));
        // f_{111} = 0
        assert!(approx(su3_structure_constant(1, 1, 1).unwrap(), 0.0));
    }

    // -- SO(3,1) / Lorentz --

    #[test]
    fn lorentz_identity() {
        let id = Lorentz::identity();
        assert!(id.is_valid(1e-10));
        let x = [1.0, 2.0, 3.0, 4.0];
        let y = id.transform(x);
        for i in 0..4 {
            assert!(approx(x[i], y[i]));
        }
    }

    #[test]
    fn lorentz_boost_is_valid() {
        let b = Lorentz::boost_x(0.5);
        assert!(b.is_valid(1e-10));
    }

    #[test]
    fn lorentz_boost_arbitrary_is_valid() {
        let b = Lorentz::boost([1.0, 1.0, 1.0], 0.8).unwrap();
        assert!(b.is_valid(1e-10));
    }

    #[test]
    fn lorentz_rotation_is_valid() {
        let r = Lorentz::rotation([0.0, 0.0, 1.0], PI / 4.0).unwrap();
        assert!(r.is_valid(1e-10));
    }

    #[test]
    fn lorentz_compose_inverse() {
        let b = Lorentz::boost([1.0, 0.0, 0.0], 0.6).unwrap();
        let bi = b.inv();
        let id = b.compose(&bi);
        assert!(id.is_valid(1e-10));
        // Should be close to identity
        for mu in 0..4 {
            for nu in 0..4 {
                let expected = if mu == nu { 1.0 } else { 0.0 };
                assert!(approx(id.get(mu, nu), expected), "Λ·Λ⁻¹ ≠ I at ({mu},{nu})");
            }
        }
    }

    #[test]
    fn lorentz_preserves_interval() {
        let b = Lorentz::boost([0.0, 1.0, 0.0], 1.2).unwrap();
        let x = [5.0, 1.0, 2.0, 3.0];
        let y = b.transform(x);

        // Minkowski interval: s² = t² - x² - y² - z²
        let s_before = x[0] * x[0] - x[1] * x[1] - x[2] * x[2] - x[3] * x[3];
        let s_after = y[0] * y[0] - y[1] * y[1] - y[2] * y[2] - y[3] * y[3];
        assert!(approx(s_before, s_after));
    }

    #[test]
    fn lorentz_exp_recovers_boost() {
        // exp(η K₃) should give a boost along z
        let eta = 0.7;
        let lt = lorentz_exp([0.0, 0.0, 0.0, 0.0, 0.0, eta]).unwrap();
        let direct = Lorentz::boost_z(eta);
        for i in 0..16 {
            assert!(
                approx(lt.data[i], direct.data[i]),
                "exp(ηK₃) ≠ boost_z(η) at index {i}"
            );
        }
    }

    #[test]
    fn lorentz_exp_recovers_rotation() {
        // exp(θ J₃) should give rotation around z
        let theta = 0.9;
        let lt = lorentz_exp([0.0, 0.0, theta, 0.0, 0.0, 0.0]).unwrap();
        let direct = Lorentz::rotation([0.0, 0.0, 1.0], theta).unwrap();
        for i in 0..16 {
            assert!(
                approx(lt.data[i], direct.data[i]),
                "exp(θJ₃) ≠ rotation(z, θ) at index {i}"
            );
        }
    }

    // -- Casimir --

    #[test]
    fn casimir_su2() {
        // C₂(SU(2)) = (σ₁² + σ₂² + σ₃²) = 3I in the fundamental rep
        // But with generators T = σ/2: C₂ = 3/4 I
        let sigma = pauli_matrices();
        let half_sigma: Vec<ComplexMatrix> = sigma.iter().map(|s| s.scale_real(0.5)).collect();
        let c2 = casimir_quadratic(&half_sigma).unwrap();
        // Should be (3/4)I₂
        let expected = ComplexMatrix::identity(2).scale_real(0.75);
        assert!(c2.sub(&expected).unwrap().frobenius_norm() < 1e-10);
    }

    #[test]
    fn casimir_su3() {
        // C₂(SU(3)) with T = λ/2: should give (4/3)I₃
        let gm = gell_mann_matrices();
        let half_gm: Vec<ComplexMatrix> = gm.iter().map(|m| m.scale_real(0.5)).collect();
        let c2 = casimir_quadratic(&half_gm).unwrap();
        let expected = ComplexMatrix::identity(3).scale_real(4.0 / 3.0);
        assert!(
            c2.sub(&expected).unwrap().frobenius_norm() < 1e-10,
            "SU(3) Casimir ≠ (4/3)I₃"
        );
    }
}
