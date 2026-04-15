use crate::HisabError;

// ---------------------------------------------------------------------------
// Sparse matrix (CSR format)
// ---------------------------------------------------------------------------

/// A sparse matrix in Compressed Sparse Row (CSR) format.
///
/// Stores only non-zero elements. Efficient for sparse matrix-vector multiply
/// and row-based access patterns.
///
/// - `values`: non-zero entries, row by row.
/// - `col_indices`: column index of each value.
/// - `row_offsets`: `row_offsets[i]` is the index into `values` where row `i` starts.
///   Length is `nrows + 1`; `row_offsets[nrows]` equals `values.len()`.
#[derive(Debug, Clone, PartialEq)]
#[must_use]
pub struct CsrMatrix {
    /// Number of rows.
    pub nrows: usize,
    /// Number of columns.
    pub ncols: usize,
    /// Non-zero values, row by row.
    values: Vec<f64>,
    /// Column index for each value.
    col_indices: Vec<usize>,
    /// Row offset pointers (length = nrows + 1).
    row_offsets: Vec<usize>,
}

impl CsrMatrix {
    /// Create a CSR matrix from raw components.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if the components are inconsistent.
    pub fn new(
        nrows: usize,
        ncols: usize,
        values: Vec<f64>,
        col_indices: Vec<usize>,
        row_offsets: Vec<usize>,
    ) -> Result<Self, HisabError> {
        if row_offsets.len() != nrows + 1 {
            return Err(HisabError::InvalidInput(format!(
                "row_offsets length {} != nrows + 1 ({})",
                row_offsets.len(),
                nrows + 1
            )));
        }
        if values.len() != col_indices.len() {
            return Err(HisabError::InvalidInput(
                "values and col_indices must have equal length".into(),
            ));
        }
        if row_offsets[nrows] != values.len() {
            return Err(HisabError::InvalidInput(
                "row_offsets[nrows] must equal values.len()".into(),
            ));
        }
        // Validate monotonically non-decreasing row_offsets
        for w in row_offsets.windows(2) {
            if w[0] > w[1] {
                return Err(HisabError::InvalidInput(
                    "row_offsets must be monotonically non-decreasing".into(),
                ));
            }
        }
        // Validate column indices: in range and sorted within each row
        for row in 0..nrows {
            let start = row_offsets[row];
            let end = row_offsets[row + 1];
            for idx in start..end {
                if col_indices[idx] >= ncols {
                    return Err(HisabError::InvalidInput(format!(
                        "column index {} >= ncols {ncols}",
                        col_indices[idx]
                    )));
                }
                if idx > start && col_indices[idx] <= col_indices[idx - 1] {
                    return Err(HisabError::InvalidInput(
                        "column indices must be strictly sorted within each row".into(),
                    ));
                }
            }
        }
        Ok(Self {
            nrows,
            ncols,
            values,
            col_indices,
            row_offsets,
        })
    }

    /// Create a CSR matrix from a dense row-major matrix, dropping zeros.
    pub fn from_dense(a: &[Vec<f64>]) -> Self {
        let nrows = a.len();
        let ncols = if nrows > 0 { a[0].len() } else { 0 };
        let mut values = Vec::new();
        let mut col_indices = Vec::new();
        let mut row_offsets = Vec::with_capacity(nrows + 1);
        row_offsets.push(0);

        for row in a {
            for (j, &v) in row.iter().enumerate() {
                if v.abs() > crate::EPSILON_F64 {
                    values.push(v);
                    col_indices.push(j);
                }
            }
            row_offsets.push(values.len());
        }

        Self {
            nrows,
            ncols,
            values,
            col_indices,
            row_offsets,
        }
    }

    /// Convert to a dense row-major matrix.
    #[must_use]
    pub fn to_dense(&self) -> Vec<Vec<f64>> {
        let mut a = vec![vec![0.0; self.ncols]; self.nrows];
        for (i, row) in a.iter_mut().enumerate() {
            for idx in self.row_offsets[i]..self.row_offsets[i + 1] {
                row[self.col_indices[idx]] = self.values[idx];
            }
        }
        a
    }

    /// Number of non-zero entries.
    #[must_use]
    #[inline]
    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    /// Sparse matrix-vector multiply: `y = A * x`.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if `x.len() != ncols`.
    #[must_use = "returns the product vector or an error"]
    #[inline]
    pub fn spmv(&self, x: &[f64]) -> Result<Vec<f64>, HisabError> {
        if x.len() != self.ncols {
            return Err(HisabError::InvalidInput(format!(
                "x length {} != ncols {}",
                x.len(),
                self.ncols
            )));
        }
        let mut y = vec![0.0; self.nrows];
        for (i, yi) in y.iter_mut().enumerate() {
            let mut sum = 0.0;
            for idx in self.row_offsets[i]..self.row_offsets[i + 1] {
                sum += self.values[idx] * x[self.col_indices[idx]];
            }
            *yi = sum;
        }
        Ok(y)
    }

    /// Sparse matrix-transpose-vector multiply: y = Aᵀ · x.
    ///
    /// Computes the product of the transpose of this matrix with a vector,
    /// without explicitly forming the transpose.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if `x` length doesn't match `nrows`.
    #[must_use = "returns the product vector or an error"]
    pub fn spmvt(&self, x: &[f64]) -> Result<Vec<f64>, HisabError> {
        if x.len() != self.nrows {
            return Err(HisabError::InvalidInput(format!(
                "x length {} != nrows {}",
                x.len(),
                self.nrows
            )));
        }
        let mut y = vec![0.0; self.ncols];
        for (i, xi) in x.iter().enumerate().take(self.nrows) {
            for idx in self.row_offsets[i]..self.row_offsets[i + 1] {
                y[self.col_indices[idx]] += self.values[idx] * xi;
            }
        }
        Ok(y)
    }

    /// Add two CSR matrices of the same dimensions.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if dimensions don't match.
    pub fn add(&self, other: &CsrMatrix) -> Result<CsrMatrix, HisabError> {
        if self.nrows != other.nrows || self.ncols != other.ncols {
            return Err(HisabError::InvalidInput(format!(
                "dimension mismatch: {}x{} vs {}x{}",
                self.nrows, self.ncols, other.nrows, other.ncols
            )));
        }

        let mut values = Vec::new();
        let mut col_indices = Vec::new();
        let mut row_offsets = Vec::with_capacity(self.nrows + 1);
        row_offsets.push(0);

        for i in 0..self.nrows {
            // Merge two sorted row segments
            let mut a_idx = self.row_offsets[i];
            let a_end = self.row_offsets[i + 1];
            let mut b_idx = other.row_offsets[i];
            let b_end = other.row_offsets[i + 1];

            while a_idx < a_end && b_idx < b_end {
                let a_col = self.col_indices[a_idx];
                let b_col = other.col_indices[b_idx];
                match a_col.cmp(&b_col) {
                    std::cmp::Ordering::Less => {
                        values.push(self.values[a_idx]);
                        col_indices.push(a_col);
                        a_idx += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        values.push(other.values[b_idx]);
                        col_indices.push(b_col);
                        b_idx += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        let sum = self.values[a_idx] + other.values[b_idx];
                        if sum.abs() > crate::EPSILON_F64 {
                            values.push(sum);
                            col_indices.push(a_col);
                        }
                        a_idx += 1;
                        b_idx += 1;
                    }
                }
            }
            while a_idx < a_end {
                values.push(self.values[a_idx]);
                col_indices.push(self.col_indices[a_idx]);
                a_idx += 1;
            }
            while b_idx < b_end {
                values.push(other.values[b_idx]);
                col_indices.push(other.col_indices[b_idx]);
                b_idx += 1;
            }
            row_offsets.push(values.len());
        }

        Ok(CsrMatrix {
            nrows: self.nrows,
            ncols: self.ncols,
            values,
            col_indices,
            row_offsets,
        })
    }

    /// Get the value at (row, col), returning 0.0 if the entry is not stored.
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> f64 {
        if row >= self.nrows || col >= self.ncols {
            return 0.0;
        }
        let start = self.row_offsets[row];
        let end = self.row_offsets[row + 1];
        // Binary search within the row
        match self.col_indices[start..end].binary_search(&col) {
            Ok(offset) => self.values[start + offset],
            Err(_) => 0.0,
        }
    }

    /// Transpose this matrix, returning a new CSR matrix.
    pub fn transpose(&self) -> CsrMatrix {
        let mut row_counts = vec![0usize; self.ncols];
        for &c in &self.col_indices {
            row_counts[c] += 1;
        }

        let mut new_offsets = Vec::with_capacity(self.ncols + 1);
        let mut cumulative = 0usize;
        new_offsets.push(0);
        for &count in &row_counts {
            cumulative += count;
            new_offsets.push(cumulative);
        }

        let mut new_values = vec![0.0; self.nnz()];
        let mut new_col_indices = vec![0usize; self.nnz()];
        let mut cursor = new_offsets[..self.ncols].to_vec();

        for i in 0..self.nrows {
            for idx in self.row_offsets[i]..self.row_offsets[i + 1] {
                let col = self.col_indices[idx];
                let dest = cursor[col];
                new_values[dest] = self.values[idx];
                new_col_indices[dest] = i;
                cursor[col] += 1;
            }
        }

        CsrMatrix {
            nrows: self.ncols,
            ncols: self.nrows,
            values: new_values,
            col_indices: new_col_indices,
            row_offsets: new_offsets,
        }
    }
}

// ---------------------------------------------------------------------------
// Sparse Cholesky factorization (for SPD matrices)
// ---------------------------------------------------------------------------

/// Sparse Cholesky factorization: decompose a symmetric positive-definite
/// sparse matrix A into L·Lᵀ, then solve A·x = b.
///
/// This is an up-looking left-Cholesky that computes L row by row.
/// The sparsity pattern of L is computed on-the-fly (symbolic + numeric
/// combined). No fill-reducing reordering is applied.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is not square or `b` length mismatches.
/// Returns [`HisabError::SingularMatrix`] if the matrix is not positive-definite.
#[allow(clippy::needless_range_loop)]
pub fn sparse_cholesky_solve(a: &CsrMatrix, b: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = a.nrows;
    if n != a.ncols {
        return Err(HisabError::InvalidInput("matrix must be square".into()));
    }
    if b.len() != n {
        return Err(HisabError::InvalidInput("b length must match n".into()));
    }

    // Build dense lower triangle L (for simplicity with fill-in)
    // For large sparse systems, a dedicated sparse Cholesky with symbolic
    // analysis would be more memory-efficient.
    let mut l = vec![vec![0.0; n]; n];

    for i in 0..n {
        let mut sum_diag = a.get(i, i);

        for k in 0..i {
            sum_diag -= l[i][k] * l[i][k];
        }

        if sum_diag <= 0.0 {
            return Err(HisabError::SingularMatrix);
        }
        l[i][i] = sum_diag.sqrt();
        let l_ii_inv = 1.0 / l[i][i];

        for j in (i + 1)..n {
            let mut sum = a.get(j, i);
            for k in 0..i {
                sum -= l[j][k] * l[i][k];
            }
            l[j][i] = sum * l_ii_inv;
        }
    }

    // Forward substitution: L·y = b
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut sum = b[i];
        for k in 0..i {
            sum -= l[i][k] * y[k];
        }
        y[i] = sum / l[i][i];
    }

    // Backward substitution: Lᵀ·x = y
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = y[i];
        for k in (i + 1)..n {
            sum -= l[k][i] * x[k];
        }
        x[i] = sum / l[i][i];
    }

    Ok(x)
}

// ---------------------------------------------------------------------------
// Sparse LU factorization
// ---------------------------------------------------------------------------

/// Sparse LU factorization with partial pivoting: solve A·x = b.
///
/// Converts to dense for the factorization (practical for moderate-size
/// sparse systems up to ~1000×1000). For very large systems, use iterative
/// solvers like [`super::bicgstab`] or [`super::gmres`].
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is not square or `b` length mismatches.
/// Returns [`HisabError::SingularMatrix`] if the matrix is singular.
pub fn sparse_lu_solve(a: &CsrMatrix, b: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = a.nrows;
    if n != a.ncols {
        return Err(HisabError::InvalidInput("matrix must be square".into()));
    }
    if b.len() != n {
        return Err(HisabError::InvalidInput("b length must match n".into()));
    }

    // Convert to dense augmented matrix [A|b] and use Gaussian elimination
    let mut aug = Vec::with_capacity(n);
    for (i, &bi) in b.iter().enumerate().take(n) {
        let mut row = Vec::with_capacity(n + 1);
        for j in 0..n {
            row.push(a.get(i, j));
        }
        row.push(bi);
        aug.push(row);
    }

    super::roots::gaussian_elimination(&mut aug)
}
