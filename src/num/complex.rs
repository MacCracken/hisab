use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// FFT (Cooley-Tukey radix-2)
// ---------------------------------------------------------------------------

/// A complex number for FFT operations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    /// Create a new complex number.
    #[must_use]
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Complex number from a real value.
    #[must_use]
    #[inline]
    pub const fn from_real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    /// Magnitude (absolute value).
    #[must_use]
    #[inline]
    pub fn abs(self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// Complex conjugate.
    #[must_use]
    #[inline]
    pub const fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{}+{}i", self.re, self.im)
        } else {
            write!(f, "{}{}i", self.re, self.im)
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl std::ops::Div<f64> for Complex {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl std::ops::Neg for Complex {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl From<f64> for Complex {
    #[inline]
    fn from(re: f64) -> Self {
        Self { re, im: 0.0 }
    }
}

impl From<(f64, f64)> for Complex {
    #[inline]
    fn from((re, im): (f64, f64)) -> Self {
        Self { re, im }
    }
}
