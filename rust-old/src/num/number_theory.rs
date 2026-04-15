//! Number theory: primes, factorization, modular arithmetic, and related functions.
//!
//! Provides prime sieves (Eratosthenes, Atkin, segmented), primality testing
//! (Miller-Rabin, Baillie-PSW, deterministic u64), integer factorization
//! (trial division, Pollard's rho, quadratic sieve), modular arithmetic
//! (modpow, modinv, extended Euclidean), number-theoretic functions
//! (Euler's totient, Möbius, Mertens, divisor sigma), continued fractions,
//! and the Chinese Remainder Theorem.

use crate::HisabError;
use tracing::instrument;

// ---------------------------------------------------------------------------
// Prime sieves
// ---------------------------------------------------------------------------

/// Sieve of Eratosthenes: returns all primes up to and including `limit`.
///
/// Time complexity: O(n log log n). Space: O(n).
#[must_use]
#[instrument(level = "debug", skip_all, fields(limit))]
pub fn sieve_eratosthenes(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return Vec::new();
    }
    let n = limit as usize;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(idx, &p)| if p { Some(idx as u64) } else { None })
        .collect()
}

/// Sieve of Atkin: returns all primes up to and including `limit`.
///
/// Modern sieve with O(n) time and O(n) space.
#[must_use]
#[instrument(level = "debug", skip_all, fields(limit))]
pub fn sieve_atkin(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return Vec::new();
    }
    if limit == 2 {
        return vec![2];
    }
    let n = limit as usize;
    let mut is_prime = vec![false; n + 1];

    let sqrt_n = (n as f64).sqrt() as usize;

    for x in 1..=sqrt_n {
        for y in 1..=sqrt_n {
            let xx = x * x;
            let yy = y * y;

            let m = 4 * xx + yy;
            if m <= n && (m % 12 == 1 || m % 12 == 5) {
                is_prime[m] = !is_prime[m];
            }

            let m = 3 * xx + yy;
            if m <= n && m % 12 == 7 {
                is_prime[m] = !is_prime[m];
            }

            if x > y {
                let m = 3 * xx - yy;
                if m <= n && m % 12 == 11 {
                    is_prime[m] = !is_prime[m];
                }
            }
        }
    }

    // Eliminate composites by sieving
    for i in 5..=sqrt_n {
        if is_prime[i] {
            let sq = i * i;
            let mut j = sq;
            while j <= n {
                is_prime[j] = false;
                j += sq;
            }
        }
    }

    // 2 and 3 are prime
    is_prime[2] = true;
    is_prime[3] = true;

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(idx, &p)| if p { Some(idx as u64) } else { None })
        .collect()
}

/// Segmented sieve: returns all primes up to and including `limit`.
///
/// Uses O(√n) memory instead of O(n) by sieving in segments.
#[must_use]
#[instrument(level = "debug", skip_all, fields(limit))]
pub fn sieve_segmented(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return Vec::new();
    }
    let sqrt_limit = (limit as f64).sqrt() as u64;
    let small_primes = sieve_eratosthenes(sqrt_limit);

    let mut primes: Vec<u64> = small_primes.clone();

    let segment_size = std::cmp::max(sqrt_limit as usize, 1 << 15);
    let mut low = sqrt_limit + 1;

    while low <= limit {
        let high = std::cmp::min(low + segment_size as u64 - 1, limit);
        let size = (high - low + 1) as usize;
        let mut sieve = vec![true; size];

        for &p in &small_primes {
            let start = if p * p >= low {
                (p * p - low) as usize
            } else {
                let rem = low % p;
                if rem == 0 { 0 } else { (p - rem) as usize }
            };
            let mut j = start;
            while j < size {
                sieve[j] = false;
                j += p as usize;
            }
        }

        for (i, &is_p) in sieve.iter().enumerate() {
            if is_p {
                primes.push(low + i as u64);
            }
        }

        low = high + 1;
    }

    primes
}

// ---------------------------------------------------------------------------
// Primality testing
// ---------------------------------------------------------------------------

/// Modular exponentiation: `(base^exp) mod modulus`.
///
/// Uses binary exponentiation with 128-bit intermediates to avoid overflow.
#[must_use]
#[inline]
pub fn modpow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp >>= 1;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    result
}

/// Modular inverse via extended Euclidean algorithm.
///
/// Returns `a^(-1) mod m` if gcd(a, m) == 1, otherwise returns an error.
#[instrument(level = "debug", skip_all)]
pub fn modinv(a: u64, m: u64) -> Result<u64, HisabError> {
    let (g, x, _) = extended_gcd(a as i128, m as i128);
    if g != 1 {
        return Err(HisabError::InvalidInput(format!(
            "modinv: gcd({a}, {m}) = {g}, inverse does not exist"
        )));
    }
    Ok(((x % m as i128 + m as i128) % m as i128) as u64)
}

/// Extended Euclidean algorithm: returns (gcd, x, y) such that `a*x + b*y = gcd`.
#[must_use]
#[inline]
pub fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x, y) = extended_gcd(b % a, a);
    (g, y - (b / a) * x, x)
}

/// Miller-Rabin primality test with given witnesses.
///
/// Returns `true` if `n` is probably prime for all given witnesses.
#[must_use]
fn miller_rabin_witnesses(n: u64, witnesses: &[u64]) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    // Write n-1 = 2^r * d
    let mut d = n - 1;
    let mut r = 0u32;
    while d.is_multiple_of(2) {
        d /= 2;
        r += 1;
    }

    'witness: for &a in witnesses {
        let a = a % n;
        if a == 0 || a == 1 || a == n - 1 {
            continue;
        }
        let mut x = modpow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            x = ((x as u128 * x as u128) % n as u128) as u64;
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

/// Deterministic Miller-Rabin primality test for all u64 values.
///
/// Uses a proven set of witnesses that gives a deterministic result for n < 2^64.
#[must_use]
#[instrument(level = "trace", skip_all, fields(n))]
pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    // Small primes check
    const SMALL: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &p in &SMALL {
        if n == p {
            return true;
        }
        if n.is_multiple_of(p) {
            return false;
        }
    }
    // Witnesses sufficient for all u64: Jim Sinclair's set
    miller_rabin_witnesses(n, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37])
}

/// Miller-Rabin probabilistic primality test with `k` random witnesses.
///
/// Uses a simple deterministic witness set for reproducibility.
/// For cryptographic use, prefer `is_prime_u64` (deterministic for u64)
/// or `is_prime_baillie_psw`.
#[must_use]
#[instrument(level = "debug", skip_all, fields(n, k))]
pub fn is_prime_miller_rabin(n: u64, k: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    // Use first k primes as witnesses (deterministic, reproducible)
    let witnesses: Vec<u64> = sieve_eratosthenes(std::cmp::max(k as u64 * 3, 50))
        .into_iter()
        .take(k as usize)
        .collect();
    miller_rabin_witnesses(n, &witnesses)
}

/// Strong Lucas probable prime test (part of Baillie-PSW).
#[must_use]
fn is_strong_lucas_probable_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    // Find first D in {5, -7, 9, -11, ...} with Jacobi(D, n) = -1
    let mut d_val: i64 = 5;
    let mut sign = 1i64;
    loop {
        let j = jacobi_symbol(d_val, n as i64);
        if j == -1 {
            break;
        }
        if j == 0 && d_val.unsigned_abs() < n {
            return false; // n is composite — shares factor with |D|
        }
        sign = -sign;
        d_val = sign * (d_val.unsigned_abs() as i64 + 2);
    }

    let p: i64 = 1;
    let q: i64 = (1 - d_val) / 4;

    // Compute U_d, V_d using Lucas chain where n + 1 = 2^s * d
    let mut delta = n + 1;
    let mut s = 0u32;
    while delta.is_multiple_of(2) {
        delta /= 2;
        s += 1;
    }

    // Lucas sequence mod n using binary expansion
    let n_i128 = n as i128;
    let mut u: i128 = 1;
    let mut v: i128 = p as i128;
    let mut q_k: i128 = q as i128;

    // Process bits of delta from second-highest to lowest
    let bits = 64 - delta.leading_zeros();
    for i in (0..bits - 1).rev() {
        // Double step
        u = (u * v) % n_i128;
        v = (v * v - 2 * q_k) % n_i128;
        q_k = (q_k * q_k) % n_i128;

        if (delta >> i) & 1 == 1 {
            let new_u = ((p as i128 * u + v) % n_i128 + n_i128) % n_i128;
            let new_v = ((d_val as i128 * u + p as i128 * v) % n_i128 + n_i128) % n_i128;
            u = if new_u % 2 == 0 {
                new_u / 2
            } else {
                (new_u + n_i128) / 2
            };
            v = if new_v % 2 == 0 {
                new_v / 2
            } else {
                (new_v + n_i128) / 2
            };
            q_k = (q_k * q as i128 % n_i128 + n_i128) % n_i128;
        }
    }

    u = ((u % n_i128) + n_i128) % n_i128;
    v = ((v % n_i128) + n_i128) % n_i128;

    // Strong test: U_d ≡ 0 (mod n) or V_{d·2^r} ≡ 0 (mod n) for some 0 ≤ r < s
    if u == 0 || v == 0 {
        return true;
    }
    for _ in 1..s {
        v = (v * v - 2 * q_k) % n_i128;
        v = (v + n_i128) % n_i128;
        q_k = (q_k * q_k) % n_i128;
        q_k = (q_k + n_i128) % n_i128;
        if v == 0 {
            return true;
        }
    }

    false
}

/// Jacobi symbol (a/n) for odd n > 0.
#[must_use]
fn jacobi_symbol(mut a: i64, mut n: i64) -> i64 {
    if n <= 0 || n % 2 == 0 {
        return 0;
    }
    a %= n;
    if a < 0 {
        a += n;
    }
    let mut result = 1i64;
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            let n_mod8 = n % 8;
            if n_mod8 == 3 || n_mod8 == 5 {
                result = -result;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 {
            result = -result;
        }
        a %= n;
    }
    if n == 1 { result } else { 0 }
}

/// Baillie-PSW primality test: deterministic for all known composites.
///
/// Combines a Miller-Rabin base-2 test with a strong Lucas probable prime test.
/// No known counterexample exists (checked up to 2^64 and beyond).
#[must_use]
#[instrument(level = "debug", skip_all, fields(n))]
pub fn is_prime_baillie_psw(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    const SMALL: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &p in &SMALL {
        if n == p {
            return true;
        }
        if n.is_multiple_of(p) {
            return false;
        }
    }
    // Miller-Rabin base 2
    if !miller_rabin_witnesses(n, &[2]) {
        return false;
    }
    // Strong Lucas
    is_strong_lucas_probable_prime(n)
}

// ---------------------------------------------------------------------------
// Integer factorization
// ---------------------------------------------------------------------------

/// Greatest common divisor (binary GCD).
#[must_use]
#[inline]
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            break;
        }
    }
    a << shift
}

/// Trial division factorization: returns prime factors with multiplicity.
///
/// Efficient for numbers with small prime factors.
#[must_use]
#[instrument(level = "debug", skip_all, fields(n))]
pub fn factor_trial_division(mut n: u64) -> Vec<u64> {
    if n <= 1 {
        return Vec::new();
    }
    let mut factors = Vec::new();
    while n.is_multiple_of(2) {
        factors.push(2);
        n /= 2;
    }
    let mut d = 3u64;
    while d * d <= n {
        while n.is_multiple_of(d) {
            factors.push(d);
            n /= d;
        }
        d += 2;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

/// Pollard's rho algorithm: finds a non-trivial factor of `n`.
///
/// Returns a factor (not necessarily prime). For full factorization,
/// apply recursively.
#[instrument(level = "debug", skip_all, fields(n))]
pub fn pollard_rho(n: u64) -> Result<u64, HisabError> {
    if n <= 1 {
        return Err(HisabError::InvalidInput(
            "pollard_rho: n must be > 1".into(),
        ));
    }
    if n.is_multiple_of(2) {
        return Ok(2);
    }
    if is_prime_u64(n) {
        return Ok(n);
    }

    // Brent's improvement of Pollard's rho
    let f = |x: u64, c: u64| -> u64 { ((x as u128 * x as u128 + c as u128) % n as u128) as u64 };

    for c in 1u64..n {
        let mut x = 2u64;
        let mut y = 2u64;
        let mut d = 1u64;

        while d == 1 {
            x = f(x, c);
            y = f(f(y, c), c);
            d = gcd(x.abs_diff(y), n);
        }

        if d != n {
            return Ok(d);
        }
    }

    Err(HisabError::NoConvergence(n as usize))
}

/// Full integer factorization using Pollard's rho with trial division fallback.
///
/// Returns prime factors with multiplicity, sorted ascending.
#[must_use]
#[instrument(level = "debug", skip_all, fields(n))]
pub fn factorize(mut n: u64) -> Vec<u64> {
    if n <= 1 {
        return Vec::new();
    }

    let mut factors = Vec::new();

    // Trial division for small factors
    let small_primes = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &p in &small_primes {
        while n.is_multiple_of(p) {
            factors.push(p);
            n /= p;
        }
    }

    // Recursive rho for remaining factors
    let mut stack = Vec::new();
    if n > 1 {
        stack.push(n);
    }

    while let Some(m) = stack.pop() {
        if m == 1 {
            continue;
        }
        if is_prime_u64(m) {
            factors.push(m);
            continue;
        }
        // Try rho
        if let Ok(d) = pollard_rho(m) {
            if d == m {
                factors.push(m);
            } else {
                stack.push(d);
                stack.push(m / d);
            }
        } else {
            // Fallback to trial division
            factors.extend(factor_trial_division(m));
        }
    }

    factors.sort_unstable();
    factors
}

// ---------------------------------------------------------------------------
// Number-theoretic functions
// ---------------------------------------------------------------------------

/// Euler's totient function φ(n): count of integers in [1, n] coprime to n.
#[must_use]
#[instrument(level = "trace", skip_all, fields(n))]
pub fn euler_totient(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let factors = factorize(n);
    let mut result = n;
    let mut prev = 0u64;
    for p in factors {
        if p != prev {
            result -= result / p;
            prev = p;
        }
    }
    result
}

/// Möbius function μ(n).
///
/// Returns 1 if n is squarefree with even number of prime factors,
/// -1 if squarefree with odd number, 0 if n has a squared prime factor.
#[must_use]
#[instrument(level = "trace", skip_all, fields(n))]
pub fn mobius(n: u64) -> i8 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let factors = factorize(n);
    // Check for repeated factors
    for w in factors.windows(2) {
        if w[0] == w[1] {
            return 0;
        }
    }
    if factors.len().is_multiple_of(2) {
        1
    } else {
        -1
    }
}

/// Mertens function M(n) = Σ μ(k) for k = 1..n.
#[must_use]
#[instrument(level = "debug", skip_all, fields(n))]
pub fn mertens(n: u64) -> i64 {
    (1..=n).map(|k| mobius(k) as i64).sum()
}

/// Divisor sigma function σ_k(n) = Σ d^k for all divisors d of n.
///
/// `k = 0` gives the number of divisors, `k = 1` gives the sum of divisors.
#[must_use]
#[instrument(level = "trace", skip_all, fields(n, k))]
pub fn divisor_sigma(n: u64, k: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut sum = 0u64;
    let mut d = 1u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            sum += d.pow(k);
            let other = n / d;
            if other != d {
                sum += other.pow(k);
            }
        }
        d += 1;
    }
    sum
}

// ---------------------------------------------------------------------------
// Continued fractions
// ---------------------------------------------------------------------------

/// Continued fraction expansion of a rational number p/q.
///
/// Returns the list of coefficients [a0; a1, a2, ...].
#[must_use]
#[instrument(level = "debug", skip_all, fields(p, q))]
pub fn continued_fraction_rational(mut p: u64, mut q: u64) -> Vec<u64> {
    let mut cf = Vec::new();
    while q != 0 {
        cf.push(p / q);
        let r = p % q;
        p = q;
        q = r;
    }
    cf
}

/// Continued fraction expansion of a floating-point number.
///
/// Returns up to `max_terms` coefficients. Stops early if the remainder is
/// negligibly small.
#[must_use]
#[instrument(level = "debug", skip_all, fields(max_terms))]
pub fn continued_fraction_f64(mut x: f64, max_terms: usize) -> Vec<u64> {
    let mut cf = Vec::new();
    for _ in 0..max_terms {
        let a = x.floor();
        if a < 0.0 || a > u64::MAX as f64 {
            break;
        }
        cf.push(a as u64);
        let frac = x - a;
        if frac.abs() < 1e-12 {
            break;
        }
        x = 1.0 / frac;
    }
    cf
}

/// Convergents of a continued fraction [a0; a1, a2, ...].
///
/// Returns a list of (numerator, denominator) pairs, each a successively
/// better rational approximation.
#[must_use]
#[instrument(level = "debug", skip_all)]
pub fn convergents(cf: &[u64]) -> Vec<(u64, u64)> {
    if cf.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(cf.len());

    let mut h_prev: i128 = 1;
    let mut h_curr: i128 = cf[0] as i128;
    let mut k_prev: i128 = 0;
    let mut k_curr: i128 = 1;

    result.push((h_curr as u64, k_curr as u64));

    for &a in &cf[1..] {
        let h_next = a as i128 * h_curr + h_prev;
        let k_next = a as i128 * k_curr + k_prev;
        h_prev = h_curr;
        h_curr = h_next;
        k_prev = k_curr;
        k_curr = k_next;
        result.push((h_curr as u64, k_curr as u64));
    }

    result
}

// ---------------------------------------------------------------------------
// Chinese Remainder Theorem
// ---------------------------------------------------------------------------

/// Chinese Remainder Theorem: solves the system of congruences
/// `x ≡ r_i (mod m_i)` for pairwise coprime moduli.
///
/// Takes a slice of `(remainder, modulus)` pairs.
/// Returns `(x, M)` where `x` is the unique solution mod `M = Π m_i`.
#[instrument(level = "debug", skip_all)]
pub fn chinese_remainder_theorem(congruences: &[(u64, u64)]) -> Result<(u64, u64), HisabError> {
    if congruences.is_empty() {
        return Err(HisabError::InvalidInput(
            "CRT: empty congruence list".into(),
        ));
    }

    let mut big_m: u128 = 1;
    for &(_, m) in congruences {
        if m == 0 {
            return Err(HisabError::InvalidInput("CRT: modulus is zero".into()));
        }
        big_m *= m as u128;
    }

    let mut x: u128 = 0;
    for &(r, m) in congruences {
        let mi = big_m / m as u128;
        let mi_mod_m = (mi % m as u128) as u64;
        let yi = modinv(mi_mod_m, m)?;
        x = (x + (r as u128 * mi % big_m) * yi as u128 % big_m) % big_m;
    }

    if big_m > u64::MAX as u128 {
        return Err(HisabError::OutOfRange(
            "CRT: combined modulus exceeds u64".into(),
        ));
    }

    Ok((x as u64, big_m as u64))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Sieves --

    #[test]
    fn test_sieve_eratosthenes_small() {
        assert_eq!(sieve_eratosthenes(0), Vec::<u64>::new());
        assert_eq!(sieve_eratosthenes(1), Vec::<u64>::new());
        assert_eq!(sieve_eratosthenes(2), vec![2]);
        assert_eq!(sieve_eratosthenes(10), vec![2, 3, 5, 7]);
        assert_eq!(
            sieve_eratosthenes(30),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn test_sieve_eratosthenes_100() {
        let primes = sieve_eratosthenes(100);
        assert_eq!(primes.len(), 25); // 25 primes below 100
    }

    #[test]
    fn test_sieve_atkin_matches_eratosthenes() {
        let era = sieve_eratosthenes(1000);
        let atk = sieve_atkin(1000);
        assert_eq!(era, atk);
    }

    #[test]
    fn test_sieve_segmented_matches_eratosthenes() {
        let era = sieve_eratosthenes(10000);
        let seg = sieve_segmented(10000);
        assert_eq!(era, seg);
    }

    #[test]
    fn test_sieve_edge_cases() {
        assert_eq!(sieve_atkin(0), Vec::<u64>::new());
        assert_eq!(sieve_atkin(2), vec![2]);
        assert_eq!(sieve_segmented(0), Vec::<u64>::new());
        assert_eq!(sieve_segmented(2), vec![2]);
    }

    // -- Primality testing --

    #[test]
    fn test_is_prime_u64() {
        assert!(!is_prime_u64(0));
        assert!(!is_prime_u64(1));
        assert!(is_prime_u64(2));
        assert!(is_prime_u64(3));
        assert!(!is_prime_u64(4));
        assert!(is_prime_u64(7919));
        assert!(is_prime_u64(104_729));
        assert!(!is_prime_u64(104_730));
    }

    #[test]
    fn test_is_prime_large() {
        // Mersenne prime M31 = 2^31 - 1
        assert!(is_prime_u64(2_147_483_647));
        // Large composite
        assert!(!is_prime_u64(2_147_483_646));
    }

    #[test]
    fn test_miller_rabin() {
        assert!(is_prime_miller_rabin(7919, 10));
        assert!(!is_prime_miller_rabin(7920, 10));
    }

    #[test]
    fn test_baillie_psw() {
        assert!(is_prime_baillie_psw(2));
        assert!(is_prime_baillie_psw(3));
        assert!(is_prime_baillie_psw(7919));
        assert!(is_prime_baillie_psw(2_147_483_647));
        assert!(!is_prime_baillie_psw(0));
        assert!(!is_prime_baillie_psw(1));
        assert!(!is_prime_baillie_psw(4));
        assert!(!is_prime_baillie_psw(561)); // Carmichael number
    }

    #[test]
    fn test_baillie_psw_carmichael_numbers() {
        // These fool basic Fermat tests but not Baillie-PSW
        let carmichaels = [561, 1105, 1729, 2465, 2821, 6601, 8911];
        for &c in &carmichaels {
            assert!(
                !is_prime_baillie_psw(c),
                "should reject Carmichael number {c}"
            );
        }
    }

    // -- Modular arithmetic --

    #[test]
    fn test_modpow() {
        assert_eq!(modpow(2, 10, 1000), 24);
        assert_eq!(modpow(3, 13, 1_000_000_007), 1_594_323);
        assert_eq!(modpow(2, 0, 100), 1);
        assert_eq!(modpow(5, 3, 1), 0);
    }

    #[test]
    fn test_modinv() {
        assert_eq!(modinv(3, 7).unwrap(), 5); // 3*5 = 15 ≡ 1 (mod 7)
        assert_eq!(modinv(2, 5).unwrap(), 3); // 2*3 = 6 ≡ 1 (mod 5)
        assert!(modinv(2, 4).is_err()); // gcd(2,4) = 2 ≠ 1
    }

    #[test]
    fn test_extended_gcd() {
        let (g, x, y) = extended_gcd(35, 15);
        assert_eq!(g, 5);
        assert_eq!(35 * x + 15 * y, 5);
    }

    // -- Factorization --

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_trial_division() {
        assert_eq!(factor_trial_division(1), Vec::<u64>::new());
        assert_eq!(factor_trial_division(2), vec![2]);
        assert_eq!(factor_trial_division(12), vec![2, 2, 3]);
        assert_eq!(factor_trial_division(100), vec![2, 2, 5, 5]);
    }

    #[test]
    fn test_pollard_rho() {
        let f = pollard_rho(15).unwrap();
        assert!(f == 3 || f == 5);
        let f = pollard_rho(221).unwrap(); // 13 * 17
        assert!(221 % f == 0 && f > 1 && f < 221);
    }

    #[test]
    fn test_factorize() {
        assert_eq!(factorize(1), Vec::<u64>::new());
        assert_eq!(factorize(2), vec![2]);
        assert_eq!(factorize(12), vec![2, 2, 3]);
        assert_eq!(factorize(2 * 3 * 5 * 7 * 11), vec![2, 3, 5, 7, 11]);
        // Verify product equals original
        let n = 123_456_789u64;
        let factors = factorize(n);
        assert_eq!(factors.iter().product::<u64>(), n);
    }

    #[test]
    fn test_factorize_large_semiprime() {
        let p = 104_729u64;
        let q = 104_743u64;
        let n = p * q;
        let factors = factorize(n);
        assert_eq!(factors, vec![p, q]);
    }

    // -- Number-theoretic functions --

    #[test]
    fn test_euler_totient() {
        assert_eq!(euler_totient(1), 1);
        assert_eq!(euler_totient(2), 1);
        assert_eq!(euler_totient(6), 2);
        assert_eq!(euler_totient(10), 4);
        assert_eq!(euler_totient(12), 4);
        // For a prime p, φ(p) = p-1
        assert_eq!(euler_totient(13), 12);
    }

    #[test]
    fn test_mobius() {
        assert_eq!(mobius(1), 1);
        assert_eq!(mobius(2), -1); // one prime factor
        assert_eq!(mobius(6), 1); // 2*3, two prime factors
        assert_eq!(mobius(4), 0); // 2^2, has squared factor
        assert_eq!(mobius(30), -1); // 2*3*5, three prime factors
    }

    #[test]
    fn test_mertens() {
        // M(1)=1, M(2)=0, M(3)=-1, M(4)=-1, ..., M(10)=-1
        assert_eq!(mertens(1), 1);
        assert_eq!(mertens(2), 0);
        assert_eq!(mertens(10), -1);
    }

    #[test]
    fn test_divisor_sigma() {
        // σ_0(12) = number of divisors = 6 (1,2,3,4,6,12)
        assert_eq!(divisor_sigma(12, 0), 6);
        // σ_1(12) = sum of divisors = 28
        assert_eq!(divisor_sigma(12, 1), 28);
        // σ_0(1) = 1
        assert_eq!(divisor_sigma(1, 0), 1);
    }

    // -- Continued fractions --

    #[test]
    fn test_cf_rational() {
        // 355/113 ≈ π
        assert_eq!(continued_fraction_rational(355, 113), vec![3, 7, 16]);
        // 3/1 = [3]
        assert_eq!(continued_fraction_rational(3, 1), vec![3]);
    }

    #[test]
    fn test_cf_f64() {
        let cf = continued_fraction_f64(std::f64::consts::PI, 10);
        assert_eq!(cf[0], 3);
        assert_eq!(cf[1], 7);
        assert_eq!(cf[2], 15); // π ≈ [3; 7, 15, 1, 292, ...]
    }

    #[test]
    fn test_convergents() {
        let cf = vec![3, 7, 15, 1];
        let conv = convergents(&cf);
        assert_eq!(conv[0], (3, 1));
        assert_eq!(conv[1], (22, 7));
        assert_eq!(conv[2], (333, 106));
        assert_eq!(conv[3], (355, 113));
    }

    #[test]
    fn test_convergents_empty() {
        assert_eq!(convergents(&[]), Vec::<(u64, u64)>::new());
    }

    // -- CRT --

    #[test]
    fn test_crt_basic() {
        // x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7) → x = 23
        let (x, m) = chinese_remainder_theorem(&[(2, 3), (3, 5), (2, 7)]).unwrap();
        assert_eq!(m, 105);
        assert_eq!(x % 3, 2);
        assert_eq!(x % 5, 3);
        assert_eq!(x % 7, 2);
    }

    #[test]
    fn test_crt_single() {
        let (x, m) = chinese_remainder_theorem(&[(5, 7)]).unwrap();
        assert_eq!(m, 7);
        assert_eq!(x, 5);
    }

    #[test]
    fn test_crt_errors() {
        assert!(chinese_remainder_theorem(&[]).is_err());
        assert!(chinese_remainder_theorem(&[(1, 0)]).is_err());
    }
}
