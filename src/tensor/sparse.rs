//! Sparse tensor for high-rank objects with many zero entries.
//!
//! Uses coordinate (COO) format: stores only non-zero entries as
//! `(indices, value)` pairs. Efficient for tensors where most entries are zero,
//! such as structure constants or sparse connection coefficients.

use crate::HisabError;
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// SparseTensor
// ---------------------------------------------------------------------------

/// A sparse N-dimensional tensor in COO (coordinate) format.
///
/// Stores only non-zero entries, indexed by their multi-dimensional position.
/// Well-suited for high-rank tensors with many zeros (e.g. Christoffel symbols,
/// structure constants, Riemann tensor components).
///
/// # Examples
///
/// ```
/// use hisab::tensor::SparseTensor;
///
/// let mut t = SparseTensor::new(vec![4, 4, 4, 4]); // Rank-4 in 4D
/// t.set(&[0, 1, 2, 3], 1.0).unwrap();
/// assert!((t.get(&[0, 1, 2, 3]).unwrap() - 1.0).abs() < 1e-12);
/// assert!((t.get(&[0, 0, 0, 0]).unwrap() - 0.0).abs() < 1e-12);
/// assert_eq!(t.nnz(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SparseTensor {
    /// Shape of each dimension.
    shape: Vec<usize>,
    /// Non-zero entries: flat_index → value.
    entries: BTreeMap<usize, f64>,
}

impl SparseTensor {
    /// Create an empty sparse tensor with the given shape.
    #[must_use]
    pub fn new(shape: Vec<usize>) -> Self {
        Self {
            shape,
            entries: BTreeMap::new(),
        }
    }

    /// Shape of the tensor.
    #[must_use]
    #[inline]
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    /// Number of dimensions (rank).
    #[must_use]
    #[inline]
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    /// Total number of elements if dense.
    #[must_use]
    pub fn total_size(&self) -> usize {
        self.shape.iter().product()
    }

    /// Number of stored non-zero entries.
    #[must_use]
    #[inline]
    pub fn nnz(&self) -> usize {
        self.entries.len()
    }

    /// Sparsity ratio: fraction of elements that are zero.
    #[must_use]
    pub fn sparsity(&self) -> f64 {
        let total = self.total_size();
        if total == 0 {
            return 1.0;
        }
        1.0 - self.entries.len() as f64 / total as f64
    }

    /// Get element at multi-index (returns 0 for unstored entries).
    ///
    /// # Errors
    ///
    /// Returns error if index dimensions don't match or values are out of range.
    pub fn get(&self, indices: &[usize]) -> Result<f64, HisabError> {
        let flat = self.flat_index(indices)?;
        Ok(self.entries.get(&flat).copied().unwrap_or(0.0))
    }

    /// Set element at multi-index. Values near zero are removed.
    ///
    /// # Errors
    ///
    /// Returns error if index dimensions don't match or values are out of range.
    pub fn set(&mut self, indices: &[usize], value: f64) -> Result<(), HisabError> {
        let flat = self.flat_index(indices)?;
        if value.abs() < 1e-300 {
            self.entries.remove(&flat);
        } else {
            self.entries.insert(flat, value);
        }
        Ok(())
    }

    /// Iterate over all non-zero entries as `(flat_index, value)`.
    pub fn iter_nonzero(&self) -> impl Iterator<Item = (&usize, &f64)> {
        self.entries.iter()
    }

    /// Iterate over non-zero entries as `(multi_index, value)`.
    pub fn iter_nonzero_indices(&self) -> impl Iterator<Item = (Vec<usize>, f64)> + '_ {
        self.entries.iter().map(|(&flat, &val)| {
            let idx = self.unflatten(flat);
            (idx, val)
        })
    }

    /// Scale all entries by a scalar.
    #[must_use]
    pub fn scale(&self, scalar: f64) -> Self {
        if scalar.abs() < 1e-300 {
            return Self::new(self.shape.clone());
        }
        let entries = self
            .entries
            .iter()
            .map(|(&k, &v)| (k, v * scalar))
            .collect();
        Self {
            shape: self.shape.clone(),
            entries,
        }
    }

    /// Add two sparse tensors with the same shape.
    ///
    /// # Errors
    ///
    /// Returns error if shapes don't match.
    pub fn add(&self, other: &Self) -> Result<Self, HisabError> {
        if self.shape != other.shape {
            return Err(HisabError::InvalidInput("shape mismatch".into()));
        }
        let mut entries = self.entries.clone();
        for (&k, &v) in &other.entries {
            let entry = entries.entry(k).or_insert(0.0);
            *entry += v;
            if entry.abs() < 1e-300 {
                entries.remove(&k);
            }
        }
        Ok(Self {
            shape: self.shape.clone(),
            entries,
        })
    }

    /// Convert to a dense flat vector (row-major).
    #[must_use]
    pub fn to_dense(&self) -> Vec<f64> {
        let total = self.total_size();
        let mut data = vec![0.0; total];
        for (&flat, &val) in &self.entries {
            data[flat] = val;
        }
        data
    }

    /// Create from a dense flat vector, dropping near-zero entries.
    pub fn from_dense(shape: Vec<usize>, data: &[f64]) -> Result<Self, HisabError> {
        let expected: usize = shape.iter().product();
        if data.len() != expected {
            return Err(HisabError::InvalidInput(format!(
                "data length {} != shape product {expected}",
                data.len()
            )));
        }
        let entries: BTreeMap<usize, f64> = data
            .iter()
            .enumerate()
            .filter(|(_, v)| v.abs() >= 1e-300)
            .map(|(i, &v)| (i, v))
            .collect();
        Ok(Self { shape, entries })
    }

    // -- internal --

    fn flat_index(&self, indices: &[usize]) -> Result<usize, HisabError> {
        if indices.len() != self.shape.len() {
            return Err(HisabError::InvalidInput(format!(
                "expected {} indices, got {}",
                self.shape.len(),
                indices.len()
            )));
        }
        let mut flat = 0;
        let mut stride = 1;
        for i in (0..self.shape.len()).rev() {
            if indices[i] >= self.shape[i] {
                return Err(HisabError::OutOfRange(format!(
                    "index {} out of range for dimension {} (size {})",
                    indices[i], i, self.shape[i]
                )));
            }
            flat += indices[i] * stride;
            stride *= self.shape[i];
        }
        Ok(flat)
    }

    fn unflatten(&self, mut flat: usize) -> Vec<usize> {
        let rank = self.shape.len();
        let mut indices = vec![0; rank];
        for i in (0..rank).rev() {
            indices[i] = flat % self.shape[i];
            flat /= self.shape[i];
        }
        indices
    }
}

impl std::fmt::Display for SparseTensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SparseTensor({:?}, nnz={}, sparsity={:.1}%)",
            self.shape,
            self.nnz(),
            self.sparsity() * 100.0
        )
    }
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

    #[test]
    fn sparse_empty() {
        let t = SparseTensor::new(vec![4, 4, 4, 4]);
        assert_eq!(t.nnz(), 0);
        assert!(approx(t.sparsity(), 1.0));
        assert!(approx(t.get(&[0, 0, 0, 0]).unwrap(), 0.0));
    }

    #[test]
    fn sparse_set_get() {
        let mut t = SparseTensor::new(vec![3, 3]);
        t.set(&[1, 2], 42.0).unwrap();
        assert!(approx(t.get(&[1, 2]).unwrap(), 42.0));
        assert!(approx(t.get(&[0, 0]).unwrap(), 0.0));
        assert_eq!(t.nnz(), 1);
    }

    #[test]
    fn sparse_set_zero_removes() {
        let mut t = SparseTensor::new(vec![3, 3]);
        t.set(&[1, 2], 42.0).unwrap();
        assert_eq!(t.nnz(), 1);
        t.set(&[1, 2], 0.0).unwrap();
        assert_eq!(t.nnz(), 0);
    }

    #[test]
    fn sparse_add() {
        let mut a = SparseTensor::new(vec![3, 3]);
        a.set(&[0, 1], 1.0).unwrap();
        let mut b = SparseTensor::new(vec![3, 3]);
        b.set(&[0, 1], 2.0).unwrap();
        b.set(&[2, 2], 5.0).unwrap();
        let c = a.add(&b).unwrap();
        assert!(approx(c.get(&[0, 1]).unwrap(), 3.0));
        assert!(approx(c.get(&[2, 2]).unwrap(), 5.0));
    }

    #[test]
    fn sparse_to_from_dense() {
        let data = vec![0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 3.0, 0.0, 0.0];
        let t = SparseTensor::from_dense(vec![3, 3], &data).unwrap();
        assert_eq!(t.nnz(), 3);
        let back = t.to_dense();
        for (a, b) in data.iter().zip(back.iter()) {
            assert!(approx(*a, *b));
        }
    }

    #[test]
    fn sparse_scale() {
        let mut t = SparseTensor::new(vec![2, 2]);
        t.set(&[0, 1], 3.0).unwrap();
        let s = t.scale(2.0);
        assert!(approx(s.get(&[0, 1]).unwrap(), 6.0));
    }

    #[test]
    fn sparse_iter_nonzero() {
        let mut t = SparseTensor::new(vec![3, 3]);
        t.set(&[0, 1], 1.0).unwrap();
        t.set(&[2, 0], 2.0).unwrap();
        let entries: Vec<_> = t.iter_nonzero_indices().collect();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn sparse_riemann_like() {
        // Riemann tensor: rank-4 in 4D = 256 total, but typically ~20 non-zero
        let mut r = SparseTensor::new(vec![4, 4, 4, 4]);
        // Set a few components
        r.set(&[0, 1, 0, 1], 1.0).unwrap();
        r.set(&[0, 1, 1, 0], -1.0).unwrap();
        assert_eq!(r.nnz(), 2);
        assert!(r.sparsity() > 0.99);
    }

    #[test]
    fn sparse_display() {
        let t = SparseTensor::new(vec![4, 4, 4]);
        let s = format!("{t}");
        assert!(s.contains("SparseTensor"));
        assert!(s.contains("nnz=0"));
    }

    #[test]
    fn sparse_shape_mismatch() {
        let a = SparseTensor::new(vec![3, 3]);
        let b = SparseTensor::new(vec![4, 4]);
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn sparse_index_out_of_range() {
        let t = SparseTensor::new(vec![3, 3]);
        assert!(t.get(&[5, 0]).is_err());
    }

    #[test]
    fn sparse_from_dense_size_mismatch() {
        assert!(SparseTensor::from_dense(vec![3, 3], &[1.0; 5]).is_err());
    }
}
