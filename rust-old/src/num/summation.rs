/// Kahan compensated summation — reduces floating-point accumulation error from O(n) to O(1).
///
/// Uses a running compensation variable to track lost low-order bits.
/// Prefer this over naive summation when accumulating many small values
/// (e.g., integration, long-running simulations).
#[must_use]
pub fn kahan_sum(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0; // compensation
    for &v in values {
        let y = v - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}

/// Neumaier compensated summation — improved Kahan that also handles
/// the case where the new addend is larger than the running sum.
///
/// Slightly more robust than [`kahan_sum`] with negligible overhead.
#[must_use]
pub fn neumaier_sum(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sum = values[0];
    let mut c = 0.0; // compensation
    for &v in &values[1..] {
        let t = sum + v;
        if sum.abs() >= v.abs() {
            c += (sum - t) + v;
        } else {
            c += (v - t) + sum;
        }
        sum = t;
    }
    sum + c
}
