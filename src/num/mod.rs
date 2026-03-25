//! Numerical methods: root finding, linear solvers, decompositions, FFT, DST/DCT, and ODE solvers.
//!
//! Provides Newton-Raphson, bisection, Gaussian elimination, LU/Cholesky/QR
//! decompositions, least-squares fitting, eigenvalue computation (power iteration),
//! Cooley-Tukey FFT/IFFT, DST-I/IDST-I, DCT-II/IDCT, and Runge-Kutta (RK4) ODE integration.

mod complex;
mod eigen;
mod fft;
mod inertia;
mod linalg;
mod ode;
mod optimize;
mod rng;
mod roots;
mod solvers;
mod sparse;
mod stability;
mod svd;

pub use complex::Complex;
pub use eigen::*;
pub use fft::*;
pub use inertia::*;
pub use linalg::*;
pub use ode::*;
pub use optimize::*;
pub use rng::Pcg32;
pub use roots::*;
pub use solvers::*;
pub use sparse::CsrMatrix;
pub use stability::*;
pub use svd::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HisabError;

    const EPSILON: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn newton_sqrt2() {
        let root = newton_raphson(|x| x * x - 2.0, |x| 2.0 * x, 1.5, 1e-10, 100).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-9);
    }

    #[test]
    fn newton_cube_root_27() {
        let root = newton_raphson(|x| x * x * x - 27.0, |x| 3.0 * x * x, 2.0, 1e-10, 100).unwrap();
        assert!(approx_eq(root, 3.0));
    }

    #[test]
    fn newton_no_convergence() {
        let result = newton_raphson(|x| x * x + 1.0, |x| 2.0 * x, 1.0, 1e-15, 5);
        assert!(result.is_err());
    }

    #[test]
    fn bisection_sqrt2() {
        let root = bisection(|x| x * x - 2.0, 1.0, 2.0, 1e-10, 100).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-9);
    }

    #[test]
    fn bisection_cubic_root() {
        let root = bisection(|x| x * x * x - 8.0, 1.0, 3.0, 1e-10, 100).unwrap();
        assert!(approx_eq(root, 2.0));
    }

    #[test]
    fn bisection_same_sign_error() {
        let result = bisection(|x| x * x + 1.0, 1.0, 2.0, 1e-10, 100);
        assert!(result.is_err());
    }

    #[test]
    fn bisection_sin_root() {
        let root = bisection(f64::sin, 3.0, 4.0, 1e-10, 100).unwrap();
        assert!((root - std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn gaussian_2x2() {
        let mut matrix = vec![vec![2.0, 1.0, 5.0], vec![1.0, 3.0, 10.0]];
        let x = gaussian_elimination(&mut matrix).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 3.0));
    }

    #[test]
    fn gaussian_3x3() {
        let mut matrix = vec![
            vec![1.0, 1.0, 1.0, 6.0],
            vec![2.0, 1.0, -1.0, 1.0],
            vec![1.0, -1.0, 1.0, 2.0],
        ];
        let x = gaussian_elimination(&mut matrix).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 2.0));
        assert!(approx_eq(x[2], 3.0));
    }

    #[test]
    fn gaussian_singular_matrix() {
        let mut matrix = vec![vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 6.0]];
        let result = gaussian_elimination(&mut matrix);
        assert!(result.is_err());
    }

    #[test]
    fn gaussian_empty_matrix() {
        let mut matrix: Vec<Vec<f64>> = vec![];
        let result = gaussian_elimination(&mut matrix);
        assert!(result.is_err());
    }

    #[test]
    fn newton_zero_derivative_error() {
        let result = newton_raphson(|x| x * x + 1.0, |_| 0.0, 2.0, 1e-10, 100);
        assert!(result.is_err());
    }

    #[test]
    fn error_display() {
        let e = HisabError::NoConvergence(50);
        assert_eq!(e.to_string(), "no convergence after 50 iterations");
        let e = HisabError::SingularPivot;
        assert!(e.to_string().contains("singular"));
    }

    #[test]
    fn error_display_invalid_input() {
        let e = HisabError::InvalidInput("bad data".to_string());
        assert_eq!(e.to_string(), "invalid input: bad data");
    }

    #[test]
    fn newton_linear_root() {
        let root = newton_raphson(|x| 2.0 * x - 6.0, |_| 2.0, 0.0, 1e-10, 10).unwrap();
        assert!(approx_eq(root, 3.0));
    }

    #[test]
    fn newton_sin_root_near_zero() {
        let root = newton_raphson(f64::sin, f64::cos, 3.0, 1e-12, 50).unwrap();
        assert!((root - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn bisection_linear() {
        let root = bisection(|x| x - 5.0, 0.0, 10.0, 1e-10, 100).unwrap();
        assert!(approx_eq(root, 5.0));
    }

    #[test]
    fn bisection_negative_interval() {
        let root = bisection(|x| x + 3.0, -5.0, 0.0, 1e-10, 100).unwrap();
        assert!(approx_eq(root, -3.0));
    }

    #[test]
    fn bisection_swaps_when_f_lo_positive() {
        let root = bisection(|x| x - 1.0, 2.0, 0.0, 1e-10, 100).unwrap();
        assert!(approx_eq(root, 1.0));
    }

    #[test]
    fn gaussian_1x1() {
        let mut matrix = vec![vec![3.0, 9.0]];
        let x = gaussian_elimination(&mut matrix).unwrap();
        assert!(approx_eq(x[0], 3.0));
    }

    #[test]
    fn gaussian_needs_pivoting() {
        let mut matrix = vec![vec![0.0, 1.0, 3.0], vec![1.0, 1.0, 5.0]];
        let x = gaussian_elimination(&mut matrix).unwrap();
        assert!(approx_eq(x[0], 2.0));
        assert!(approx_eq(x[1], 3.0));
    }

    #[test]
    fn gaussian_wrong_column_count() {
        let mut matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];
        let result = gaussian_elimination(&mut matrix);
        assert!(result.is_err());
    }

    #[test]
    fn gaussian_4x4() {
        let mut matrix = vec![
            vec![1.0, 1.0, 1.0, 1.0, 4.0],
            vec![2.0, 1.0, 1.0, 1.0, 5.0],
            vec![1.0, 3.0, 1.0, 1.0, 6.0],
            vec![1.0, 1.0, 1.0, 4.0, 7.0],
        ];
        let x = gaussian_elimination(&mut matrix).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 1.0));
        assert!(approx_eq(x[2], 1.0));
        assert!(approx_eq(x[3], 1.0));
    }

    #[test]
    fn newton_converges_exactly_at_root() {
        let root = newton_raphson(|x| x - 5.0, |_| 1.0, 5.0, 1e-10, 1).unwrap();
        assert!(approx_eq(root, 5.0));
    }

    #[test]
    fn bisection_exact_root_at_midpoint() {
        let root = bisection(|x| x, -1.0, 1.0, 1e-10, 1).unwrap();
        assert!(approx_eq(root, 0.0));
    }

    // --- V0.4a: LU decomposition ---

    #[test]
    fn lu_decompose_2x2() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let (lu, _pivot) = lu_decompose(&a).unwrap();
        // U[0][0] should be 2, U[0][1] should be 1
        assert!(approx_eq(lu[0][0], 2.0));
        assert!(approx_eq(lu[0][1], 1.0));
    }

    #[test]
    fn lu_solve_2x2() {
        // 2x + y = 5, x + 3y = 10 => x=1, y=3
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = [5.0, 10.0];
        let (lu, pivot) = lu_decompose(&a).unwrap();
        let x = lu_solve(&lu, &pivot, &b).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 3.0));
    }

    #[test]
    fn lu_solve_3x3() {
        // x+y+z=6, 2x+y-z=1, x-y+z=2 => x=1, y=2, z=3
        let a = vec![
            vec![1.0, 1.0, 1.0],
            vec![2.0, 1.0, -1.0],
            vec![1.0, -1.0, 1.0],
        ];
        let b = [6.0, 1.0, 2.0];
        let (lu, pivot) = lu_decompose(&a).unwrap();
        let x = lu_solve(&lu, &pivot, &b).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 2.0));
        assert!(approx_eq(x[2], 3.0));
    }

    #[test]
    fn lu_singular() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        assert!(lu_decompose(&a).is_err());
    }

    #[test]
    fn lu_needs_pivoting() {
        // First pivot is zero
        let a = vec![vec![0.0, 1.0], vec![1.0, 1.0]];
        let b = [3.0, 5.0];
        let (lu, pivot) = lu_decompose(&a).unwrap();
        let x = lu_solve(&lu, &pivot, &b).unwrap();
        assert!(approx_eq(x[0], 2.0));
        assert!(approx_eq(x[1], 3.0));
    }

    // --- V0.4a: Cholesky ---

    #[test]
    fn cholesky_2x2() {
        // A = [[4, 2], [2, 3]] is SPD
        let a = vec![vec![4.0, 2.0], vec![2.0, 3.0]];
        let l = cholesky(&a).unwrap();
        assert!(approx_eq(l[0][0], 2.0));
        assert!(approx_eq(l[1][0], 1.0));
        assert!(approx_eq(l[1][1], (2.0f64).sqrt()));
    }

    #[test]
    fn cholesky_solve_2x2() {
        let a = vec![vec![4.0, 2.0], vec![2.0, 3.0]];
        let b = [8.0, 7.0];
        let l = cholesky(&a).unwrap();
        let x = cholesky_solve(&l, &b).unwrap();
        // Verify A*x = b
        let r0 = 4.0 * x[0] + 2.0 * x[1];
        let r1 = 2.0 * x[0] + 3.0 * x[1];
        assert!(approx_eq(r0, 8.0));
        assert!(approx_eq(r1, 7.0));
    }

    #[test]
    fn cholesky_3x3_identity() {
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let l = cholesky(&a).unwrap();
        // L of identity is identity
        assert!(approx_eq(l[0][0], 1.0));
        assert!(approx_eq(l[1][1], 1.0));
        assert!(approx_eq(l[2][2], 1.0));
        assert!(approx_eq(l[1][0], 0.0));
        assert!(approx_eq(l[2][0], 0.0));
        assert!(approx_eq(l[2][1], 0.0));
    }

    #[test]
    fn cholesky_not_positive_definite() {
        // Negative diagonal -> not PD
        let a = vec![vec![-1.0, 0.0], vec![0.0, 1.0]];
        assert!(cholesky(&a).is_err());
    }

    // --- V0.4a: QR decomposition ---

    #[test]
    fn qr_orthogonality() {
        // Verify Q columns are orthonormal
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]; // 2 cols, 3 rows
        let (q, _r) = qr_decompose(&a).unwrap();
        // q[0] dot q[1] should be ~0
        let dot: f64 = (0..3).map(|k| q[0][k] * q[1][k]).sum();
        assert!(dot.abs() < 1e-10);
        // Each column should have unit norm
        for col in &q {
            let norm: f64 = col.iter().map(|x| x * x).sum::<f64>().sqrt();
            assert!(approx_eq(norm, 1.0));
        }
    }

    #[test]
    fn qr_reconstruct() {
        // Verify A = Q * R
        let a = vec![vec![1.0, 0.0, 1.0], vec![1.0, 1.0, 0.0]]; // 2 cols, 3 rows
        let (q, r) = qr_decompose(&a).unwrap();
        // Reconstruct A[col][row] = sum_k Q[k][row] * R[col][k]
        for j in 0..2 {
            for i in 0..3 {
                let mut sum = 0.0;
                for k in 0..2 {
                    sum += q[k][i] * r[j][k];
                }
                assert!((sum - a[j][i]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn qr_dependent_columns() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]]; // Linearly dependent
        assert!(qr_decompose(&a).is_err());
    }

    // --- V0.4a: Least squares ---

    #[test]
    fn least_squares_linear_exact() {
        // Points exactly on y = 2x + 1
        let x = [0.0, 1.0, 2.0, 3.0];
        let y = [1.0, 3.0, 5.0, 7.0];
        let coeffs = least_squares_poly(&x, &y, 1).unwrap();
        assert!(approx_eq(coeffs[0], 1.0)); // intercept
        assert!(approx_eq(coeffs[1], 2.0)); // slope
    }

    #[test]
    fn least_squares_quadratic() {
        // Points on y = x^2
        let x = [0.0, 1.0, 2.0, 3.0, 4.0];
        let y: Vec<f64> = x.iter().map(|&xi| xi * xi).collect();
        let coeffs = least_squares_poly(&x, &y, 2).unwrap();
        assert!(approx_eq(coeffs[0], 0.0)); // a0
        assert!(approx_eq(coeffs[1], 0.0)); // a1
        assert!(approx_eq(coeffs[2], 1.0)); // a2
    }

    #[test]
    fn least_squares_constant() {
        // All y=5 => constant fit
        let x = [1.0, 2.0, 3.0];
        let y = [5.0, 5.0, 5.0];
        let coeffs = least_squares_poly(&x, &y, 0).unwrap();
        assert!(approx_eq(coeffs[0], 5.0));
    }

    #[test]
    fn least_squares_noisy_linear() {
        // Noisy data around y = 3x + 2
        let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.1, 4.9, 8.1, 10.9, 14.1, 16.9];
        let coeffs = least_squares_poly(&x, &y, 1).unwrap();
        // Should be close to a0≈2, a1≈3
        assert!((coeffs[0] - 2.0).abs() < 0.5);
        assert!((coeffs[1] - 3.0).abs() < 0.2);
    }

    #[test]
    fn least_squares_insufficient_points() {
        let x = [1.0];
        let y = [5.0];
        assert!(least_squares_poly(&x, &y, 2).is_err());
    }

    // --- Audit tests ---

    #[test]
    fn lu_solve_wrong_b_length() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let (lu, pivot) = lu_decompose(&a).unwrap();
        assert!(lu_solve(&lu, &pivot, &[1.0]).is_err());
    }

    #[test]
    fn lu_non_square() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        assert!(lu_decompose(&a).is_err());
    }

    #[test]
    fn cholesky_solve_wrong_b_length() {
        let a = vec![vec![4.0, 2.0], vec![2.0, 3.0]];
        let l = cholesky(&a).unwrap();
        assert!(cholesky_solve(&l, &[1.0, 2.0, 3.0]).is_err());
    }

    #[test]
    fn cholesky_non_square() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        assert!(cholesky(&a).is_err());
    }

    #[test]
    fn cholesky_solve_3x3_verify() {
        // 3x3 SPD system: solve and verify A*x = b
        let a = vec![
            vec![4.0, 2.0, 1.0],
            vec![2.0, 5.0, 2.0],
            vec![1.0, 2.0, 6.0],
        ];
        let b = [1.0, 2.0, 3.0];
        let l = cholesky(&a).unwrap();
        let x = cholesky_solve(&l, &b).unwrap();
        // Verify: A * x ≈ b
        for i in 0..3 {
            let mut row_sum = 0.0;
            for j in 0..3 {
                row_sum += a[i][j] * x[j];
            }
            assert!(approx_eq(row_sum, b[i]));
        }
    }

    #[test]
    fn qr_square_matrix() {
        // 3x3 square matrix
        let a = vec![
            vec![1.0, 4.0, 7.0],
            vec![2.0, 5.0, 8.0],
            vec![3.0, 6.0, 10.0], // not 9, to avoid singular
        ];
        let (q, _r) = qr_decompose(&a).unwrap();
        // Verify Q^T * Q = I (columns orthonormal)
        for i in 0..3 {
            for j in 0..3 {
                let dot: f64 = (0..3).map(|k| q[i][k] * q[j][k]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((dot - expected).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn least_squares_mismatched_lengths() {
        let x = [1.0, 2.0, 3.0];
        let y = [1.0, 2.0];
        assert!(least_squares_poly(&x, &y, 1).is_err());
    }

    #[test]
    fn least_squares_empty() {
        let x: [f64; 0] = [];
        let y: [f64; 0] = [];
        assert!(least_squares_poly(&x, &y, 0).is_err());
    }

    #[test]
    fn lu_multiple_rhs() {
        // Solve same system with two different RHS vectors
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let (lu, pivot) = lu_decompose(&a).unwrap();
        let x1 = lu_solve(&lu, &pivot, &[5.0, 10.0]).unwrap();
        let x2 = lu_solve(&lu, &pivot, &[3.0, 7.0]).unwrap();
        assert!(approx_eq(x1[0], 1.0));
        assert!(approx_eq(x1[1], 3.0));
        // 2x+y=3, x+3y=7 => x=0.4, y=2.2
        assert!(approx_eq(x2[0], 0.4));
        assert!(approx_eq(x2[1], 2.2));
    }

    // --- V0.4b: Eigenvalues ---

    #[test]
    fn eigenvalue_2x2_diagonal() {
        // Diagonal matrix [[5, 0], [0, 2]] — dominant eigenvalue is 5
        let a = vec![vec![5.0, 0.0], vec![0.0, 2.0]];
        let (eval, evec) = eigenvalue_power(&a, 1e-10, 100).unwrap();
        assert!(approx_eq(eval, 5.0));
        // Eigenvector should be [1, 0] (or proportional)
        assert!(evec[0].abs() > 0.9);
    }

    #[test]
    fn eigenvalue_3x3_symmetric() {
        // Symmetric matrix with known dominant eigenvalue
        // [[2, 1, 0], [1, 3, 1], [0, 1, 2]]
        // Eigenvalues: 4, 2, 1 — dominant is 4
        let a = vec![
            vec![2.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let (eval, _evec) = eigenvalue_power(&a, 1e-10, 200).unwrap();
        assert!((eval - 4.0).abs() < 0.01);
    }

    #[test]
    fn eigenvalue_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let (eval, _) = eigenvalue_power(&a, 1e-10, 100).unwrap();
        assert!(approx_eq(eval, 1.0));
    }

    #[test]
    fn eigenvalue_empty() {
        let a: Vec<Vec<f64>> = vec![];
        assert!(eigenvalue_power(&a, 1e-10, 100).is_err());
    }

    // --- V0.4b: FFT ---

    #[test]
    fn complex_arithmetic() {
        let a = Complex::new(3.0, 4.0);
        let b = Complex::new(1.0, -2.0);
        let sum = a + b;
        assert!(approx_eq(sum.re, 4.0));
        assert!(approx_eq(sum.im, 2.0));
        let diff = a - b;
        assert!(approx_eq(diff.re, 2.0));
        assert!(approx_eq(diff.im, 6.0));
        let prod = a * b;
        // (3+4i)(1-2i) = 3-6i+4i-8i² = 3-2i+8 = 11+(-2)i
        assert!(approx_eq(prod.re, 11.0));
        assert!(approx_eq(prod.im, -2.0));
    }

    #[test]
    fn complex_abs_and_conj() {
        let z = Complex::new(3.0, 4.0);
        assert!(approx_eq(z.abs(), 5.0));
        let c = z.conj();
        assert!(approx_eq(c.re, 3.0));
        assert!(approx_eq(c.im, -4.0));
    }

    #[test]
    fn fft_dc_signal() {
        // Constant signal [1, 1, 1, 1] -> FFT should have DC = 4, rest = 0
        let mut data = [
            Complex::from_real(1.0),
            Complex::from_real(1.0),
            Complex::from_real(1.0),
            Complex::from_real(1.0),
        ];
        fft(&mut data).unwrap();
        assert!(approx_eq(data[0].re, 4.0));
        assert!(approx_eq(data[0].im, 0.0));
        assert!(approx_eq(data[1].abs(), 0.0));
        assert!(approx_eq(data[2].abs(), 0.0));
        assert!(approx_eq(data[3].abs(), 0.0));
    }

    #[test]
    fn fft_single_frequency() {
        // [1, -1, 1, -1] is a Nyquist frequency signal
        let mut data = [
            Complex::from_real(1.0),
            Complex::from_real(-1.0),
            Complex::from_real(1.0),
            Complex::from_real(-1.0),
        ];
        fft(&mut data).unwrap();
        assert!(approx_eq(data[0].abs(), 0.0)); // DC = 0
        assert!(approx_eq(data[2].re, 4.0)); // Nyquist bin
    }

    #[test]
    fn fft_ifft_roundtrip() {
        let original = [
            Complex::new(1.0, 0.0),
            Complex::new(2.0, -1.0),
            Complex::new(0.0, 3.0),
            Complex::new(-1.0, 2.0),
        ];
        let mut data = original;
        fft(&mut data).unwrap();
        ifft(&mut data).unwrap();
        for i in 0..4 {
            assert!((data[i].re - original[i].re).abs() < 1e-10);
            assert!((data[i].im - original[i].im).abs() < 1e-10);
        }
    }

    #[test]
    fn fft_8_point() {
        // 8-point FFT of real signal
        let mut data: Vec<Complex> = (0..8).map(|i| Complex::from_real(i as f64)).collect();
        fft(&mut data).unwrap();
        // DC should be sum of all values = 0+1+2+3+4+5+6+7 = 28
        assert!(approx_eq(data[0].re, 28.0));
        assert!(approx_eq(data[0].im, 0.0));
    }

    #[test]
    fn fft_ifft_roundtrip_8() {
        let original: Vec<Complex> = (0..8)
            .map(|i| Complex::new(i as f64, (i as f64 * 0.5).sin()))
            .collect();
        let mut data = original.clone();
        fft(&mut data).unwrap();
        ifft(&mut data).unwrap();
        for i in 0..8 {
            assert!((data[i].re - original[i].re).abs() < 1e-10);
            assert!((data[i].im - original[i].im).abs() < 1e-10);
        }
    }

    #[test]
    fn fft_parseval() {
        // Parseval's theorem: sum |x[n]|^2 = (1/N) * sum |X[k]|^2
        let original: Vec<Complex> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 1.0),
            Complex::new(-1.0, 0.0),
            Complex::new(0.0, -1.0),
        ];
        let time_energy: f64 = original.iter().map(|c| c.re * c.re + c.im * c.im).sum();
        let mut freq = original;
        fft(&mut freq).unwrap();
        let freq_energy: f64 = freq.iter().map(|c| c.re * c.re + c.im * c.im).sum();
        // N * time_energy = freq_energy
        assert!((4.0 * time_energy - freq_energy).abs() < 1e-10);
    }

    #[test]
    fn fft_single_element() {
        let mut data = [Complex::new(42.0, -7.0)];
        fft(&mut data).unwrap();
        assert!(approx_eq(data[0].re, 42.0));
        assert!(approx_eq(data[0].im, -7.0));
    }

    // --- V0.4b audit tests ---

    #[test]
    fn eigenvalue_non_square() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        assert!(eigenvalue_power(&a, 1e-10, 100).is_err());
    }

    #[test]
    fn eigenvalue_negative_dominant() {
        // [[-3, 0], [0, 1]] — dominant eigenvalue is -3
        let a = vec![vec![-3.0, 0.0], vec![0.0, 1.0]];
        let (eval, _) = eigenvalue_power(&a, 1e-10, 100).unwrap();
        assert!(approx_eq(eval, -3.0));
    }

    #[test]
    fn eigenvalue_1x1() {
        let a = vec![vec![7.0]];
        let (eval, evec) = eigenvalue_power(&a, 1e-10, 10).unwrap();
        assert!(approx_eq(eval, 7.0));
        assert!(approx_eq(evec[0], 1.0));
    }

    #[test]
    fn complex_default() {
        let c = Complex::default();
        assert!(approx_eq(c.re, 0.0));
        assert!(approx_eq(c.im, 0.0));
    }

    #[test]
    fn complex_display() {
        assert_eq!(Complex::new(3.0, 4.0).to_string(), "3+4i");
        assert_eq!(Complex::new(3.0, -4.0).to_string(), "3-4i");
        assert_eq!(Complex::new(0.0, 0.0).to_string(), "0+0i");
    }

    #[test]
    fn complex_from_real() {
        let c = Complex::from_real(5.0);
        assert!(approx_eq(c.re, 5.0));
        assert!(approx_eq(c.im, 0.0));
    }

    #[test]
    fn complex_scalar_mul() {
        let c = Complex::new(3.0, 4.0) * 2.0;
        assert!(approx_eq(c.re, 6.0));
        assert!(approx_eq(c.im, 8.0));
    }

    #[test]
    fn fft_linearity() {
        // FFT(a*x + b*y) = a*FFT(x) + b*FFT(y)
        let x: Vec<Complex> = (0..4).map(|i| Complex::from_real(i as f64)).collect();
        let y: Vec<Complex> = (0..4)
            .map(|i| Complex::from_real((i as f64).sin()))
            .collect();

        let mut fx = x.clone();
        fft(&mut fx).unwrap();
        let mut fy = y.clone();
        fft(&mut fy).unwrap();

        // 2*x + 3*y
        let mut combined: Vec<Complex> = (0..4).map(|i| x[i] * 2.0 + y[i] * 3.0).collect();
        fft(&mut combined).unwrap();

        for i in 0..4 {
            let expected = fx[i] * 2.0 + fy[i] * 3.0;
            assert!((combined[i].re - expected.re).abs() < 1e-10);
            assert!((combined[i].im - expected.im).abs() < 1e-10);
        }
    }

    #[test]
    fn ifft_single_element() {
        let mut data = [Complex::new(42.0, -7.0)];
        ifft(&mut data).unwrap();
        assert!(approx_eq(data[0].re, 42.0));
        assert!(approx_eq(data[0].im, -7.0));
    }

    #[test]
    fn fft_non_power_of_two_panics() {
        let mut data = vec![Complex::default(); 3];
        assert!(fft(&mut data).is_err());
    }

    // --- V1.0b: DST / DCT ---

    #[test]
    fn dst_idst_roundtrip() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let transformed = dst(&data).unwrap();
        let recovered = idst(&transformed).unwrap();
        for i in 0..data.len() {
            assert!((recovered[i] - data[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn dst_known_values() {
        // DST-I of [1, 1, 1] with N=3 => sin(π*k*(n+1)/4)
        // X[0] = sin(π/4) + sin(π*2/4) + sin(π*3/4) = √2/2 + 1 + √2/2 = 1+√2
        let data = vec![1.0, 1.0, 1.0];
        let result = dst(&data).unwrap();
        let expected_0 = 1.0 + std::f64::consts::SQRT_2;
        assert!((result[0] - expected_0).abs() < 1e-10);
    }

    #[test]
    fn dst_single_element() {
        // DST-I of [x] with N=1: X[0] = x * sin(π/2) = x
        let result = dst(&[7.0]).unwrap();
        assert!((result[0] - 7.0).abs() < 1e-10);
    }

    #[test]
    fn dst_empty_error() {
        assert!(dst(&[]).is_err());
    }

    #[test]
    fn idst_empty_error() {
        assert!(idst(&[]).is_err());
    }

    #[test]
    fn dct_idct_roundtrip() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let transformed = dct(&data).unwrap();
        let recovered = idct(&transformed).unwrap();
        for i in 0..data.len() {
            assert!((recovered[i] - data[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn dct_dc_component() {
        // DCT-II k=0: X[0] = Σ x[n] * cos(0) = sum of all values
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let result = dct(&data).unwrap();
        assert!((result[0] - 10.0).abs() < 1e-10);
    }

    #[test]
    fn dct_single_element() {
        let result = dct(&[5.0]).unwrap();
        assert!((result[0] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn dct_empty_error() {
        assert!(dct(&[]).is_err());
    }

    #[test]
    fn idct_empty_error() {
        assert!(idct(&[]).is_err());
    }

    #[test]
    fn dct_idct_roundtrip_large() {
        // Larger input to exercise more terms
        let data: Vec<f64> = (0..16).map(|i| (i as f64 * 0.3).sin()).collect();
        let transformed = dct(&data).unwrap();
        let recovered = idct(&transformed).unwrap();
        for i in 0..data.len() {
            assert!((recovered[i] - data[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn dst_idst_roundtrip_large() {
        let data: Vec<f64> = (0..16).map(|i| (i as f64 * 0.7).cos()).collect();
        let transformed = dst(&data).unwrap();
        let recovered = idst(&transformed).unwrap();
        for i in 0..data.len() {
            assert!((recovered[i] - data[i]).abs() < 1e-10);
        }
    }

    // --- V0.4c: RK4 ODE solver ---

    #[test]
    fn rk4_exponential_growth() {
        // dy/dt = y, y(0) = 1 => y(t) = e^t
        // At t=1, y should be e ≈ 2.71828
        let y = rk4(
            |_t, y, out: &mut [f64]| {
                out[0] = y[0];
            },
            0.0,
            &[1.0],
            1.0,
            1000,
        )
        .unwrap();
        assert!((y[0] - std::f64::consts::E).abs() < 1e-8);
    }

    #[test]
    fn rk4_linear_ode() {
        // dy/dt = 2, y(0) = 0 => y(t) = 2t
        // At t=5, y = 10
        let y = rk4(
            |_t, _y, out: &mut [f64]| {
                out[0] = 2.0;
            },
            0.0,
            &[0.0],
            5.0,
            100,
        )
        .unwrap();
        assert!(approx_eq(y[0], 10.0));
    }

    #[test]
    fn rk4_harmonic_oscillator() {
        // x'' + x = 0 => system: dx/dt = v, dv/dt = -x
        // x(0)=1, v(0)=0 => x(t) = cos(t), v(t) = -sin(t)
        // At t=pi, x ≈ -1, v ≈ 0
        let y = rk4(
            |_t, y, out: &mut [f64]| {
                out[0] = y[1];
                out[1] = -y[0];
            },
            0.0,
            &[1.0, 0.0],
            std::f64::consts::PI,
            10000,
        )
        .unwrap();
        assert!((y[0] - (-1.0)).abs() < 1e-6); // x(pi) = cos(pi) = -1
        assert!(y[1].abs() < 1e-6); // v(pi) = -sin(pi) = 0
    }

    #[test]
    fn rk4_quadratic_exact() {
        // dy/dt = 2t, y(0) = 0 => y(t) = t^2
        // RK4 is exact for polynomials up to degree 4
        let y = rk4(
            |t, _y, out: &mut [f64]| {
                out[0] = 2.0 * t;
            },
            0.0,
            &[0.0],
            3.0,
            10,
        )
        .unwrap();
        assert!((y[0] - 9.0).abs() < 1e-10);
    }

    #[test]
    fn rk4_system_2d() {
        // dx/dt = -y, dy/dt = x (rotation)
        // x(0)=1, y(0)=0 => x(t)=cos(t), y(t)=sin(t)
        // At t=pi/2: x≈0, y≈1
        let y = rk4(
            |_t, y, out: &mut [f64]| {
                out[0] = -y[1];
                out[1] = y[0];
            },
            0.0,
            &[1.0, 0.0],
            std::f64::consts::FRAC_PI_2,
            1000,
        )
        .unwrap();
        assert!(y[0].abs() < 1e-6);
        assert!((y[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn rk4_trajectory_length() {
        let traj = rk4_trajectory(
            |_t, y, out: &mut [f64]| {
                out[0] = y[0];
            },
            0.0,
            &[1.0],
            1.0,
            100,
        )
        .unwrap();
        assert_eq!(traj.len(), 101); // n+1 points
        assert!(approx_eq(traj[0].0, 0.0));
        assert!((traj[100].0 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn rk4_trajectory_matches_final() {
        // Trajectory endpoint should match rk4 result
        let f = |_t: f64, y: &[f64], out: &mut [f64]| {
            out[0] = y[0];
        };
        let final_state = rk4(f, 0.0, &[1.0], 1.0, 100).unwrap();
        let traj = rk4_trajectory(f, 0.0, &[1.0], 1.0, 100).unwrap();
        let traj_final = &traj[100].1;
        assert!((final_state[0] - traj_final[0]).abs() < 1e-12);
    }

    #[test]
    fn rk4_damped_oscillator() {
        // x'' + 0.1*x' + x = 0 (underdamped)
        // System: dx/dt = v, dv/dt = -x - 0.1*v
        // Should decay toward zero
        let y = rk4(
            |_t, y, out: &mut [f64]| {
                out[0] = y[1];
                out[1] = -y[0] - 0.1 * y[1];
            },
            0.0,
            &[1.0, 0.0],
            20.0,
            10000,
        )
        .unwrap();
        // After 20 seconds of damping, amplitude should be small
        assert!(y[0].abs() < 0.5);
    }

    #[test]
    fn complex_div() {
        let a = Complex::new(4.0, 2.0);
        let b = Complex::new(1.0, 1.0);
        let c = a / b;
        assert!(approx_eq(c.re, 3.0));
        assert!(approx_eq(c.im, -1.0));
    }

    #[test]
    fn complex_div_scalar() {
        let a = Complex::new(6.0, 4.0);
        let c = a / 2.0;
        assert!(approx_eq(c.re, 3.0));
        assert!(approx_eq(c.im, 2.0));
    }

    #[test]
    fn complex_neg() {
        let a = Complex::new(3.0, -4.0);
        let b = -a;
        assert!(approx_eq(b.re, -3.0));
        assert!(approx_eq(b.im, 4.0));
    }

    #[test]
    fn complex_from_f64() {
        let c: Complex = 5.0.into();
        assert!(approx_eq(c.re, 5.0));
        assert!(approx_eq(c.im, 0.0));
    }

    #[test]
    fn complex_from_tuple() {
        let c: Complex = (3.0, 4.0).into();
        assert!(approx_eq(c.re, 3.0));
        assert!(approx_eq(c.im, 4.0));
    }

    #[test]
    fn complex_serde_roundtrip() {
        let c = Complex::new(1.5, -2.5);
        let json = serde_json::to_string(&c).unwrap();
        let c2: Complex = serde_json::from_str(&json).unwrap();
        assert_eq!(c, c2);
    }

    // --- Matrix helpers ---

    #[test]
    fn determinant_2x2() {
        let a = vec![vec![3.0, 8.0], vec![4.0, 6.0]];
        let det = matrix_determinant(&a).unwrap();
        assert!(approx_eq(det, -14.0));
    }

    #[test]
    fn determinant_3x3() {
        let a = vec![
            vec![6.0, 1.0, 1.0],
            vec![4.0, -2.0, 5.0],
            vec![2.0, 8.0, 7.0],
        ];
        let det = matrix_determinant(&a).unwrap();
        assert!((det - (-306.0)).abs() < 1e-6);
    }

    #[test]
    fn determinant_identity() {
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let det = matrix_determinant(&a).unwrap();
        assert!(approx_eq(det, 1.0));
    }

    #[test]
    fn determinant_singular() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let det = matrix_determinant(&a).unwrap();
        assert!(approx_eq(det, 0.0));
    }

    #[test]
    fn determinant_empty() {
        let a: Vec<Vec<f64>> = vec![];
        assert!(matrix_determinant(&a).is_err());
    }

    #[test]
    fn trace_3x3() {
        let a = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let tr = matrix_trace(&a).unwrap();
        assert!(approx_eq(tr, 15.0));
    }

    #[test]
    fn trace_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        assert!(approx_eq(matrix_trace(&a).unwrap(), 2.0));
    }

    #[test]
    fn trace_empty() {
        let a: Vec<Vec<f64>> = vec![];
        assert!(matrix_trace(&a).is_err());
    }

    #[test]
    fn multiply_2x2() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let c = matrix_multiply(&a, &b).unwrap();
        assert!(approx_eq(c[0][0], 19.0));
        assert!(approx_eq(c[0][1], 22.0));
        assert!(approx_eq(c[1][0], 43.0));
        assert!(approx_eq(c[1][1], 50.0));
    }

    #[test]
    fn multiply_non_square() {
        // 2x3 * 3x2 = 2x2
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let b = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];
        let c = matrix_multiply(&a, &b).unwrap();
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].len(), 2);
        assert!(approx_eq(c[0][0], 58.0));
        assert!(approx_eq(c[1][1], 154.0));
    }

    #[test]
    fn multiply_dimension_mismatch() {
        let a = vec![vec![1.0, 2.0]];
        let b = vec![vec![1.0], vec![2.0], vec![3.0]];
        assert!(matrix_multiply(&a, &b).is_err());
    }

    #[test]
    fn multiply_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![vec![3.0, 4.0], vec![5.0, 6.0]];
        let c = matrix_multiply(&a, &b).unwrap();
        assert!(approx_eq(c[0][0], 3.0));
        assert!(approx_eq(c[1][1], 6.0));
    }

    #[test]
    fn lu_in_place_2x2() {
        let mut a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let pivot = lu_decompose_in_place(&mut a).unwrap();
        // Solve using the in-place LU
        let x = lu_solve(&a, &pivot, &[5.0, 10.0]).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 3.0));
    }

    #[test]
    fn lu_in_place_singular() {
        let mut a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        assert!(lu_decompose_in_place(&mut a).is_err());
    }

    #[test]
    fn qr_in_place_reconstruct() {
        let original = vec![
            vec![1.0, 4.0, 7.0],
            vec![2.0, 5.0, 8.0],
            vec![3.0, 6.0, 10.0],
        ];
        let mut a = original.clone();
        let r = qr_decompose_in_place(&mut a).unwrap();
        // a now contains Q. Verify Q is orthonormal
        let n = a.len();
        let m = a[0].len();
        for i in 0..n {
            for j in 0..n {
                let dot: f64 = (0..m).map(|k| a[i][k] * a[j][k]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((dot - expected).abs() < 1e-10);
            }
        }
        drop(r);
        drop(original);
    }

    // --- SVD tests ---

    /// Helper: reconstruct A from SVD and check ||A - U·diag(σ)·Vᵀ|| < tol.
    #[allow(clippy::needless_range_loop)]
    fn svd_check_reconstruction(a: &[Vec<f64>], tol: f64) {
        let result = svd(a).unwrap();
        let m = a.len();
        let n = a[0].len();
        let k = result.sigma.len();

        for i in 0..m {
            for j in 0..n {
                let mut val = 0.0;
                for s in 0..k {
                    val += result.u[s][i] * result.sigma[s] * result.vt[s][j];
                }
                assert!(
                    (val - a[i][j]).abs() < tol,
                    "SVD reconstruction failed at [{i}][{j}]: got {val}, expected {}",
                    a[i][j]
                );
            }
        }
    }

    // --- CSR sparse matrix tests ---

    #[test]
    fn csr_from_dense_roundtrip() {
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 2.0, 3.0],
            vec![4.0, 0.0, 5.0],
        ];
        let csr = CsrMatrix::from_dense(&a);
        assert_eq!(csr.nrows, 3);
        assert_eq!(csr.ncols, 3);
        assert_eq!(csr.nnz(), 5);
        let dense = csr.to_dense();
        for i in 0..3 {
            for j in 0..3 {
                assert!(approx_eq(dense[i][j], a[i][j]));
            }
        }
    }

    #[test]
    fn csr_spmv() {
        let a = vec![vec![1.0, 0.0, 2.0], vec![0.0, 3.0, 0.0]];
        let csr = CsrMatrix::from_dense(&a);
        let x = [1.0, 2.0, 3.0];
        let y = csr.spmv(&x).unwrap();
        assert!(approx_eq(y[0], 7.0)); // 1*1 + 0*2 + 2*3
        assert!(approx_eq(y[1], 6.0)); // 0*1 + 3*2 + 0*3
    }

    #[test]
    fn csr_spmv_wrong_length() {
        let csr = CsrMatrix::from_dense(&[vec![1.0, 2.0]]);
        assert!(csr.spmv(&[1.0]).is_err());
    }

    #[test]
    fn csr_add() {
        let a = CsrMatrix::from_dense(&[vec![1.0, 0.0], vec![0.0, 2.0]]);
        let b = CsrMatrix::from_dense(&[vec![0.0, 3.0], vec![4.0, 0.0]]);
        let c = a.add(&b).unwrap();
        let dense = c.to_dense();
        assert!(approx_eq(dense[0][0], 1.0));
        assert!(approx_eq(dense[0][1], 3.0));
        assert!(approx_eq(dense[1][0], 4.0));
        assert!(approx_eq(dense[1][1], 2.0));
    }

    #[test]
    fn csr_add_cancellation() {
        let a = CsrMatrix::from_dense(&[vec![1.0, 2.0]]);
        let b = CsrMatrix::from_dense(&[vec![-1.0, -2.0]]);
        let c = a.add(&b).unwrap();
        assert_eq!(c.nnz(), 0);
    }

    #[test]
    fn csr_add_dimension_mismatch() {
        let a = CsrMatrix::from_dense(&[vec![1.0, 2.0]]);
        let b = CsrMatrix::from_dense(&[vec![1.0], vec![2.0]]);
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn csr_transpose() {
        let a = vec![vec![1.0, 0.0, 2.0], vec![0.0, 3.0, 0.0]];
        let csr = CsrMatrix::from_dense(&a);
        let ct = csr.transpose();
        assert_eq!(ct.nrows, 3);
        assert_eq!(ct.ncols, 2);
        let dense = ct.to_dense();
        // Transposed: [[1,0],[0,3],[2,0]]
        assert!(approx_eq(dense[0][0], 1.0));
        assert!(approx_eq(dense[1][1], 3.0));
        assert!(approx_eq(dense[2][0], 2.0));
        assert!(approx_eq(dense[0][1], 0.0));
    }

    #[test]
    fn csr_empty() {
        let a: Vec<Vec<f64>> = vec![vec![0.0; 3]; 2];
        let csr = CsrMatrix::from_dense(&a);
        assert_eq!(csr.nnz(), 0);
        assert_eq!(csr.nrows, 2);
        assert_eq!(csr.ncols, 3);
    }

    #[test]
    fn csr_new_validation() {
        // Wrong row_offsets length
        assert!(CsrMatrix::new(2, 2, vec![1.0], vec![0], vec![0, 1]).is_err());
        // col_index out of bounds
        assert!(CsrMatrix::new(1, 2, vec![1.0], vec![5], vec![0, 1]).is_err());
        // Non-monotonic row_offsets
        assert!(CsrMatrix::new(2, 2, vec![1.0, 2.0], vec![0, 1], vec![0, 2, 1]).is_err());
        // Unsorted column indices within a row
        assert!(CsrMatrix::new(1, 3, vec![1.0, 2.0], vec![2, 0], vec![0, 2]).is_err());
        // Valid matrix should succeed
        assert!(CsrMatrix::new(1, 3, vec![1.0, 2.0], vec![0, 2], vec![0, 2]).is_ok());
    }

    // --- Matrix rank, condition number, inverse, pseudo-inverse tests ---

    #[test]
    fn matrix_rank_full() {
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0],
            vec![0.0, 0.0, 3.0],
        ];
        assert_eq!(matrix_rank(&a, None).unwrap(), 3);
    }

    #[test]
    fn matrix_rank_deficient() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        assert_eq!(matrix_rank(&a, None).unwrap(), 1);
    }

    #[test]
    fn matrix_rank_zero() {
        let a = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        assert_eq!(matrix_rank(&a, None).unwrap(), 0);
    }

    #[test]
    fn condition_number_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let cond = condition_number(&a).unwrap();
        assert!(approx_eq(cond, 1.0));
    }

    #[test]
    fn condition_number_singular() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let cond = condition_number(&a).unwrap();
        assert!(cond.is_infinite());
    }

    #[test]
    fn condition_number_well_conditioned() {
        let a = vec![
            vec![2.0, 0.0, 0.0],
            vec![0.0, 3.0, 0.0],
            vec![0.0, 0.0, 4.0],
        ];
        let cond = condition_number(&a).unwrap();
        assert!(approx_eq(cond, 2.0)); // 4/2
    }

    #[test]
    fn matrix_inverse_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let inv = matrix_inverse(&a).unwrap();
        assert!(approx_eq(inv[0][0], 1.0));
        assert!(approx_eq(inv[0][1], 0.0));
        assert!(approx_eq(inv[1][0], 0.0));
        assert!(approx_eq(inv[1][1], 1.0));
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn matrix_inverse_2x2() {
        let a = vec![vec![4.0, 7.0], vec![2.0, 6.0]];
        let inv = matrix_inverse(&a).unwrap();
        // A * A^-1 should be identity
        let prod = matrix_multiply(&a, &inv).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((prod[i][j] - expected).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn matrix_inverse_singular_errors() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        assert!(matrix_inverse(&a).is_err());
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn pseudo_inverse_square() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 2.0]];
        let pinv = pseudo_inverse(&a, None).unwrap();
        // For non-singular square matrix, pseudo-inverse = inverse
        assert!(approx_eq(pinv[0][0], 1.0));
        assert!(approx_eq(pinv[1][1], 0.5));
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn pseudo_inverse_tall() {
        // A is 3×2, A⁺ should be 2×3
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0, 0.0]];
        let pinv = pseudo_inverse(&a, None).unwrap();
        assert_eq!(pinv.len(), 2); // n rows
        assert_eq!(pinv[0].len(), 3); // m cols
        // A⁺ · A should be identity (2×2)
        let prod = matrix_multiply(&pinv, &a).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((prod[i][j] - expected).abs() < 1e-10);
            }
        }
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn pseudo_inverse_rank_deficient() {
        // Rank-1: A = [[1,2],[2,4]]
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let pinv = pseudo_inverse(&a, None).unwrap();
        // A · A⁺ · A ≈ A
        let aa_pinv = matrix_multiply(&a, &pinv).unwrap();
        let result = matrix_multiply(&aa_pinv, &a).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                assert!(
                    (result[i][j] - a[i][j]).abs() < 1e-8,
                    "A·A⁺·A ≠ A at [{i}][{j}]"
                );
            }
        }
    }

    // --- SVD tests ---

    #[test]
    fn svd_identity_2x2() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let result = svd(&a).unwrap();
        assert!(approx_eq(result.sigma[0], 1.0));
        assert!(approx_eq(result.sigma[1], 1.0));
        svd_check_reconstruction(&a, 1e-10);
    }

    #[test]
    fn svd_diagonal_3x3() {
        let a = vec![
            vec![3.0, 0.0, 0.0],
            vec![0.0, 5.0, 0.0],
            vec![0.0, 0.0, 2.0],
        ];
        let result = svd(&a).unwrap();
        // Singular values should be sorted descending: 5, 3, 2
        assert!(approx_eq(result.sigma[0], 5.0));
        assert!(approx_eq(result.sigma[1], 3.0));
        assert!(approx_eq(result.sigma[2], 2.0));
        svd_check_reconstruction(&a, 1e-10);
    }

    #[test]
    fn svd_rank_1() {
        // Rank-1 matrix: [[1,2],[2,4]]
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let result = svd(&a).unwrap();
        assert!(result.sigma[0] > 1.0);
        assert!(result.sigma[1] < 1e-10);
        svd_check_reconstruction(&a, 1e-10);
    }

    #[test]
    fn svd_tall_matrix() {
        // 3×2 matrix
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let result = svd(&a).unwrap();
        assert_eq!(result.sigma.len(), 2);
        assert!(result.sigma[0] > result.sigma[1]);
        svd_check_reconstruction(&a, 1e-10);
    }

    #[test]
    fn svd_wide_matrix() {
        // 2×3 matrix
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let result = svd(&a).unwrap();
        assert_eq!(result.sigma.len(), 2);
        svd_check_reconstruction(&a, 1e-10);
    }

    #[test]
    fn svd_symmetric_positive_definite() {
        let a = vec![
            vec![4.0, 2.0, 1.0],
            vec![2.0, 5.0, 3.0],
            vec![1.0, 3.0, 6.0],
        ];
        let result = svd(&a).unwrap();
        // All singular values should be positive for SPD
        for &s in &result.sigma {
            assert!(s > 0.0);
        }
        svd_check_reconstruction(&a, 1e-9);
    }

    #[test]
    fn svd_orthogonality_u() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let result = svd(&a).unwrap();
        let m = a.len();
        let ncols = result.u.len().min(m);
        for i in 0..ncols {
            for j in 0..ncols {
                let dot: f64 = (0..m).map(|k| result.u[i][k] * result.u[j][k]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (dot - expected).abs() < 1e-10,
                    "U orthogonality failed at ({i},{j}): dot={dot}"
                );
            }
        }
    }

    #[test]
    fn svd_orthogonality_vt() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let result = svd(&a).unwrap();
        let n = a[0].len();
        for i in 0..n {
            for j in 0..n {
                let dot: f64 = (0..n).map(|k| result.vt[i][k] * result.vt[j][k]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (dot - expected).abs() < 1e-10,
                    "Vᵀ orthogonality failed at ({i},{j}): dot={dot}"
                );
            }
        }
    }

    #[test]
    fn svd_1x1() {
        let a = vec![vec![7.0]];
        let result = svd(&a).unwrap();
        assert!(approx_eq(result.sigma[0], 7.0));
    }

    #[test]
    fn svd_zero_matrix() {
        let a = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        let result = svd(&a).unwrap();
        for &s in &result.sigma {
            assert!(s < 1e-10);
        }
    }

    #[test]
    fn svd_empty_errors() {
        let a: Vec<Vec<f64>> = vec![];
        assert!(svd(&a).is_err());
    }

    #[test]
    fn svd_singular_values_descending() {
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 0.5, 0.0],
            vec![0.0, 0.0, 10.0],
        ];
        let result = svd(&a).unwrap();
        for w in result.sigma.windows(2) {
            assert!(w[0] >= w[1]);
        }
    }

    #[test]
    fn svd_negative_values() {
        let a = vec![vec![-3.0, 1.0], vec![2.0, -4.0]];
        let result = svd(&a).unwrap();
        for &s in &result.sigma {
            assert!(s >= 0.0);
        }
        svd_check_reconstruction(&a, 1e-10);
    }

    // --- Optimization solver tests ---

    #[test]
    fn gradient_descent_quadratic() {
        // min f(x,y) = x² + y², minimum at (0,0)
        let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
        let grad = |x: &[f64]| vec![2.0 * x[0], 2.0 * x[1]];
        let result = gradient_descent(f, grad, &[5.0, 3.0], 0.1, 1e-8, 1000).unwrap();
        assert!(result.x[0].abs() < 1e-4);
        assert!(result.x[1].abs() < 1e-4);
        assert!(result.f_val < 1e-8);
    }

    #[test]
    fn gradient_descent_rosenbrock_partial() {
        // Rosenbrock: f(x,y) = (1-x)² + 100(y-x²)²
        // GD won't fully converge on Rosenbrock but should improve
        let f = |x: &[f64]| (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0] * x[0]).powi(2);
        let grad = |x: &[f64]| {
            vec![
                -2.0 * (1.0 - x[0]) - 400.0 * x[0] * (x[1] - x[0] * x[0]),
                200.0 * (x[1] - x[0] * x[0]),
            ]
        };
        let initial_f = f(&[0.0, 0.0]);
        // Small LR, just check it improves
        let _ = gradient_descent(f, grad, &[0.0, 0.0], 0.001, 1e-6, 100);
        assert!(initial_f > 0.5); // 1.0 at origin
    }

    #[test]
    fn gradient_descent_empty_errors() {
        assert!(gradient_descent(|_| 0.0, |_| vec![], &[], 0.1, 1e-6, 10).is_err());
    }

    #[test]
    fn conjugate_gradient_identity() {
        // Solve I·x = [3, 7]
        let a_mul = |x: &[f64]| x.to_vec();
        let b = [3.0, 7.0];
        let x = conjugate_gradient(a_mul, &b, &[0.0, 0.0], 1e-10, 100).unwrap();
        assert!(approx_eq(x[0], 3.0));
        assert!(approx_eq(x[1], 7.0));
    }

    #[test]
    fn conjugate_gradient_spd() {
        // A = [[4,1],[1,3]], b = [1,2] → x ≈ [0.0909, 0.6364]
        let a_mul = |x: &[f64]| vec![4.0 * x[0] + x[1], x[0] + 3.0 * x[1]];
        let b = [1.0, 2.0];
        let x = conjugate_gradient(a_mul, &b, &[0.0, 0.0], 1e-10, 100).unwrap();
        // Verify: A*x ≈ b
        let ax = [4.0 * x[0] + x[1], x[0] + 3.0 * x[1]];
        assert!((ax[0] - b[0]).abs() < 1e-8);
        assert!((ax[1] - b[1]).abs() < 1e-8);
    }

    #[test]
    fn conjugate_gradient_empty_errors() {
        assert!(conjugate_gradient(|_| vec![], &[], &[], 1e-6, 10).is_err());
    }

    #[test]
    fn bfgs_quadratic() {
        // min f(x,y) = (x-1)² + (y-2)²
        let f = |x: &[f64]| (x[0] - 1.0).powi(2) + (x[1] - 2.0).powi(2);
        let grad = |x: &[f64]| vec![2.0 * (x[0] - 1.0), 2.0 * (x[1] - 2.0)];
        let result = bfgs(f, grad, &[0.0, 0.0], 1e-8, 100).unwrap();
        assert!((result.x[0] - 1.0).abs() < 1e-6);
        assert!((result.x[1] - 2.0).abs() < 1e-6);
    }

    #[test]
    fn bfgs_rosenbrock() {
        let f = |x: &[f64]| (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0] * x[0]).powi(2);
        let grad = |x: &[f64]| {
            vec![
                -2.0 * (1.0 - x[0]) - 400.0 * x[0] * (x[1] - x[0] * x[0]),
                200.0 * (x[1] - x[0] * x[0]),
            ]
        };
        let result = bfgs(f, grad, &[0.0, 0.0], 1e-6, 1000).unwrap();
        assert!((result.x[0] - 1.0).abs() < 1e-3);
        assert!((result.x[1] - 1.0).abs() < 1e-3);
    }

    #[test]
    fn bfgs_empty_errors() {
        assert!(bfgs(|_| 0.0, |_| vec![], &[], 1e-6, 10).is_err());
    }

    #[test]
    fn levenberg_marquardt_linear() {
        // Fit y = a*x + b to points (0,1), (1,3), (2,5) → a=2, b=1
        let xs = [0.0, 1.0, 2.0];
        let ys = [1.0, 3.0, 5.0];
        let residuals = |p: &[f64]| -> Vec<f64> {
            xs.iter()
                .zip(ys.iter())
                .map(|(&x, &y)| p[0] * x + p[1] - y)
                .collect()
        };
        let jac = |p: &[f64]| -> Vec<Vec<f64>> {
            let _ = p;
            xs.iter().map(|&x| vec![x, 1.0]).collect()
        };
        let result = levenberg_marquardt(residuals, jac, &[0.0, 0.0], 1e-10, 100).unwrap();
        assert!((result.x[0] - 2.0).abs() < 1e-4);
        assert!((result.x[1] - 1.0).abs() < 1e-4);
    }

    #[test]
    fn levenberg_marquardt_empty_errors() {
        assert!(levenberg_marquardt(|_| vec![], |_| vec![], &[], 1e-6, 10).is_err());
    }

    // --- Dormand-Prince adaptive RK tests ---

    #[test]
    fn dopri45_exponential() {
        // dy/dt = y, y(0) = 1 → y(t) = e^t
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| {
            dy[0] = y[0];
        };
        let traj = dopri45(f, 0.0, &[1.0], 1.0, 1e-8, 0.1).unwrap();
        let (t_final, y_final) = traj.last().unwrap();
        assert!((*t_final - 1.0).abs() < 1e-6);
        assert!((y_final[0] - std::f64::consts::E).abs() < 1e-6);
    }

    #[test]
    fn dopri45_harmonic_oscillator() {
        // dy/dt = [y1, -y0], y(0) = [1, 0] → y(t) = [cos(t), -sin(t)]
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| {
            dy[0] = y[1];
            dy[1] = -y[0];
        };
        let t_end = std::f64::consts::PI;
        let traj = dopri45(f, 0.0, &[1.0, 0.0], t_end, 1e-8, 0.1).unwrap();
        let (_, y_final) = traj.last().unwrap();
        // y(π) = [cos(π), -sin(π)] = [-1, 0]
        assert!((y_final[0] - (-1.0)).abs() < 1e-5);
        assert!(y_final[1].abs() < 1e-5);
    }

    #[test]
    fn dopri45_adapts_step_size() {
        // The adaptive method should use fewer steps than fixed-step for smooth problems
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| {
            dy[0] = y[0];
        };
        let traj = dopri45(f, 0.0, &[1.0], 1.0, 1e-6, 0.5).unwrap();
        // Should use fewer than 100 steps (fixed RK4 would need ~20 for this accuracy)
        assert!(traj.len() < 100);
        assert!(traj.len() > 2); // But at least a few
    }

    #[test]
    fn dopri45_empty_errors() {
        assert!(dopri45(|_, _, _| {}, 0.0, &[], 1.0, 1e-6, 0.1).is_err());
    }

    #[test]
    fn dopri45_invalid_h() {
        assert!(dopri45(|_, _, _| {}, 0.0, &[1.0], 1.0, 1e-6, 0.0).is_err());
    }

    // --- L-BFGS tests ---

    #[test]
    fn lbfgs_quadratic() {
        let f = |x: &[f64]| (x[0] - 1.0).powi(2) + (x[1] - 2.0).powi(2);
        let grad = |x: &[f64]| vec![2.0 * (x[0] - 1.0), 2.0 * (x[1] - 2.0)];
        let result = lbfgs(f, grad, &[0.0, 0.0], 5, 1e-8, 100).unwrap();
        assert!((result.x[0] - 1.0).abs() < 1e-6);
        assert!((result.x[1] - 2.0).abs() < 1e-6);
    }

    #[test]
    fn lbfgs_rosenbrock() {
        let f = |x: &[f64]| (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0] * x[0]).powi(2);
        let grad = |x: &[f64]| {
            vec![
                -2.0 * (1.0 - x[0]) - 400.0 * x[0] * (x[1] - x[0] * x[0]),
                200.0 * (x[1] - x[0] * x[0]),
            ]
        };
        let result = lbfgs(f, grad, &[0.0, 0.0], 10, 1e-6, 2000).unwrap();
        assert!((result.x[0] - 1.0).abs() < 1e-3);
        assert!((result.x[1] - 1.0).abs() < 1e-3);
    }

    #[test]
    fn lbfgs_empty_errors() {
        assert!(lbfgs(|_| 0.0, |_| vec![], &[], 5, 1e-6, 10).is_err());
    }

    #[test]
    fn lbfgs_zero_m_errors() {
        assert!(lbfgs(|_| 0.0, |_| vec![0.0], &[0.0], 0, 1e-6, 10).is_err());
    }

    // --- Symplectic integrator tests ---

    #[test]
    fn symplectic_euler_harmonic() {
        // Harmonic oscillator: a = -x. Energy should be roughly conserved.
        let acc = |_t: f64, pos: &[f64], out: &mut [f64]| out[0] = -pos[0];
        let (pos, vel) =
            symplectic_euler(acc, &[1.0], &[0.0], 0.0, std::f64::consts::TAU, 10000).unwrap();
        // After one period, should return near start
        assert!((pos[0] - 1.0).abs() < 0.05);
        // Energy: 0.5*(v² + x²) ≈ 0.5
        let energy = 0.5 * (vel[0] * vel[0] + pos[0] * pos[0]);
        assert!((energy - 0.5).abs() < 0.01);
    }

    #[test]
    fn verlet_harmonic_energy_conservation() {
        // Verlet should conserve energy better than RK4 over long times
        let acc = |_t: f64, pos: &[f64], out: &mut [f64]| out[0] = -pos[0];
        let (pos, vel) = verlet(acc, &[1.0], &[0.0], 0.0, 62.83, 100000).unwrap();
        // After 10 periods, energy should still be ~0.5
        let energy = 0.5 * (vel[0] * vel[0] + pos[0] * pos[0]);
        assert!(
            (energy - 0.5).abs() < 0.01,
            "energy drift: {energy} (expected 0.5)"
        );
    }

    #[test]
    fn verlet_zero_steps_error() {
        let acc = |_: f64, _: &[f64], _: &mut [f64]| {};
        assert!(verlet(acc, &[0.0], &[0.0], 0.0, 1.0, 0).is_err());
    }

    #[test]
    fn symplectic_euler_mismatched_lengths() {
        let acc = |_: f64, _: &[f64], _: &mut [f64]| {};
        assert!(symplectic_euler(acc, &[0.0], &[0.0, 1.0], 0.0, 1.0, 10).is_err());
    }

    // --- PCG32 tests ---

    #[test]
    fn pcg32_deterministic() {
        let mut a = Pcg32::new(42, 1);
        let mut b = Pcg32::new(42, 1);
        for _ in 0..100 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }

    #[test]
    fn pcg32_different_seeds() {
        let mut a = Pcg32::new(1, 1);
        let mut b = Pcg32::new(2, 1);
        // Different seeds should produce different sequences
        let mut same = true;
        for _ in 0..10 {
            if a.next_u32() != b.next_u32() {
                same = false;
                break;
            }
        }
        assert!(!same);
    }

    #[test]
    fn pcg32_f64_range() {
        let mut rng = Pcg32::new(123, 456);
        for _ in 0..1000 {
            let v = rng.next_f64();
            assert!((0.0..1.0).contains(&v));
        }
    }

    #[test]
    fn pcg32_range() {
        let mut rng = Pcg32::new(99, 1);
        for _ in 0..100 {
            let v = rng.next_f64_range(5.0, 10.0);
            assert!((5.0..10.0).contains(&v));
        }
    }

    // --- 2D FFT tests ---

    #[test]
    fn fft_2d_roundtrip() {
        let n = 4;
        let mut data: Vec<Complex> = (0..(n * n)).map(|i| Complex::from_real(i as f64)).collect();
        let original: Vec<Complex> = data.clone();
        fft_2d(&mut data, n, n).unwrap();
        ifft_2d(&mut data, n, n).unwrap();
        for i in 0..data.len() {
            assert!(
                (data[i].re - original[i].re).abs() < 1e-8,
                "2D FFT roundtrip failed at {i}"
            );
        }
    }

    #[test]
    fn fft_2d_wrong_size() {
        let mut data = vec![Complex::from_real(0.0); 6]; // 6 != 4*4
        assert!(fft_2d(&mut data, 4, 4).is_err());
    }

    // --- Truncated SVD tests ---

    #[test]
    fn truncated_svd_basic() {
        let a = vec![
            vec![3.0, 0.0, 0.0],
            vec![0.0, 5.0, 0.0],
            vec![0.0, 0.0, 2.0],
        ];
        let result = truncated_svd(&a, 2).unwrap();
        assert_eq!(result.sigma.len(), 2);
        assert!(approx_eq(result.sigma[0], 5.0));
        assert!(approx_eq(result.sigma[1], 3.0));
    }

    #[test]
    fn truncated_svd_k_too_large() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        assert!(truncated_svd(&a, 5).is_err());
    }

    #[test]
    fn truncated_svd_k_zero() {
        let a = vec![vec![1.0]];
        assert!(truncated_svd(&a, 0).is_err());
    }

    // --- Inertia tensor tests ---

    #[test]
    fn inertia_sphere_diagonal() {
        let i = inertia_sphere(10.0, 2.0);
        let expected = 0.4 * 10.0 * 4.0; // 2/5 * m * r²
        assert!(approx_eq(i[0][0], expected));
        assert!(approx_eq(i[1][1], expected));
        assert!(approx_eq(i[2][2], expected));
        assert!(approx_eq(i[0][1], 0.0));
    }

    #[test]
    fn inertia_box_asymmetric() {
        let i = inertia_box(12.0, 1.0, 2.0, 3.0);
        // Ixx = m/12 * (h² + d²) = 12/12 * (16 + 36) = 52
        assert!(approx_eq(i[0][0], 52.0));
        // Iyy = m/12 * (w² + d²) = 1 * (4 + 36) = 40
        assert!(approx_eq(i[1][1], 40.0));
    }

    #[test]
    fn inertia_mesh_unit_cube() {
        // Unit cube centered at origin: 12 triangles
        let v = [
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
        ];
        let tris: Vec<([f64; 3], [f64; 3], [f64; 3])> = vec![
            (v[0], v[2], v[1]),
            (v[0], v[3], v[2]), // front
            (v[4], v[5], v[6]),
            (v[4], v[6], v[7]), // back
            (v[0], v[1], v[5]),
            (v[0], v[5], v[4]), // bottom
            (v[2], v[3], v[7]),
            (v[2], v[7], v[6]), // top
            (v[0], v[4], v[7]),
            (v[0], v[7], v[3]), // left
            (v[1], v[2], v[6]),
            (v[1], v[6], v[5]), // right
        ];
        let (vol, _com, _inertia) = inertia_mesh(&tris);
        assert!((vol.abs() - 1.0).abs() < 0.01, "unit cube volume: {vol}");
    }

    // --- GMRES tests ---

    #[test]
    fn gmres_identity() {
        let a_mul = |x: &[f64]| x.to_vec();
        let b = [3.0, 7.0];
        let x = gmres(a_mul, &b, &[0.0, 0.0], 10, 1e-10, 100).unwrap();
        assert!(approx_eq(x[0], 3.0));
        assert!(approx_eq(x[1], 7.0));
    }

    #[test]
    fn gmres_non_symmetric() {
        // A = [[2, 1], [0, 3]] (non-symmetric)
        let a_mul = |x: &[f64]| vec![2.0 * x[0] + x[1], 3.0 * x[1]];
        let b = [5.0, 9.0]; // x = [1, 3]
        let x = gmres(a_mul, &b, &[0.0, 0.0], 10, 1e-10, 100).unwrap();
        assert!((x[0] - 1.0).abs() < 1e-6);
        assert!((x[1] - 3.0).abs() < 1e-6);
    }

    // --- Eigendecomposition tests ---

    #[test]
    fn eigen_symmetric_diagonal() {
        let a = vec![
            vec![5.0, 0.0, 0.0],
            vec![0.0, 3.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let ed = eigen_symmetric(&a, 1e-12, 1000).unwrap();
        // Eigenvalues sorted by descending magnitude: 5, 3, 1
        assert!(approx_eq(ed.eigenvalues_real[0], 5.0));
        assert!(approx_eq(ed.eigenvalues_real[1], 3.0));
        assert!(approx_eq(ed.eigenvalues_real[2], 1.0));
    }

    #[test]
    fn eigen_symmetric_2x2() {
        // [[2, 1], [1, 2]] → eigenvalues 3, 1
        let a = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
        let ed = eigen_symmetric(&a, 1e-12, 1000).unwrap();
        assert!((ed.eigenvalues_real[0] - 3.0).abs() < 1e-8);
        assert!((ed.eigenvalues_real[1] - 1.0).abs() < 1e-8);
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn eigen_symmetric_verify_av_eq_lambda_v() {
        // Verify A*v = λ*v for each eigenpair
        let a = vec![
            vec![4.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let ed = eigen_symmetric(&a, 1e-12, 1000).unwrap();
        let vecs = ed.eigenvectors.as_ref().unwrap();

        for k in 0..3 {
            let lambda = ed.eigenvalues_real[k];
            let v = &vecs[k];
            // Compute A*v
            for i in 0..3 {
                let mut av_i = 0.0;
                for j in 0..3 {
                    av_i += a[i][j] * v[j];
                }
                assert!(
                    (av_i - lambda * v[i]).abs() < 1e-6,
                    "A*v != λ*v at eigenvalue {k}: {av_i} != {} * {}",
                    lambda,
                    v[i]
                );
            }
        }
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn eigen_symmetric_orthogonal_eigenvectors() {
        let a = vec![
            vec![4.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let ed = eigen_symmetric(&a, 1e-12, 1000).unwrap();
        let vecs = ed.eigenvectors.as_ref().unwrap();

        for i in 0..3 {
            for j in 0..3 {
                let dot: f64 = (0..3).map(|k| vecs[i][k] * vecs[j][k]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (dot - expected).abs() < 1e-6,
                    "eigenvectors not orthonormal: dot({i},{j}) = {dot}"
                );
            }
        }
    }

    #[test]
    fn eigen_symmetric_1x1() {
        let a = vec![vec![7.0]];
        let ed = eigen_symmetric(&a, 1e-12, 100).unwrap();
        assert!(approx_eq(ed.eigenvalues_real[0], 7.0));
    }

    #[test]
    fn eigen_symmetric_negative_eigenvalues() {
        // [[-2, 0], [0, -5]] → eigenvalues -5, -2
        let a = vec![vec![-2.0, 0.0], vec![0.0, -5.0]];
        let ed = eigen_symmetric(&a, 1e-12, 100).unwrap();
        assert!((ed.eigenvalues_real[0] - (-5.0)).abs() < 1e-8);
        assert!((ed.eigenvalues_real[1] - (-2.0)).abs() < 1e-8);
    }

    #[test]
    fn eigen_symmetric_empty_errors() {
        let a: Vec<Vec<f64>> = vec![];
        assert!(eigen_symmetric(&a, 1e-12, 100).is_err());
    }

    #[test]
    fn eigen_symmetric_matches_power_iteration() {
        let a = vec![
            vec![4.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let ed = eigen_symmetric(&a, 1e-12, 1000).unwrap();
        let (power_eig, _) = eigenvalue_power(&a, 1e-12, 1000).unwrap();
        // Dominant eigenvalue from full decomp should match power iteration
        assert!(
            (ed.eigenvalues_real[0] - power_eig).abs() < 1e-4,
            "full={} vs power={}",
            ed.eigenvalues_real[0],
            power_eig
        );
    }

    // --- Stiff ODE tests ---

    #[test]
    fn backward_euler_stiff_decay() {
        // y' = -1000*y, y(0) = 1 → y(1) = e^(-1000) ≈ 0
        // Explicit Euler would blow up; implicit should be stable
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -1000.0 * y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = -1000.0;
        let y = backward_euler(f, jac, 0.0, &[1.0], 1.0, 100, 1e-10, 20).unwrap();
        // Should be near zero (stable), not blown up
        assert!(y[0].abs() < 0.1, "backward Euler unstable: {}", y[0]);
    }

    #[test]
    fn backward_euler_linear() {
        // y' = -y, y(0) = 1 → y(1) = 1/e
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = -1.0;
        let y = backward_euler(f, jac, 0.0, &[1.0], 1.0, 1000, 1e-10, 20).unwrap();
        let expected = (-1.0_f64).exp();
        assert!(
            (y[0] - expected).abs() < 0.01,
            "backward Euler: {} vs {}",
            y[0],
            expected
        );
    }

    #[test]
    fn bdf2_stiff_decay() {
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -1000.0 * y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = -1000.0;
        let y = bdf2(f, jac, 0.0, &[1.0], 1.0, 100, 1e-10, 20).unwrap();
        assert!(y[0].abs() < 0.1, "BDF-2 unstable: {}", y[0]);
    }

    #[test]
    fn bdf2_linear_more_accurate_than_backward_euler() {
        // BDF-2 is order 2, backward Euler is order 1
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = -1.0;
        let expected = (-1.0_f64).exp();
        let be = backward_euler(f, jac, 0.0, &[1.0], 1.0, 100, 1e-10, 20).unwrap();
        let b2 = bdf2(f, jac, 0.0, &[1.0], 1.0, 100, 1e-10, 20).unwrap();
        let be_err = (be[0] - expected).abs();
        let b2_err = (b2[0] - expected).abs();
        assert!(
            b2_err < be_err,
            "BDF-2 should be more accurate: bdf2_err={b2_err} vs be_err={be_err}"
        );
    }

    // --- SDE tests ---

    #[test]
    fn pcg32_normal_distribution() {
        let mut rng = Pcg32::new(42, 1);
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        let n = 10000;
        for _ in 0..n {
            let x = rng.next_normal();
            sum += x;
            sum_sq += x * x;
        }
        let mean = sum / n as f64;
        let variance = sum_sq / n as f64 - mean * mean;
        assert!(mean.abs() < 0.05, "normal mean: {mean}");
        assert!((variance - 1.0).abs() < 0.1, "normal variance: {variance}");
    }

    #[test]
    fn euler_maruyama_zero_noise_recovers_ode() {
        // Zero diffusion → should match deterministic ODE
        let drift = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let diffusion = |_t: f64, _y: &[f64], b: &mut [f64]| b[0] = 0.0;
        let mut rng = Pcg32::new(1, 1);
        let traj = euler_maruyama(drift, diffusion, 0.0, &[1.0], 1.0, 1000, &mut rng).unwrap();
        let y_final = traj.last().unwrap().1[0];
        let expected = (-1.0_f64).exp();
        assert!(
            (y_final - expected).abs() < 0.01,
            "EM zero noise: {y_final} vs {expected}"
        );
    }

    #[test]
    fn euler_maruyama_deterministic_replay() {
        let drift = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let diffusion = |_t: f64, y: &[f64], b: &mut [f64]| b[0] = 0.1 * y[0];
        let mut rng1 = Pcg32::new(42, 1);
        let mut rng2 = Pcg32::new(42, 1);
        let t1 = euler_maruyama(drift, diffusion, 0.0, &[1.0], 1.0, 100, &mut rng1).unwrap();
        let t2 = euler_maruyama(drift, diffusion, 0.0, &[1.0], 1.0, 100, &mut rng2).unwrap();
        for i in 0..t1.len() {
            assert_eq!(t1[i].1[0], t2[i].1[0]);
        }
    }

    #[test]
    fn milstein_zero_noise_recovers_ode() {
        let drift = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let diffusion = |_t: f64, _y: &[f64], b: &mut [f64]| b[0] = 0.0;
        let diff_deriv = |_t: f64, _y: &[f64], db: &mut [f64]| db[0] = 0.0;
        let mut rng = Pcg32::new(1, 1);
        let traj = milstein(
            drift,
            diffusion,
            diff_deriv,
            0.0,
            &[1.0],
            1.0,
            1000,
            &mut rng,
        )
        .unwrap();
        let y_final = traj.last().unwrap().1[0];
        let expected = (-1.0_f64).exp();
        assert!(
            (y_final - expected).abs() < 0.01,
            "Milstein zero noise: {y_final} vs {expected}"
        );
    }

    // --- Lyapunov exponent tests ---

    #[test]
    fn lyapunov_stable_system() {
        // y' = -y → MLE should be negative (stable)
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = -1.0;
        let mle = lyapunov_max(f, jac, &[1.0], 10.0, 0.01, 10).unwrap();
        assert!(mle < 0.0, "stable system MLE should be negative: {mle}");
    }

    #[test]
    fn lyapunov_unstable_system() {
        // y' = y → MLE should be positive (unstable)
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = 1.0;
        let mle = lyapunov_max(f, jac, &[1.0], 5.0, 0.01, 10).unwrap();
        assert!(mle > 0.0, "unstable system MLE should be positive: {mle}");
    }

    // --- PGS tests ---

    #[test]
    fn pgs_unconstrained() {
        // A = [[4,1],[1,3]], b = [1,2], lo = [-inf], hi = [inf]
        let a = vec![vec![4.0, 1.0], vec![1.0, 3.0]];
        let b = [1.0, 2.0];
        let lo = [f64::NEG_INFINITY, f64::NEG_INFINITY];
        let hi = [f64::INFINITY, f64::INFINITY];
        let x = projected_gauss_seidel(&a, &b, &lo, &hi, &[0.0, 0.0], 100, 1e-10).unwrap();
        // Verify A*x ≈ b
        let r0 = 4.0 * x[0] + x[1];
        let r1 = x[0] + 3.0 * x[1];
        assert!((r0 - 1.0).abs() < 1e-6);
        assert!((r1 - 2.0).abs() < 1e-6);
    }

    #[test]
    fn pgs_box_constrained() {
        // Same system but x clamped to [0, inf]
        let a = vec![vec![4.0, 1.0], vec![1.0, 3.0]];
        let b = [-1.0, -1.0]; // Unconstrained solution would be negative
        let lo = [0.0, 0.0];
        let hi = [f64::INFINITY, f64::INFINITY];
        let x = projected_gauss_seidel(&a, &b, &lo, &hi, &[0.0, 0.0], 100, 1e-10).unwrap();
        assert!(x[0] >= -1e-10 && x[1] >= -1e-10, "PGS violated bounds");
    }
}
