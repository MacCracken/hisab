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
