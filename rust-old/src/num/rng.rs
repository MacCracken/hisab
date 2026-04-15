// ---------------------------------------------------------------------------
// PCG32 random number generator
// ---------------------------------------------------------------------------

/// A PCG32 (Permuted Congruential Generator) — fast, small-state, high-quality PRNG.
///
/// Deterministic and seedable — suitable for simulation replay.
#[derive(Debug, Clone)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    /// Create a new PCG32 with the given seed and stream.
    #[must_use]
    pub fn new(seed: u64, stream: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (stream << 1) | 1,
        };
        rng.next_u32();
        rng.state = rng.state.wrapping_add(seed);
        rng.next_u32();
        rng
    }

    /// Generate the next u32 value.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let old_state = self.state;
        self.state = old_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc);
        let xor_shifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;
        (xor_shifted >> rot) | (xor_shifted << (rot.wrapping_neg() & 31))
    }

    /// Generate a random f64 in [0, 1).
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u32() as f64) / (u32::MAX as f64 + 1.0)
    }

    /// Generate a random f32 in [0, 1).
    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() as f32) / (u32::MAX as f32 + 1.0)
    }

    /// Generate a random f64 in [lo, hi).
    #[inline]
    pub fn next_f64_range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next_f64()
    }

    /// Generate a random f64 from the standard normal distribution N(0,1).
    ///
    /// Uses the Box-Muller transform.
    #[inline]
    pub fn next_normal(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-300); // avoid log(0)
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }
}

// ---------------------------------------------------------------------------
// Quasi-random sequences
// ---------------------------------------------------------------------------

/// Halton sequence value for a given index and base.
///
/// Quasi-random low-discrepancy sequence. Use different prime bases for each
/// dimension (e.g., 2 for x, 3 for y, 5 for z).
///
/// Returns a value in \[0, 1).
#[must_use]
#[inline]
pub fn halton(index: u32, base: u32) -> f64 {
    let mut result = 0.0;
    let mut f = 1.0 / base as f64;
    let mut i = index;
    while i > 0 {
        result += f * (i % base) as f64;
        i /= base;
        f /= base as f64;
    }
    result
}

/// Generate a 2D Halton point at the given index.
///
/// Uses bases 2 and 3 (the standard choice for 2D).
#[must_use]
#[inline]
pub fn halton_2d(index: u32) -> (f64, f64) {
    (halton(index, 2), halton(index, 3))
}

/// Sobol sequence (dimension 0) — the Van der Corput sequence in base 2.
///
/// Generates a quasi-random value in \[0, 1) using bit-reversal.
/// For multi-dimensional Sobol, use different direction numbers per dimension;
/// this implementation provides the foundational 1D sequence.
#[must_use]
#[inline]
pub fn sobol(index: u32) -> f64 {
    let mut n = index;
    // Gray code: n ^= n >> 1
    n ^= n >> 1;
    // Bit-reverse
    n = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1);
    n = ((n & 0xCCCCCCCC) >> 2) | ((n & 0x33333333) << 2);
    n = ((n & 0xF0F0F0F0) >> 4) | ((n & 0x0F0F0F0F) << 4);
    n = ((n & 0xFF00FF00) >> 8) | ((n & 0x00FF00FF) << 8);
    n = n.rotate_left(16);
    n as f64 / (1u64 << 32) as f64
}
