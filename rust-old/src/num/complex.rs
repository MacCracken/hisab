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

    /// Squared magnitude (avoids sqrt).
    #[must_use]
    #[inline]
    pub fn norm_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Phase angle (argument) in radians.
    #[must_use]
    #[inline]
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Construct from polar form: `r * e^(iθ)`.
    #[must_use]
    #[inline]
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    /// Complex exponential: `e^(a+bi) = e^a (cos b + i sin b)`.
    #[must_use]
    #[inline]
    pub fn exp(self) -> Self {
        let r = self.re.exp();
        Self {
            re: r * self.im.cos(),
            im: r * self.im.sin(),
        }
    }

    /// Complex natural logarithm (principal branch).
    #[must_use]
    #[inline]
    pub fn ln(self) -> Self {
        Self {
            re: self.abs().ln(),
            im: self.arg(),
        }
    }

    /// Complex square root (principal branch).
    #[must_use]
    #[inline]
    pub fn sqrt(self) -> Self {
        let r = self.abs().sqrt();
        let theta = self.arg() / 2.0;
        Self::from_polar(r, theta)
    }

    /// Complex power: `self^n`.
    #[must_use]
    #[inline]
    pub fn powf(self, n: f64) -> Self {
        if self.norm_sq() < 1e-300 {
            return Self::new(0.0, 0.0);
        }
        let r = self.abs().powf(n);
        let theta = self.arg() * n;
        Self::from_polar(r, theta)
    }

    /// Complex sine.
    #[must_use]
    #[inline]
    pub fn sin(self) -> Self {
        Self {
            re: self.re.sin() * self.im.cosh(),
            im: self.re.cos() * self.im.sinh(),
        }
    }

    /// Complex cosine.
    #[must_use]
    #[inline]
    pub fn cos(self) -> Self {
        Self {
            re: self.re.cos() * self.im.cosh(),
            im: -self.re.sin() * self.im.sinh(),
        }
    }

    /// Multiplicative inverse: `1 / self`.
    #[must_use]
    #[inline]
    pub fn inv(self) -> Self {
        let d = self.norm_sq();
        Self {
            re: self.re / d,
            im: -self.im / d,
        }
    }

    /// Check if approximately zero within tolerance.
    #[must_use]
    #[inline]
    pub fn is_zero(self, tol: f64) -> bool {
        self.norm_sq() < tol * tol
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

impl std::ops::AddAssign for Complex {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl std::ops::SubAssign for Complex {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl std::ops::MulAssign for Complex {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        self.re = re;
        self.im = im;
    }
}

impl std::ops::MulAssign<f64> for Complex {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.re *= rhs;
        self.im *= rhs;
    }
}

impl std::ops::Mul<Complex> for f64 {
    type Output = Complex;
    #[inline]
    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            re: self * rhs.re,
            im: self * rhs.im,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    fn capprox(a: Complex, b: Complex) -> bool {
        (a - b).norm_sq() < 1e-16
    }

    #[test]
    fn norm_sq_3_4i() {
        assert!(approx(Complex::new(3.0, 4.0).norm_sq(), 25.0));
    }

    #[test]
    fn arg_1_plus_i() {
        assert!(approx(Complex::new(1.0, 1.0).arg(), FRAC_PI_4));
    }

    #[test]
    fn from_polar_i() {
        let z = Complex::from_polar(1.0, FRAC_PI_2);
        assert!(approx(z.re, 0.0));
        assert!(approx(z.im, 1.0));
    }

    #[test]
    fn exp_euler_formula() {
        let z = Complex::new(0.0, PI).exp() + Complex::from_real(1.0);
        assert!(approx(z.re, 0.0));
        assert!(approx(z.im, 0.0));
    }

    #[test]
    fn exp_zero_is_one() {
        let z = Complex::new(0.0, 0.0).exp();
        assert!(capprox(z, Complex::from_real(1.0)));
    }

    #[test]
    fn ln_exp_roundtrip() {
        let z = Complex::new(1.5, 0.8);
        let recovered = z.ln().exp();
        assert!(approx(recovered.re, z.re));
        assert!(approx(recovered.im, z.im));
    }

    #[test]
    fn exp_ln_roundtrip() {
        let z = Complex::new(0.5, 1.2);
        let recovered = z.exp().ln();
        assert!(approx(recovered.re, z.re));
        assert!(approx(recovered.im, z.im));
    }

    #[test]
    fn sqrt_squared_roundtrip() {
        let z = Complex::new(3.0, -2.0);
        let s = z.sqrt();
        let sq = s * s;
        assert!(approx(sq.re, z.re));
        assert!(approx(sq.im, z.im));
    }

    #[test]
    fn sqrt_neg_one_is_i() {
        let z = Complex::new(-1.0, 0.0).sqrt();
        assert!(approx(z.re, 0.0));
        assert!(approx(z.im, 1.0));
    }

    #[test]
    fn powf_vs_mul() {
        let z = Complex::new(2.0, 1.0);
        let via_powf = z.powf(2.0);
        let via_mul = z * z;
        assert!(approx(via_powf.re, via_mul.re));
        assert!(approx(via_powf.im, via_mul.im));
    }

    #[test]
    fn sin_zero() {
        assert!(capprox(
            Complex::new(0.0, 0.0).sin(),
            Complex::new(0.0, 0.0)
        ));
    }

    #[test]
    fn cos_zero() {
        assert!(capprox(
            Complex::new(0.0, 0.0).cos(),
            Complex::from_real(1.0)
        ));
    }

    #[test]
    fn sin_sq_plus_cos_sq() {
        let z = Complex::new(1.3, -0.7);
        let s = z.sin();
        let c = z.cos();
        let sum = s * s + c * c;
        assert!(approx(sum.re, 1.0));
        assert!(approx(sum.im, 0.0));
    }

    #[test]
    fn inv_product_is_one() {
        let z = Complex::new(3.0, 4.0);
        let product = z * z.inv();
        assert!(approx(product.re, 1.0));
        assert!(approx(product.im, 0.0));
    }

    #[test]
    fn is_zero_default() {
        assert!(Complex::default().is_zero(1e-12));
    }

    #[test]
    fn is_zero_nonzero() {
        assert!(!Complex::new(1.0, 0.0).is_zero(1e-12));
    }

    #[test]
    fn add_assign() {
        let mut z = Complex::new(1.0, 2.0);
        z += Complex::new(3.0, 4.0);
        assert!(capprox(z, Complex::new(4.0, 6.0)));
    }

    #[test]
    fn sub_assign() {
        let mut z = Complex::new(5.0, 6.0);
        z -= Complex::new(1.0, 2.0);
        assert!(capprox(z, Complex::new(4.0, 4.0)));
    }

    #[test]
    fn mul_assign() {
        let mut z = Complex::new(1.0, 1.0);
        z *= Complex::new(1.0, -1.0);
        assert!(capprox(z, Complex::new(2.0, 0.0)));
    }

    #[test]
    fn f64_mul_complex_commutativity() {
        let z = Complex::new(2.0, 3.0);
        let s = 5.0;
        let lhs = s * z;
        let rhs = z * s;
        assert!(capprox(lhs, rhs));
    }

    #[test]
    fn product_abs_is_product_of_abs() {
        let z = Complex::new(1.0, 2.0);
        let w = Complex::new(3.0, -1.0);
        assert!(approx((z * w).abs(), z.abs() * w.abs()));
    }

    #[test]
    fn conj_conj_is_identity() {
        let z = Complex::new(1.5, -2.3);
        assert!(capprox(z.conj().conj(), z));
    }
}
