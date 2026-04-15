//! Storage-efficient symmetric and antisymmetric tensors.
//!
//! [`SymmetricTensor`] stores only the independent components of a fully
//! symmetric rank-k tensor in n dimensions, reducing the Riemann tensor's
//! 256 components to 20, for example.
//!
//! [`AntisymmetricTensor`] stores only the independent components of a fully
//! antisymmetric (alternating) tensor.

use crate::HisabError;

// ---------------------------------------------------------------------------
// SymmetricTensor
// ---------------------------------------------------------------------------

/// A fully symmetric tensor of given rank and dimension.
///
/// Stores only the independent components using the multiset coefficient
/// formula: C(n + k - 1, k) independent entries for rank k in n dimensions.
///
/// # Examples
///
/// ```
/// use hisab::tensor::SymmetricTensor;
///
/// // Symmetric 2-tensor in 4D (metric tensor) = 10 independent components
/// let mut g = SymmetricTensor::zeros(4, 2);
/// g.set(&[0, 0], 1.0).unwrap();
/// g.set(&[1, 1], -1.0).unwrap();
/// assert_eq!(g.num_independent(), 10);
/// assert!((g.get(&[0, 0]).unwrap() - 1.0).abs() < 1e-12);
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SymmetricTensor {
    /// Dimension of each index.
    dim: usize,
    /// Number of indices (rank).
    rank: usize,
    /// Independent components in canonical (sorted) index order.
    data: Vec<f64>,
}

impl SymmetricTensor {
    /// Create a zero-filled symmetric tensor.
    #[must_use]
    pub fn zeros(dim: usize, rank: usize) -> Self {
        let count = multiset_coeff(dim, rank);
        Self {
            dim,
            rank,
            data: vec![0.0; count],
        }
    }

    /// Number of independent components.
    #[must_use]
    #[inline]
    pub fn num_independent(&self) -> usize {
        self.data.len()
    }

    /// Dimension of each index.
    #[must_use]
    #[inline]
    pub fn dim(&self) -> usize {
        self.dim
    }

    /// Rank of the tensor.
    #[must_use]
    #[inline]
    pub fn rank(&self) -> usize {
        self.rank
    }

    /// Get element at multi-index (order doesn't matter — indices are sorted).
    ///
    /// # Errors
    ///
    /// Returns error if index count or values are wrong.
    pub fn get(&self, indices: &[usize]) -> Result<f64, HisabError> {
        let flat = self.canonical_index(indices)?;
        Ok(self.data[flat])
    }

    /// Set element at multi-index (order doesn't matter — indices are sorted).
    ///
    /// # Errors
    ///
    /// Returns error if index count or values are wrong.
    pub fn set(&mut self, indices: &[usize], value: f64) -> Result<(), HisabError> {
        let flat = self.canonical_index(indices)?;
        self.data[flat] = value;
        Ok(())
    }

    /// Raw data slice (canonical ordering).
    #[must_use]
    #[inline]
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    /// Expand to a full dense tensor (all permutations filled).
    #[must_use]
    #[allow(clippy::needless_range_loop)]
    pub fn to_dense(&self) -> Vec<f64> {
        let total = self.dim.pow(self.rank as u32);
        let mut dense = vec![0.0; total];
        let mut idx = vec![0usize; self.rank];

        for flat in 0..total {
            // Convert flat to multi-index
            let mut remainder = flat;
            for i in (0..self.rank).rev() {
                idx[i] = remainder % self.dim;
                remainder /= self.dim;
            }
            // Look up using canonical (sorted) form
            if let Ok(val) = self.get(&idx) {
                dense[flat] = val;
            }
        }
        dense
    }

    /// Map a multi-index to canonical (sorted) flat index.
    fn canonical_index(&self, indices: &[usize]) -> Result<usize, HisabError> {
        if indices.len() != self.rank {
            return Err(HisabError::InvalidInput(format!(
                "expected {} indices, got {}",
                self.rank,
                indices.len()
            )));
        }
        for &i in indices {
            if i >= self.dim {
                return Err(HisabError::OutOfRange(format!(
                    "index {i} out of range for dimension {}",
                    self.dim
                )));
            }
        }

        // Sort indices for canonical form
        let mut sorted = indices.to_vec();
        sorted.sort_unstable();

        // Compute combinatorial index for sorted multiset
        // Using the combinatorial number system for multisets
        multiset_to_flat(&sorted, self.dim)
    }
}

impl std::fmt::Display for SymmetricTensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SymmetricTensor(dim={}, rank={}, {} independent)",
            self.dim,
            self.rank,
            self.data.len()
        )
    }
}

// ---------------------------------------------------------------------------
// AntisymmetricTensor
// ---------------------------------------------------------------------------

/// A fully antisymmetric tensor of given rank and dimension.
///
/// Stores only C(n, k) independent components. Accessing with permuted indices
/// returns the value with the appropriate sign.
///
/// # Examples
///
/// ```
/// use hisab::tensor::AntisymmetricTensor;
///
/// // Antisymmetric 2-tensor in 4D (electromagnetic field tensor) = 6 independent
/// let mut f = AntisymmetricTensor::zeros(4, 2);
/// f.set(&[0, 1], 1.0).unwrap();
/// assert!((f.get(&[1, 0]).unwrap() + 1.0).abs() < 1e-12); // antisymmetric
/// assert_eq!(f.num_independent(), 6);
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AntisymmetricTensor {
    dim: usize,
    rank: usize,
    data: Vec<f64>,
}

impl AntisymmetricTensor {
    /// Create a zero-filled antisymmetric tensor.
    #[must_use]
    pub fn zeros(dim: usize, rank: usize) -> Self {
        let count = binomial(dim, rank);
        Self {
            dim,
            rank,
            data: vec![0.0; count],
        }
    }

    /// Number of independent components.
    #[must_use]
    #[inline]
    pub fn num_independent(&self) -> usize {
        self.data.len()
    }

    /// Dimension of each index.
    #[must_use]
    #[inline]
    pub fn dim(&self) -> usize {
        self.dim
    }

    /// Rank of the tensor.
    #[must_use]
    #[inline]
    pub fn rank(&self) -> usize {
        self.rank
    }

    /// Get element at multi-index with sign from permutation parity.
    ///
    /// Returns 0 if any indices are repeated.
    ///
    /// # Errors
    ///
    /// Returns error if index count or values are wrong.
    pub fn get(&self, indices: &[usize]) -> Result<f64, HisabError> {
        if indices.len() != self.rank {
            return Err(HisabError::InvalidInput(format!(
                "expected {} indices, got {}",
                self.rank,
                indices.len()
            )));
        }
        for &i in indices {
            if i >= self.dim {
                return Err(HisabError::OutOfRange(format!(
                    "index {i} out of range for dimension {}",
                    self.dim
                )));
            }
        }

        // Check for repeated indices
        let mut sorted = indices.to_vec();
        let sign = sort_and_count_sign(&mut sorted);
        if sign == 0 {
            return Ok(0.0);
        }

        let flat = strictly_increasing_to_flat(&sorted, self.dim)?;
        Ok(self.data[flat] * sign as f64)
    }

    /// Set element at canonical (strictly increasing) multi-index.
    ///
    /// For antisymmetric tensors, you should only set the canonical ordering
    /// where `indices[0] < indices[1] < ...`. The sign is handled automatically
    /// on read.
    ///
    /// # Errors
    ///
    /// Returns error if index count or values are wrong.
    pub fn set(&mut self, indices: &[usize], value: f64) -> Result<(), HisabError> {
        if indices.len() != self.rank {
            return Err(HisabError::InvalidInput(format!(
                "expected {} indices, got {}",
                self.rank,
                indices.len()
            )));
        }
        for &i in indices {
            if i >= self.dim {
                return Err(HisabError::OutOfRange(format!(
                    "index {i} out of range for dimension {}",
                    self.dim
                )));
            }
        }

        let mut sorted = indices.to_vec();
        let sign = sort_and_count_sign(&mut sorted);
        if sign == 0 {
            // Repeated indices — antisymmetric component is forced to 0
            return Ok(());
        }

        let flat = strictly_increasing_to_flat(&sorted, self.dim)?;
        self.data[flat] = value * sign as f64;
        Ok(())
    }

    /// Raw data slice (canonical ordering).
    #[must_use]
    #[inline]
    pub fn data(&self) -> &[f64] {
        &self.data
    }
}

impl std::fmt::Display for AntisymmetricTensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AntisymmetricTensor(dim={}, rank={}, {} independent)",
            self.dim,
            self.rank,
            self.data.len()
        )
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Multiset coefficient: C(n + k - 1, k) = number of independent components
/// of a symmetric rank-k tensor in n dimensions.
#[must_use]
fn multiset_coeff(n: usize, k: usize) -> usize {
    if k == 0 {
        return 1;
    }
    binomial(n + k - 1, k)
}

/// Binomial coefficient C(n, k).
#[must_use]
fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result = 1usize;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

/// Convert a sorted multiset index to a flat position.
///
/// For a symmetric tensor, the canonical form has sorted indices.
/// We enumerate sorted tuples in lexicographic order.
fn multiset_to_flat(sorted: &[usize], dim: usize) -> Result<usize, HisabError> {
    let k = sorted.len();
    if k == 0 {
        return Ok(0);
    }

    // Enumerate using combinatorial number system for multisets
    let mut flat = 0;
    let mut prev = 0;
    for (pos, &idx) in sorted.iter().enumerate() {
        // Count how many canonical tuples come before this one at position `pos`
        for v in prev..idx {
            flat += multiset_coeff(dim - v, k - pos - 1);
        }
        // For multisets (not strictly increasing), the minimum next value can equal current
        prev = idx;
    }
    Ok(flat)
}

/// Sort indices and return the permutation sign (+1 or -1), or 0 if repeated.
fn sort_and_count_sign(indices: &mut [usize]) -> i32 {
    let n = indices.len();
    let mut sign = 1i32;
    // Bubble sort to count transpositions
    for i in 0..n {
        for j in 0..n - 1 - i {
            if indices[j] > indices[j + 1] {
                indices.swap(j, j + 1);
                sign = -sign;
            } else if indices[j] == indices[j + 1] {
                return 0;
            }
        }
    }
    sign
}

/// Convert a strictly increasing index list to flat position (for antisymmetric).
fn strictly_increasing_to_flat(sorted: &[usize], dim: usize) -> Result<usize, HisabError> {
    let k = sorted.len();
    if k == 0 {
        return Ok(0);
    }

    // Combinatorial number system: flat = Σ C(sorted[i], i+1) doesn't work
    // directly, use enumeration of k-subsets of {0..n-1}
    let mut flat = 0;
    let mut prev = 0;
    for (pos, &idx) in sorted.iter().enumerate() {
        for v in prev..idx {
            flat += binomial(dim - 1 - v, k - 1 - pos);
        }
        prev = idx + 1;
    }
    Ok(flat)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-12;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < TOL
    }

    // -- SymmetricTensor --

    #[test]
    fn symmetric_2_tensor_4d() {
        // 4D symmetric rank-2 = C(5, 2) = 10 independent components
        let g = SymmetricTensor::zeros(4, 2);
        assert_eq!(g.num_independent(), 10);
    }

    #[test]
    fn symmetric_get_set() {
        let mut t = SymmetricTensor::zeros(3, 2);
        t.set(&[0, 1], 5.0).unwrap();
        // Symmetric: t[0,1] == t[1,0]
        assert!(approx(t.get(&[0, 1]).unwrap(), 5.0));
        assert!(approx(t.get(&[1, 0]).unwrap(), 5.0));
    }

    #[test]
    fn symmetric_diagonal() {
        let mut t = SymmetricTensor::zeros(4, 2);
        t.set(&[2, 2], std::f64::consts::PI).unwrap();
        assert!(approx(t.get(&[2, 2]).unwrap(), std::f64::consts::PI));
    }

    #[test]
    fn symmetric_riemann_storage() {
        // Rank-4 symmetric tensor pair in 4D has C(7, 4) = 35 components
        // But actual Riemann has more structure. A fully symmetric rank-4 in 4D:
        // C(4 + 4 - 1, 4) = C(7, 4) = 35
        let r = SymmetricTensor::zeros(4, 4);
        assert_eq!(r.num_independent(), 35);
    }

    #[test]
    fn symmetric_to_dense() {
        let mut t = SymmetricTensor::zeros(2, 2);
        t.set(&[0, 0], 1.0).unwrap();
        t.set(&[0, 1], 2.0).unwrap();
        t.set(&[1, 1], 3.0).unwrap();
        let dense = t.to_dense();
        // Should be: [[1, 2], [2, 3]]
        assert_eq!(dense.len(), 4);
        assert!(approx(dense[0], 1.0)); // [0,0]
        assert!(approx(dense[1], 2.0)); // [0,1]
        assert!(approx(dense[2], 2.0)); // [1,0]
        assert!(approx(dense[3], 3.0)); // [1,1]
    }

    #[test]
    fn symmetric_rank0() {
        let t = SymmetricTensor::zeros(4, 0);
        assert_eq!(t.num_independent(), 1);
    }

    // -- AntisymmetricTensor --

    #[test]
    fn antisymmetric_2_tensor_4d() {
        // 4D antisymmetric rank-2 = C(4, 2) = 6 independent components
        let f = AntisymmetricTensor::zeros(4, 2);
        assert_eq!(f.num_independent(), 6);
    }

    #[test]
    fn antisymmetric_sign() {
        let mut f = AntisymmetricTensor::zeros(4, 2);
        f.set(&[0, 1], 5.0).unwrap();
        assert!(approx(f.get(&[0, 1]).unwrap(), 5.0));
        assert!(approx(f.get(&[1, 0]).unwrap(), -5.0)); // antisymmetric
    }

    #[test]
    fn antisymmetric_repeated_zero() {
        let mut f = AntisymmetricTensor::zeros(3, 2);
        f.set(&[0, 0], 999.0).unwrap(); // should be forced to 0
        assert!(approx(f.get(&[0, 0]).unwrap(), 0.0));
    }

    #[test]
    fn antisymmetric_em_tensor() {
        // Electromagnetic field tensor F_μν, 4D antisymmetric rank-2
        let mut f = AntisymmetricTensor::zeros(4, 2);
        // Set E_x = F_{01}
        f.set(&[0, 1], 1.0).unwrap();
        // F_{10} = -F_{01}
        assert!(approx(f.get(&[1, 0]).unwrap(), -1.0));
    }

    #[test]
    fn antisymmetric_rank3() {
        // 4D rank-3 antisymmetric = C(4, 3) = 4
        let t = AntisymmetricTensor::zeros(4, 3);
        assert_eq!(t.num_independent(), 4);
    }

    #[test]
    fn binomial_known() {
        assert_eq!(binomial(4, 2), 6);
        assert_eq!(binomial(10, 3), 120);
        assert_eq!(binomial(5, 0), 1);
        assert_eq!(binomial(5, 5), 1);
        assert_eq!(binomial(0, 0), 1);
    }

    #[test]
    fn multiset_coeff_known() {
        // C(n+k-1, k)
        assert_eq!(multiset_coeff(4, 2), 10); // C(5,2)
        assert_eq!(multiset_coeff(3, 3), 10); // C(5,3)
        assert_eq!(multiset_coeff(4, 0), 1);
    }

    // -- Error paths --

    #[test]
    fn symmetric_get_wrong_index_count() {
        let t = SymmetricTensor::zeros(3, 2);
        assert!(t.get(&[0, 1, 2]).is_err());
    }

    #[test]
    fn symmetric_set_wrong_index_count() {
        let mut t = SymmetricTensor::zeros(3, 2);
        assert!(t.set(&[0], 1.0).is_err());
    }

    #[test]
    fn symmetric_get_out_of_range() {
        let t = SymmetricTensor::zeros(3, 2);
        assert!(t.get(&[0, 5]).is_err());
    }

    #[test]
    fn symmetric_set_out_of_range() {
        let mut t = SymmetricTensor::zeros(3, 2);
        assert!(t.set(&[3, 0], 1.0).is_err());
    }

    #[test]
    fn antisymmetric_get_wrong_index_count() {
        let t = AntisymmetricTensor::zeros(4, 2);
        assert!(t.get(&[0]).is_err());
    }

    #[test]
    fn antisymmetric_set_wrong_index_count() {
        let mut t = AntisymmetricTensor::zeros(4, 2);
        assert!(t.set(&[0, 1, 2], 1.0).is_err());
    }

    #[test]
    fn antisymmetric_get_out_of_range() {
        let t = AntisymmetricTensor::zeros(4, 2);
        assert!(t.get(&[0, 7]).is_err());
    }

    #[test]
    fn antisymmetric_set_out_of_range() {
        let mut t = AntisymmetricTensor::zeros(4, 2);
        assert!(t.set(&[5, 0], 1.0).is_err());
    }
}
