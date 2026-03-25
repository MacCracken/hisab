//! Interval arithmetic for verified numerics.
//!
//! An [`Interval`] represents a closed range `[lo, hi]`. Arithmetic operations
//! propagate bounds correctly, guaranteeing that the true result is always
//! contained within the computed interval.

use std::ops;

/// A closed interval `[lo, hi]`.
///
/// All operations maintain the invariant `lo <= hi`.
///
/// # Examples
///
/// ```
/// use hisab::interval::Interval;
///
/// let a = Interval::new(1.0, 3.0);
/// let b = Interval::new(2.0, 4.0);
/// let sum = a + b;
/// assert!((sum.lo() - 3.0).abs() < 1e-12);
/// assert!((sum.hi() - 7.0).abs() < 1e-12);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    lo: f64,
    hi: f64,
}

impl Interval {
    /// Create a new interval. Swaps bounds if `lo > hi`.
    #[must_use]
    #[inline]
    pub fn new(a: f64, b: f64) -> Self {
        if a <= b {
            Self { lo: a, hi: b }
        } else {
            Self { lo: b, hi: a }
        }
    }

    /// Create a point interval `[x, x]`.
    #[must_use]
    #[inline]
    pub fn point(x: f64) -> Self {
        Self { lo: x, hi: x }
    }

    /// Lower bound.
    #[must_use]
    #[inline]
    pub fn lo(self) -> f64 {
        self.lo
    }

    /// Upper bound.
    #[must_use]
    #[inline]
    pub fn hi(self) -> f64 {
        self.hi
    }

    /// Width of the interval.
    #[must_use]
    #[inline]
    pub fn width(self) -> f64 {
        self.hi - self.lo
    }

    /// Midpoint of the interval.
    #[must_use]
    #[inline]
    pub fn midpoint(self) -> f64 {
        (self.lo + self.hi) * 0.5
    }

    /// Whether the interval contains a value.
    #[must_use]
    #[inline]
    pub fn contains(self, x: f64) -> bool {
        x >= self.lo && x <= self.hi
    }

    /// Whether this interval overlaps with another.
    #[must_use]
    #[inline]
    pub fn overlaps(self, other: Self) -> bool {
        self.lo <= other.hi && other.lo <= self.hi
    }

    /// Intersection of two intervals, or `None` if disjoint.
    #[must_use]
    #[inline]
    pub fn intersect(self, other: Self) -> Option<Self> {
        let lo = self.lo.max(other.lo);
        let hi = self.hi.min(other.hi);
        if lo <= hi {
            Some(Self { lo, hi })
        } else {
            None
        }
    }

    /// Union (hull) of two intervals — smallest interval containing both.
    #[must_use]
    #[inline]
    pub fn hull(self, other: Self) -> Self {
        Self {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }

    /// Whether the interval contains zero.
    #[must_use]
    #[inline]
    pub fn contains_zero(self) -> bool {
        self.lo <= 0.0 && self.hi >= 0.0
    }

    /// Absolute value of the interval.
    #[must_use]
    #[inline]
    pub fn abs(self) -> Self {
        if self.lo >= 0.0 {
            self
        } else if self.hi <= 0.0 {
            Self {
                lo: -self.hi,
                hi: -self.lo,
            }
        } else {
            Self {
                lo: 0.0,
                hi: self.lo.abs().max(self.hi.abs()),
            }
        }
    }

    /// Square of the interval.
    #[must_use]
    #[inline]
    pub fn sqr(self) -> Self {
        if self.lo >= 0.0 {
            Self {
                lo: self.lo * self.lo,
                hi: self.hi * self.hi,
            }
        } else if self.hi <= 0.0 {
            Self {
                lo: self.hi * self.hi,
                hi: self.lo * self.lo,
            }
        } else {
            Self {
                lo: 0.0,
                hi: self.lo.abs().max(self.hi.abs()).powi(2),
            }
        }
    }

    /// Square root of the interval (clamps lo to 0).
    #[must_use]
    #[inline]
    pub fn sqrt(self) -> Self {
        Self {
            lo: self.lo.max(0.0).sqrt(),
            hi: self.hi.max(0.0).sqrt(),
        }
    }
}

impl ops::Add for Interval {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            lo: self.lo + rhs.lo,
            hi: self.hi + rhs.hi,
        }
    }
}

impl ops::Sub for Interval {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            lo: self.lo - rhs.hi,
            hi: self.hi - rhs.lo,
        }
    }
}

impl ops::Mul for Interval {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let products = [
            self.lo * rhs.lo,
            self.lo * rhs.hi,
            self.hi * rhs.lo,
            self.hi * rhs.hi,
        ];
        let lo = products.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = products.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Self { lo, hi }
    }
}

impl ops::Div for Interval {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        if rhs.contains_zero() {
            // Division by interval containing zero → [-∞, ∞]
            Self {
                lo: f64::NEG_INFINITY,
                hi: f64::INFINITY,
            }
        } else {
            let inv = Interval::new(1.0 / rhs.hi, 1.0 / rhs.lo);
            self * inv
        }
    }
}

impl ops::Neg for Interval {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            lo: -self.hi,
            hi: -self.lo,
        }
    }
}

impl From<f64> for Interval {
    #[inline]
    fn from(val: f64) -> Self {
        Self::point(val)
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.lo, self.hi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-12;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn interval_add() {
        let a = Interval::new(1.0, 3.0);
        let b = Interval::new(2.0, 4.0);
        let r = a + b;
        assert!(approx(r.lo(), 3.0));
        assert!(approx(r.hi(), 7.0));
    }

    #[test]
    fn interval_sub() {
        let a = Interval::new(1.0, 3.0);
        let b = Interval::new(2.0, 4.0);
        let r = a - b;
        assert!(approx(r.lo(), -3.0));
        assert!(approx(r.hi(), 1.0));
    }

    #[test]
    fn interval_mul() {
        let a = Interval::new(-2.0, 3.0);
        let b = Interval::new(1.0, 4.0);
        let r = a * b;
        assert!(approx(r.lo(), -8.0));
        assert!(approx(r.hi(), 12.0));
    }

    #[test]
    fn interval_div() {
        let a = Interval::new(1.0, 4.0);
        let b = Interval::new(2.0, 8.0);
        let r = a / b;
        assert!(approx(r.lo(), 0.125));
        assert!(approx(r.hi(), 2.0));
    }

    #[test]
    fn interval_div_by_zero() {
        let a = Interval::new(1.0, 2.0);
        let b = Interval::new(-1.0, 1.0);
        let r = a / b;
        assert!(r.lo().is_infinite());
        assert!(r.hi().is_infinite());
    }

    #[test]
    fn interval_contains() {
        let i = Interval::new(1.0, 5.0);
        assert!(i.contains(3.0));
        assert!(i.contains(1.0));
        assert!(i.contains(5.0));
        assert!(!i.contains(0.0));
        assert!(!i.contains(6.0));
    }

    #[test]
    fn interval_width_midpoint() {
        let i = Interval::new(2.0, 8.0);
        assert!(approx(i.width(), 6.0));
        assert!(approx(i.midpoint(), 5.0));
    }

    #[test]
    fn interval_intersect() {
        let a = Interval::new(1.0, 5.0);
        let b = Interval::new(3.0, 7.0);
        let r = a.intersect(b).unwrap();
        assert!(approx(r.lo(), 3.0));
        assert!(approx(r.hi(), 5.0));

        let c = Interval::new(6.0, 7.0);
        assert!(a.intersect(c).is_none());
    }

    #[test]
    fn interval_hull() {
        let a = Interval::new(1.0, 3.0);
        let b = Interval::new(5.0, 7.0);
        let r = a.hull(b);
        assert!(approx(r.lo(), 1.0));
        assert!(approx(r.hi(), 7.0));
    }

    #[test]
    fn interval_neg() {
        let i = Interval::new(2.0, 5.0);
        let r = -i;
        assert!(approx(r.lo(), -5.0));
        assert!(approx(r.hi(), -2.0));
    }

    #[test]
    fn interval_abs() {
        let i = Interval::new(-3.0, 5.0);
        let r = i.abs();
        assert!(approx(r.lo(), 0.0));
        assert!(approx(r.hi(), 5.0));

        let neg = Interval::new(-5.0, -2.0);
        let r2 = neg.abs();
        assert!(approx(r2.lo, 2.0));
        assert!(approx(r2.hi, 5.0));
    }

    #[test]
    fn interval_sqr() {
        let i = Interval::new(-3.0, 2.0);
        let r = i.sqr();
        assert!(approx(r.lo(), 0.0));
        assert!(approx(r.hi(), 9.0));
    }

    #[test]
    fn interval_sqrt() {
        let i = Interval::new(4.0, 16.0);
        let r = i.sqrt();
        assert!(approx(r.lo(), 2.0));
        assert!(approx(r.hi(), 4.0));
    }

    #[test]
    fn interval_point() {
        let p = Interval::point(3.0);
        assert!(approx(p.width(), 0.0));
        assert!(p.contains(3.0));
    }

    #[test]
    fn interval_display() {
        let i = Interval::new(1.0, 2.0);
        assert_eq!(format!("{i}"), "[1, 2]");
    }

    #[test]
    fn interval_from_f64() {
        let i: Interval = 5.0.into();
        assert!(approx(i.lo(), 5.0));
        assert!(approx(i.hi(), 5.0));
    }

    #[test]
    fn interval_overlaps() {
        let a = Interval::new(1.0, 5.0);
        let b = Interval::new(3.0, 7.0);
        assert!(a.overlaps(b));
        let c = Interval::new(6.0, 8.0);
        assert!(!a.overlaps(c));
    }

    #[test]
    fn interval_swaps_bounds() {
        let i = Interval::new(5.0, 1.0);
        assert!(approx(i.lo(), 1.0));
        assert!(approx(i.hi(), 5.0));
    }
}
