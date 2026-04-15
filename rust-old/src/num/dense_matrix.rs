//! Row-major dense matrix backed by a flat `Vec<f64>`.
//!
//! [`DenseMatrix`] stores an *m × n* matrix as a single contiguous allocation,
//! which is cache-friendly for row-wise access patterns and avoids the pointer
//! indirection of `Vec<Vec<f64>>`.

use crate::HisabError;

// ---------------------------------------------------------------------------

/// Row-major dense matrix stored as a flat `Vec<f64>`.
///
/// Indexing is `row * cols + col`. All public mutating operations return
/// `&mut Self` or take `&mut self` — there are no hidden reallocations after
/// construction.
///
/// # Examples
///
/// ```
/// use hisab::num::DenseMatrix;
///
/// let mut m = DenseMatrix::zeros(2, 3);
/// m.set(0, 1, 7.0);
/// assert_eq!(m.get(0, 1), 7.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DenseMatrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl DenseMatrix {
    // -----------------------------------------------------------------------
    // Constructors

    /// Create a zero-filled *rows × cols* matrix.
    #[must_use]
    #[inline]
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    /// Create an *n × n* identity matrix.
    #[must_use]
    #[inline]
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.data[i * n + i] = 1.0;
        }
        m
    }

    /// Construct from a flat row-major `Vec<f64>`.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if `data.len() != rows * cols`.
    #[must_use = "returns the matrix or an error"]
    pub fn from_rows(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, HisabError> {
        if data.len() != rows * cols {
            return Err(HisabError::InvalidInput(alloc_msg(
                "data length",
                data.len(),
                rows * cols,
            )));
        }
        Ok(Self { data, rows, cols })
    }

    /// Construct from a slice of row vectors.
    ///
    /// All rows must have the same length.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if the input is empty or rows have
    /// inconsistent lengths.
    #[must_use = "returns the matrix or an error"]
    pub fn from_vec_of_vec(v: &[Vec<f64>]) -> Result<Self, HisabError> {
        if v.is_empty() {
            return Err(HisabError::InvalidInput("empty row list".into()));
        }
        let cols = v[0].len();
        let rows = v.len();
        let mut data = Vec::with_capacity(rows * cols);
        for (r, row) in v.iter().enumerate() {
            if row.len() != cols {
                return Err(HisabError::InvalidInput(alloc_msg(
                    &format!("row {r} length"),
                    row.len(),
                    cols,
                )));
            }
            data.extend_from_slice(row);
        }
        Ok(Self { data, rows, cols })
    }

    // -----------------------------------------------------------------------
    // Conversions

    /// Convert to `Vec<Vec<f64>>` (row-major).
    #[must_use]
    pub fn to_vec_of_vec(&self) -> Vec<Vec<f64>> {
        (0..self.rows)
            .map(|r| self.data[r * self.cols..(r + 1) * self.cols].to_vec())
            .collect()
    }

    // -----------------------------------------------------------------------
    // Dimensions

    /// Number of rows.
    #[must_use]
    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Number of columns.
    #[must_use]
    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    // -----------------------------------------------------------------------
    // Element access

    /// Read the element at `(row, col)`.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `row >= self.rows || col >= self.cols`.
    #[must_use]
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f64 {
        debug_assert!(row < self.rows && col < self.cols, "index out of bounds");
        self.data[row * self.cols + col]
    }

    /// Mutable reference to the element at `(row, col)`.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `row >= self.rows || col >= self.cols`.
    #[inline]
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        debug_assert!(row < self.rows && col < self.cols, "index out of bounds");
        &mut self.data[row * self.cols + col]
    }

    /// Immutable slice of row `i`.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `i >= self.rows`.
    #[must_use]
    #[inline]
    pub fn row(&self, i: usize) -> &[f64] {
        debug_assert!(i < self.rows, "row index out of bounds");
        &self.data[i * self.cols..(i + 1) * self.cols]
    }

    /// Set the element at `(row, col)` to `val`.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `row >= self.rows || col >= self.cols`.
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        debug_assert!(row < self.rows && col < self.cols, "index out of bounds");
        self.data[row * self.cols + col] = val;
    }

    // -----------------------------------------------------------------------
    // Operations

    /// Matrix-vector multiply: **A** · **x**, returning **y** = **Ax**.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if `x.len() != self.cols`.
    #[must_use = "returns the product vector or an error"]
    pub fn mul_vec(&self, x: &[f64]) -> Result<Vec<f64>, HisabError> {
        if x.len() != self.cols {
            return Err(HisabError::InvalidInput(alloc_msg(
                "vector length",
                x.len(),
                self.cols,
            )));
        }
        let mut out = vec![0.0; self.rows];
        for (r, dst) in out.iter_mut().enumerate() {
            let row = &self.data[r * self.cols..(r + 1) * self.cols];
            // Neumaier-compensated dot product for accuracy.
            let mut sum = 0.0_f64;
            let mut comp = 0.0_f64;
            for c in 0..self.cols {
                let v = row[c] * x[c];
                let t = sum + v;
                if sum.abs() >= v.abs() {
                    comp += (sum - t) + v;
                } else {
                    comp += (v - t) + sum;
                }
                sum = t;
            }
            *dst = sum + comp;
        }
        Ok(out)
    }

    /// Matrix-matrix multiply: **self** · **other**.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if `self.cols != other.rows`.
    #[must_use = "returns the product matrix or an error"]
    pub fn mul_mat(&self, other: &DenseMatrix) -> Result<DenseMatrix, HisabError> {
        if self.cols != other.rows {
            return Err(HisabError::InvalidInput(alloc_msg(
                "self.cols",
                self.cols,
                other.rows,
            )));
        }
        let rows = self.rows;
        let cols = other.cols;
        let inner = self.cols;
        let mut out = DenseMatrix::zeros(rows, cols);
        for r in 0..rows {
            for c in 0..cols {
                // Neumaier-compensated dot product along the inner dimension.
                let mut sum = 0.0_f64;
                let mut comp = 0.0_f64;
                for k in 0..inner {
                    let v = self.data[r * inner + k] * other.data[k * cols + c];
                    let t = sum + v;
                    if sum.abs() >= v.abs() {
                        comp += (sum - t) + v;
                    } else {
                        comp += (v - t) + sum;
                    }
                    sum = t;
                }
                out.data[r * cols + c] = sum + comp;
            }
        }
        Ok(out)
    }

    /// Transpose: returns a new *cols × rows* matrix.
    #[must_use]
    pub fn transpose(&self) -> DenseMatrix {
        let mut out = DenseMatrix::zeros(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                out.data[c * self.rows + r] = self.data[r * self.cols + c];
            }
        }
        out
    }

    /// Frobenius norm: √(∑ aᵢⱼ²).
    #[must_use]
    pub fn frobenius_norm(&self) -> f64 {
        self.data.iter().map(|&v| v * v).sum::<f64>().sqrt()
    }
}

// ---------------------------------------------------------------------------
// Index / IndexMut

impl std::ops::Index<(usize, usize)> for DenseMatrix {
    type Output = f64;

    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &f64 {
        debug_assert!(row < self.rows && col < self.cols, "index out of bounds");
        &self.data[row * self.cols + col]
    }
}

impl std::ops::IndexMut<(usize, usize)> for DenseMatrix {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        debug_assert!(row < self.rows && col < self.cols, "index out of bounds");
        &mut self.data[row * self.cols + col]
    }
}

// ---------------------------------------------------------------------------
// Internal helpers

/// Build a size-mismatch error message without heap allocation via format!.
fn alloc_msg(field: &str, got: usize, expected: usize) -> String {
    let mut s = String::new();
    let _ = std::fmt::write(
        &mut s,
        format_args!("{field}: expected {expected}, got {got}"),
    );
    s
}

// ---------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros_is_all_zero() {
        let m = DenseMatrix::zeros(3, 4);
        for r in 0..3 {
            for c in 0..4 {
                assert_eq!(m.get(r, c), 0.0);
            }
        }
    }

    #[test]
    fn identity_diagonal() {
        let id = DenseMatrix::identity(4);
        for r in 0..4 {
            for c in 0..4 {
                let expected = if r == c { 1.0 } else { 0.0 };
                assert_eq!(id.get(r, c), expected);
            }
        }
    }

    #[test]
    fn from_rows_roundtrip() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let m = DenseMatrix::from_rows(2, 3, data.clone()).unwrap();
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 2), 3.0);
        assert_eq!(m.get(1, 0), 4.0);
        assert_eq!(m.get(1, 2), 6.0);
    }

    #[test]
    fn from_rows_size_mismatch() {
        let result = DenseMatrix::from_rows(2, 3, vec![1.0; 5]);
        assert!(result.is_err());
    }

    #[test]
    fn from_vec_of_vec_and_back() {
        let rows = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let m = DenseMatrix::from_vec_of_vec(&rows).unwrap();
        let back = m.to_vec_of_vec();
        assert_eq!(back, rows);
    }

    #[test]
    fn from_vec_of_vec_inconsistent_cols() {
        let rows = vec![vec![1.0, 2.0], vec![3.0]];
        assert!(DenseMatrix::from_vec_of_vec(&rows).is_err());
    }

    #[test]
    fn from_vec_of_vec_empty() {
        assert!(DenseMatrix::from_vec_of_vec(&[]).is_err());
    }

    #[test]
    fn set_get_roundtrip() {
        let mut m = DenseMatrix::zeros(3, 3);
        m.set(1, 2, 42.0);
        assert_eq!(m.get(1, 2), 42.0);
        // Other cells untouched.
        assert_eq!(m.get(0, 0), 0.0);
    }

    #[test]
    fn index_operator() {
        let mut m = DenseMatrix::zeros(2, 2);
        m[(0, 1)] = 99.0;
        assert_eq!(m[(0, 1)], 99.0);
    }

    #[test]
    fn row_slice() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let m = DenseMatrix::from_rows(2, 3, data).unwrap();
        assert_eq!(m.row(0), &[1.0, 2.0, 3.0]);
        assert_eq!(m.row(1), &[4.0, 5.0, 6.0]);
    }

    #[test]
    fn mul_vec_identity() {
        let id = DenseMatrix::identity(3);
        let x = vec![1.0, 2.0, 3.0];
        let y = id.mul_vec(&x).unwrap();
        assert_eq!(y, x);
    }

    #[test]
    fn mul_vec_known() {
        // [[1,2],[3,4]] * [1,1] = [3,7]
        let m = DenseMatrix::from_rows(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let y = m.mul_vec(&[1.0, 1.0]).unwrap();
        assert!((y[0] - 3.0).abs() < 1e-12);
        assert!((y[1] - 7.0).abs() < 1e-12);
    }

    #[test]
    fn mul_vec_size_mismatch() {
        let m = DenseMatrix::zeros(2, 3);
        assert!(m.mul_vec(&[1.0, 2.0]).is_err());
    }

    #[test]
    fn mul_mat_identity() {
        let m = DenseMatrix::from_rows(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let id = DenseMatrix::identity(2);
        let result = m.mul_mat(&id).unwrap();
        assert_eq!(result, m);
    }

    #[test]
    fn mul_mat_known() {
        // [[1,2],[3,4]] * [[5,6],[7,8]] = [[19,22],[43,50]]
        let a = DenseMatrix::from_rows(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = DenseMatrix::from_rows(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();
        let c = a.mul_mat(&b).unwrap();
        assert!((c.get(0, 0) - 19.0).abs() < 1e-12);
        assert!((c.get(0, 1) - 22.0).abs() < 1e-12);
        assert!((c.get(1, 0) - 43.0).abs() < 1e-12);
        assert!((c.get(1, 1) - 50.0).abs() < 1e-12);
    }

    #[test]
    fn mul_mat_size_mismatch() {
        let a = DenseMatrix::zeros(2, 3);
        let b = DenseMatrix::zeros(2, 2);
        assert!(a.mul_mat(&b).is_err());
    }

    #[test]
    fn transpose_square() {
        let m = DenseMatrix::from_rows(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let t = m.transpose();
        assert_eq!(t.get(0, 0), 1.0);
        assert_eq!(t.get(0, 1), 3.0);
        assert_eq!(t.get(1, 0), 2.0);
        assert_eq!(t.get(1, 1), 4.0);
    }

    #[test]
    fn transpose_rectangular() {
        // 2×3 → 3×2
        let m = DenseMatrix::from_rows(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let t = m.transpose();
        assert_eq!(t.rows(), 3);
        assert_eq!(t.cols(), 2);
        assert_eq!(t.get(0, 0), 1.0);
        assert_eq!(t.get(2, 1), 6.0);
    }

    #[test]
    fn transpose_double_is_identity() {
        let m = DenseMatrix::from_rows(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        assert_eq!(m.transpose().transpose(), m);
    }

    #[test]
    fn frobenius_norm_identity() {
        // Identity n×n has n ones, so Frobenius = sqrt(n).
        let id = DenseMatrix::identity(4);
        assert!((id.frobenius_norm() - 2.0).abs() < 1e-12);
    }

    #[test]
    fn frobenius_norm_zeros() {
        assert_eq!(DenseMatrix::zeros(5, 5).frobenius_norm(), 0.0);
    }

    #[test]
    fn get_mut_modifies() {
        let mut m = DenseMatrix::zeros(2, 2);
        *m.get_mut(1, 0) = 55.0;
        assert_eq!(m.get(1, 0), 55.0);
    }

    #[test]
    fn mul_mat_non_square() {
        // (2×3) * (3×4) = (2×4)
        let a = DenseMatrix::from_rows(2, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0]).unwrap();
        let b = DenseMatrix::from_rows(
            3,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        )
        .unwrap();
        let c = a.mul_mat(&b).unwrap();
        assert_eq!(c.rows(), 2);
        assert_eq!(c.cols(), 4);
        // Row 0 of result = row 0 of b (a row 0 = [1,0,0])
        assert!((c.get(0, 0) - 1.0).abs() < 1e-12);
        // Row 1 of result = row 1 of b (a row 1 = [0,1,0])
        assert!((c.get(1, 0) - 5.0).abs() < 1e-12);
    }
}
