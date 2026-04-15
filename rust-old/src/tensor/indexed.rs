//! Index-aware tensor algebra for theoretical physics.
//!
//! Provides [`IndexedTensor`] with covariant/contravariant index tracking,
//! Einstein summation convention, tensor contraction, outer product,
//! and index raising/lowering via a metric tensor.

use super::Tensor;
use crate::HisabError;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Index variance
// ---------------------------------------------------------------------------

/// Whether a tensor index is upper (contravariant) or lower (covariant).
///
/// In Einstein notation:
/// - **Contravariant** (upper): `Tᵘ` — transforms inversely to basis vectors
/// - **Covariant** (lower): `Tᵤ` — transforms like basis vectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IndexVariance {
    /// Contravariant (upper) index.
    Contravariant,
    /// Covariant (lower) index.
    Covariant,
}

impl IndexVariance {
    /// Flip the variance: upper ↔ lower.
    #[must_use]
    #[inline]
    pub fn flip(self) -> Self {
        match self {
            Self::Contravariant => Self::Covariant,
            Self::Covariant => Self::Contravariant,
        }
    }
}

/// A labeled tensor index with a name, variance, and dimension.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TensorIndex {
    /// Human-readable label (e.g. "μ", "ν", "α").
    pub label: String,
    /// Upper or lower index.
    pub variance: IndexVariance,
    /// Dimension of this index (e.g. 4 for spacetime).
    pub dim: usize,
}

impl TensorIndex {
    /// Create a new tensor index.
    #[must_use]
    pub fn new(label: impl Into<String>, variance: IndexVariance, dim: usize) -> Self {
        Self {
            label: label.into(),
            variance,
            dim,
        }
    }

    /// Create a contravariant (upper) index.
    #[must_use]
    pub fn upper(label: impl Into<String>, dim: usize) -> Self {
        Self::new(label, IndexVariance::Contravariant, dim)
    }

    /// Create a covariant (lower) index.
    #[must_use]
    pub fn lower(label: impl Into<String>, dim: usize) -> Self {
        Self::new(label, IndexVariance::Covariant, dim)
    }
}

// ---------------------------------------------------------------------------
// IndexedTensor
// ---------------------------------------------------------------------------

/// A tensor with named, typed indices for physics calculations.
///
/// Tracks covariant/contravariant structure and supports Einstein summation,
/// contraction, outer products, and index raising/lowering.
///
/// # Examples
///
/// ```
/// use hisab::tensor::{IndexedTensor, TensorIndex, IndexVariance};
///
/// // Create a rank-2 tensor Tᵘᵥ (one upper, one lower index) in 4D spacetime
/// let indices = vec![
///     TensorIndex::upper("μ", 4),
///     TensorIndex::lower("ν", 4),
/// ];
/// let data = vec![0.0; 16]; // 4×4
/// let t = IndexedTensor::new(indices, data).unwrap();
/// assert_eq!(t.rank(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexedTensor {
    /// The indices with variance and dimension metadata.
    indices: Vec<TensorIndex>,
    /// Underlying dense storage.
    data: Tensor,
}

impl IndexedTensor {
    /// Create a new indexed tensor.
    ///
    /// The shape is derived from the index dimensions.
    ///
    /// # Errors
    ///
    /// Returns error if `data.len()` doesn't match the product of index dimensions.
    pub fn new(indices: Vec<TensorIndex>, data: Vec<f64>) -> Result<Self, HisabError> {
        let shape: Vec<usize> = indices.iter().map(|i| i.dim).collect();
        let tensor = Tensor::new(shape, data)?;
        Ok(Self {
            indices,
            data: tensor,
        })
    }

    /// Create a zero-filled indexed tensor.
    #[must_use]
    pub fn zeros(indices: Vec<TensorIndex>) -> Self {
        let shape: Vec<usize> = indices.iter().map(|i| i.dim).collect();
        Self {
            data: Tensor::zeros(shape),
            indices,
        }
    }

    /// Create the Kronecker delta `δᵘᵥ` (identity tensor).
    ///
    /// # Errors
    ///
    /// Returns error if `dim` is zero.
    pub fn kronecker_delta(
        upper_label: impl Into<String>,
        lower_label: impl Into<String>,
        dim: usize,
    ) -> Result<Self, HisabError> {
        if dim == 0 {
            return Err(HisabError::InvalidInput(
                "dimension must be positive".into(),
            ));
        }
        let indices = vec![
            TensorIndex::upper(upper_label, dim),
            TensorIndex::lower(lower_label, dim),
        ];
        let mut data = vec![0.0; dim * dim];
        for i in 0..dim {
            data[i * dim + i] = 1.0;
        }
        Self::new(indices, data)
    }

    /// Create the Minkowski metric `ηᵤᵥ = diag(+1, −1, −1, −1)`.
    #[must_use]
    pub fn minkowski(label_a: impl Into<String>, label_b: impl Into<String>) -> Self {
        let indices = vec![
            TensorIndex::lower(label_a, 4),
            TensorIndex::lower(label_b, 4),
        ];
        let mut data = vec![0.0; 16];
        data[0] = 1.0; // η₀₀ = +1
        data[5] = -1.0; // η₁₁ = -1
        data[10] = -1.0; // η₂₂ = -1
        data[15] = -1.0; // η₃₃ = -1
        Self {
            indices,
            data: Tensor::from_raw(vec![4, 4], data),
        }
    }

    /// Create the inverse Minkowski metric `ηᵘᵛ = diag(+1, −1, −1, −1)`.
    #[must_use]
    pub fn minkowski_inverse(label_a: impl Into<String>, label_b: impl Into<String>) -> Self {
        let indices = vec![
            TensorIndex::upper(label_a, 4),
            TensorIndex::upper(label_b, 4),
        ];
        let mut data = vec![0.0; 16];
        data[0] = 1.0;
        data[5] = -1.0;
        data[10] = -1.0;
        data[15] = -1.0;
        Self {
            indices,
            data: Tensor::from_raw(vec![4, 4], data),
        }
    }

    /// The Levi-Civita symbol `εᵘᵛᵖˢ` in n dimensions.
    ///
    /// # Errors
    ///
    /// Returns error if `dim` is zero.
    pub fn levi_civita(dim: usize, variance: IndexVariance) -> Result<Self, HisabError> {
        if dim == 0 {
            return Err(HisabError::InvalidInput(
                "dimension must be positive".into(),
            ));
        }
        let indices: Vec<TensorIndex> = (0..dim)
            .map(|i| TensorIndex::new(format!("i{i}"), variance, dim))
            .collect();
        let total: usize = dim.pow(dim as u32);
        let mut data = vec![0.0; total];
        let shape: Vec<usize> = vec![dim; dim];

        // Iterate over all index combinations
        let mut combo = vec![0usize; dim];
        'outer: loop {
            // Check if all indices are distinct
            let mut distinct = true;
            'check: for a in 0..dim {
                for b in (a + 1)..dim {
                    if combo[a] == combo[b] {
                        distinct = false;
                        break 'check;
                    }
                }
            }

            if distinct {
                // Count inversions (parity of permutation)
                let mut inversions = 0;
                for a in 0..dim {
                    for b in (a + 1)..dim {
                        if combo[a] > combo[b] {
                            inversions += 1;
                        }
                    }
                }
                let sign = if inversions % 2 == 0 { 1.0 } else { -1.0 };

                // Compute flat index
                let mut flat = 0;
                let mut stride = 1;
                for i in (0..dim).rev() {
                    flat += combo[i] * stride;
                    stride *= shape[i];
                }
                data[flat] = sign;
            }

            // Increment combo (odometer style)
            let mut carry = true;
            for i in (0..dim).rev() {
                if carry {
                    combo[i] += 1;
                    if combo[i] >= dim {
                        combo[i] = 0;
                    } else {
                        carry = false;
                    }
                }
            }
            if carry {
                break 'outer;
            }
        }

        Ok(Self {
            indices,
            data: Tensor::from_raw(shape, data),
        })
    }

    // -----------------------------------------------------------------------
    // Accessors

    /// Tensor rank (number of indices).
    #[must_use]
    #[inline]
    pub fn rank(&self) -> usize {
        self.indices.len()
    }

    /// The index metadata.
    #[must_use]
    #[inline]
    pub fn indices(&self) -> &[TensorIndex] {
        &self.indices
    }

    /// Shape derived from index dimensions.
    #[must_use]
    #[inline]
    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }

    /// Raw data in row-major order.
    #[must_use]
    #[inline]
    pub fn data(&self) -> &[f64] {
        self.data.data()
    }

    /// Mutable raw data.
    #[inline]
    pub fn data_mut(&mut self) -> &mut [f64] {
        self.data.data_mut()
    }

    /// Get element by multi-dimensional index.
    ///
    /// # Errors
    ///
    /// Returns error if indices are out of range.
    pub fn get(&self, idx: &[usize]) -> Result<f64, HisabError> {
        self.data.get(idx)
    }

    /// Set element by multi-dimensional index.
    ///
    /// # Errors
    ///
    /// Returns error if indices are out of range.
    pub fn set(&mut self, idx: &[usize], value: f64) -> Result<(), HisabError> {
        self.data.set(idx, value)
    }

    /// Reference to the underlying dense tensor.
    #[must_use]
    #[inline]
    pub fn as_tensor(&self) -> &Tensor {
        &self.data
    }

    // -----------------------------------------------------------------------
    // Arithmetic

    /// Element-wise addition (indices must match exactly).
    ///
    /// # Errors
    ///
    /// Returns error if index structure doesn't match.
    pub fn add(&self, other: &Self) -> Result<Self, HisabError> {
        if self.indices != other.indices {
            return Err(HisabError::InvalidInput(
                "index structure mismatch for addition".into(),
            ));
        }
        Ok(Self {
            indices: self.indices.clone(),
            data: self.data.add(&other.data)?,
        })
    }

    /// Element-wise subtraction (indices must match exactly).
    ///
    /// # Errors
    ///
    /// Returns error if index structure doesn't match.
    pub fn sub(&self, other: &Self) -> Result<Self, HisabError> {
        if self.indices != other.indices {
            return Err(HisabError::InvalidInput(
                "index structure mismatch for subtraction".into(),
            ));
        }
        Ok(Self {
            indices: self.indices.clone(),
            data: self.data.sub(&other.data)?,
        })
    }

    /// Scalar multiplication.
    #[must_use]
    pub fn scale(&self, scalar: f64) -> Self {
        Self {
            indices: self.indices.clone(),
            data: self.data.scale(scalar),
        }
    }

    /// Negate all elements.
    #[must_use]
    pub fn neg(&self) -> Self {
        self.scale(-1.0)
    }

    // -----------------------------------------------------------------------
    // Tensor operations

    /// Outer (tensor) product: `C^{a...b...} = A^{a...} ⊗ B^{b...}`.
    ///
    /// Result has rank = `self.rank() + other.rank()`, with indices concatenated.
    #[must_use]
    pub fn outer(&self, other: &Self) -> Self {
        let mut new_indices = self.indices.clone();
        new_indices.extend_from_slice(&other.indices);

        let self_data = self.data.data();
        let other_data = other.data.data();
        let mut data = Vec::with_capacity(self_data.len() * other_data.len());
        for &a in self_data {
            for &b in other_data {
                data.push(a * b);
            }
        }

        let shape: Vec<usize> = new_indices.iter().map(|i| i.dim).collect();
        Self {
            indices: new_indices,
            data: Tensor::from_raw(shape, data),
        }
    }

    /// Contract (trace) over two indices by position.
    ///
    /// The two indices must have the same dimension. One should be upper and
    /// one lower for physical correctness, but this is not enforced.
    ///
    /// # Errors
    ///
    /// Returns error if positions are invalid or dimensions don't match.
    #[allow(clippy::needless_range_loop)]
    pub fn contract(&self, pos_a: usize, pos_b: usize) -> Result<Self, HisabError> {
        let rank = self.rank();
        if pos_a >= rank || pos_b >= rank || pos_a == pos_b {
            return Err(HisabError::InvalidInput(format!(
                "invalid contraction positions {pos_a}, {pos_b} for rank-{rank} tensor"
            )));
        }
        let dim = self.indices[pos_a].dim;
        if self.indices[pos_b].dim != dim {
            return Err(HisabError::InvalidInput(format!(
                "dimension mismatch for contraction: {} vs {}",
                dim, self.indices[pos_b].dim
            )));
        }
        tracing::trace!(
            pos_a,
            pos_b,
            label_a = %self.indices[pos_a].label,
            label_b = %self.indices[pos_b].label,
            "contracting indices"
        );

        // Build new index list (remove pos_a and pos_b)
        let (lo, hi) = if pos_a < pos_b {
            (pos_a, pos_b)
        } else {
            (pos_b, pos_a)
        };
        let mut new_indices: Vec<TensorIndex> = Vec::with_capacity(rank - 2);
        for (i, idx) in self.indices.iter().enumerate() {
            if i != lo && i != hi {
                new_indices.push(idx.clone());
            }
        }

        let new_shape: Vec<usize> = new_indices.iter().map(|i| i.dim).collect();
        let new_total: usize = if new_shape.is_empty() {
            1
        } else {
            new_shape.iter().product()
        };
        let mut new_data = vec![0.0; new_total];

        let old_shape = self.data.shape();
        let old_rank = old_shape.len();

        // Compute strides for old tensor
        let mut strides = vec![1usize; old_rank];
        for i in (0..old_rank - 1).rev() {
            strides[i] = strides[i + 1] * old_shape[i + 1];
        }

        // Iterate over all elements of the new tensor
        let mut new_idx = vec![0usize; new_indices.len()];
        for flat_new in 0..new_total {
            // Convert flat_new to multi-index
            if !new_indices.is_empty() {
                let mut remainder = flat_new;
                for i in (0..new_indices.len()).rev() {
                    new_idx[i] = remainder % new_shape[i];
                    remainder /= new_shape[i];
                }
            }

            // Sum over the contracted index
            let mut sum = 0.0;
            for k in 0..dim {
                // Build old multi-index
                let mut old_flat = 0;
                let mut new_pos = 0;
                for i in 0..old_rank {
                    let val = if i == lo || i == hi {
                        k
                    } else {
                        let v = new_idx[new_pos];
                        new_pos += 1;
                        v
                    };
                    old_flat += val * strides[i];
                }
                sum += self.data.data()[old_flat];
            }
            new_data[flat_new] = sum;
        }

        if new_indices.is_empty() {
            // Scalar result — return rank-0 tensor stored as [1]
            Ok(Self {
                indices: new_indices,
                data: Tensor::new(vec![1], new_data)?,
            })
        } else {
            Ok(Self {
                indices: new_indices,
                data: Tensor::new(new_shape, new_data)?,
            })
        }
    }

    /// Contract with another tensor (Einstein summation on matching label pairs).
    ///
    /// Finds pairs where one tensor has an upper index and the other has a lower
    /// index with the same label. Performs the outer product then contracts all
    /// matching pairs.
    ///
    /// # Errors
    ///
    /// Returns error on dimension mismatches.
    pub fn contract_with(&self, other: &Self) -> Result<Self, HisabError> {
        // Find matching index pairs (one upper, one lower, same label)
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        for (i, idx_a) in self.indices.iter().enumerate() {
            for (j, idx_b) in other.indices.iter().enumerate() {
                if idx_a.label == idx_b.label
                    && idx_a.variance != idx_b.variance
                    && idx_a.dim == idx_b.dim
                {
                    pairs.push((i, self.rank() + j));
                }
            }
        }

        tracing::trace!(matched_pairs = pairs.len(), "contract_with");

        if pairs.is_empty() {
            // No matching indices — return outer product
            return Ok(self.outer(other));
        }

        // Build outer product first
        let mut result = self.outer(other);

        // Contract pairs (adjust positions as earlier contractions reduce rank)
        // Sort pairs in reverse order of position to avoid index shifting issues
        let mut sorted_pairs = pairs;
        sorted_pairs.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

        for (a, b) in sorted_pairs {
            result = result.contract(a.min(b), a.max(b))?;
        }

        Ok(result)
    }

    /// Raise an index using a metric tensor.
    ///
    /// Contracts `metric^{new_label, idx_label} * self_{..., idx_label, ...}`.
    ///
    /// # Errors
    ///
    /// Returns error if the index position is invalid or metric dimensions mismatch.
    #[allow(clippy::needless_range_loop)]
    pub fn raise_index(&self, pos: usize, metric_inverse: &Self) -> Result<Self, HisabError> {
        if pos >= self.rank() {
            return Err(HisabError::InvalidInput(format!(
                "index position {pos} out of range for rank-{} tensor",
                self.rank()
            )));
        }
        if self.indices[pos].variance == IndexVariance::Contravariant {
            return Err(HisabError::InvalidInput(
                "index is already contravariant (upper)".into(),
            ));
        }
        tracing::trace!(pos, label = %self.indices[pos].label, "raising index");

        // The metric inverse should have two upper indices
        if metric_inverse.rank() != 2 {
            return Err(HisabError::InvalidInput("metric must be rank-2".into()));
        }

        let dim = self.indices[pos].dim;

        // Build result by contracting metric with tensor
        // T^...μ... = g^{μν} T_{...ν...}
        let old_shape = self.data.shape();
        let rank = self.rank();

        let mut new_indices = self.indices.clone();
        new_indices[pos].variance = IndexVariance::Contravariant;

        let new_shape: Vec<usize> = new_indices.iter().map(|i| i.dim).collect();
        let total: usize = new_shape.iter().product();
        let mut new_data = vec![0.0; total];

        // Compute strides
        let mut strides = vec![1usize; rank];
        for i in (0..rank - 1).rev() {
            strides[i] = strides[i + 1] * old_shape[i + 1];
        }

        let mut new_strides = vec![1usize; rank];
        for i in (0..rank - 1).rev() {
            new_strides[i] = new_strides[i + 1] * new_shape[i + 1];
        }

        // For each element of the result
        let mut idx = vec![0usize; rank];
        for flat in 0..total {
            // Convert flat to multi-index
            let mut remainder = flat;
            for i in (0..rank).rev() {
                idx[i] = remainder % new_shape[i];
                remainder /= new_shape[i];
            }

            let mu = idx[pos]; // The new (raised) index value

            let mut sum = 0.0;
            for nu in 0..dim {
                // g^{μν}
                let g_val = metric_inverse.data.get(&[mu, nu]).unwrap_or(0.0);
                if g_val.abs() < 1e-300 {
                    continue;
                }

                // T_{...ν...} (replace idx[pos] with nu)
                let mut old_flat = 0;
                for (i, &s) in strides.iter().enumerate() {
                    let v = if i == pos { nu } else { idx[i] };
                    old_flat += v * s;
                }
                sum += g_val * self.data.data()[old_flat];
            }
            new_data[flat] = sum;
        }

        Ok(Self {
            indices: new_indices,
            data: Tensor::new(new_shape, new_data)?,
        })
    }

    /// Lower an index using a metric tensor.
    ///
    /// # Errors
    ///
    /// Returns error if the index position is invalid or metric dimensions mismatch.
    #[allow(clippy::needless_range_loop)]
    pub fn lower_index(&self, pos: usize, metric: &Self) -> Result<Self, HisabError> {
        if pos >= self.rank() {
            return Err(HisabError::InvalidInput(format!(
                "index position {pos} out of range for rank-{} tensor",
                self.rank()
            )));
        }
        if self.indices[pos].variance == IndexVariance::Covariant {
            return Err(HisabError::InvalidInput(
                "index is already covariant (lower)".into(),
            ));
        }
        tracing::trace!(pos, label = %self.indices[pos].label, "lowering index");

        if metric.rank() != 2 {
            return Err(HisabError::InvalidInput("metric must be rank-2".into()));
        }

        let dim = self.indices[pos].dim;
        let old_shape = self.data.shape();
        let rank = self.rank();

        let mut new_indices = self.indices.clone();
        new_indices[pos].variance = IndexVariance::Covariant;

        let new_shape: Vec<usize> = new_indices.iter().map(|i| i.dim).collect();
        let total: usize = new_shape.iter().product();
        let mut new_data = vec![0.0; total];

        let mut strides = vec![1usize; rank];
        for i in (0..rank - 1).rev() {
            strides[i] = strides[i + 1] * old_shape[i + 1];
        }

        let mut new_strides = vec![1usize; rank];
        for i in (0..rank - 1).rev() {
            new_strides[i] = new_strides[i + 1] * new_shape[i + 1];
        }

        let mut idx = vec![0usize; rank];
        for flat in 0..total {
            let mut remainder = flat;
            for i in (0..rank).rev() {
                idx[i] = remainder % new_shape[i];
                remainder /= new_shape[i];
            }

            let mu = idx[pos];

            let mut sum = 0.0;
            for nu in 0..dim {
                let g_val = metric.data.get(&[mu, nu]).unwrap_or(0.0);
                if g_val.abs() < 1e-300 {
                    continue;
                }
                let mut old_flat = 0;
                for (i, &s) in strides.iter().enumerate() {
                    let v = if i == pos { nu } else { idx[i] };
                    old_flat += v * s;
                }
                sum += g_val * self.data.data()[old_flat];
            }
            new_data[flat] = sum;
        }

        Ok(Self {
            indices: new_indices,
            data: Tensor::new(new_shape, new_data)?,
        })
    }

    /// Permute indices to a new order.
    ///
    /// `perm[new_pos] = old_pos`
    ///
    /// # Errors
    ///
    /// Returns error if `perm` is not a valid permutation of `0..rank`.
    #[allow(clippy::needless_range_loop)]
    pub fn permute(&self, perm: &[usize]) -> Result<Self, HisabError> {
        let rank = self.rank();
        if perm.len() != rank {
            return Err(HisabError::InvalidInput(format!(
                "permutation length {} != rank {rank}",
                perm.len()
            )));
        }
        // Verify it's a valid permutation
        let mut seen = vec![false; rank];
        for &p in perm {
            if p >= rank || seen[p] {
                return Err(HisabError::InvalidInput("invalid permutation".into()));
            }
            seen[p] = true;
        }

        let old_shape = self.data.shape();
        let new_indices: Vec<TensorIndex> = perm.iter().map(|&p| self.indices[p].clone()).collect();
        let new_shape: Vec<usize> = new_indices.iter().map(|i| i.dim).collect();
        let total: usize = new_shape.iter().product();

        // Compute old strides
        let mut old_strides = vec![1usize; rank];
        for i in (0..rank - 1).rev() {
            old_strides[i] = old_strides[i + 1] * old_shape[i + 1];
        }

        let mut new_data = vec![0.0; total];
        let mut new_idx = vec![0usize; rank];
        for flat in 0..total {
            let mut remainder = flat;
            for i in (0..rank).rev() {
                new_idx[i] = remainder % new_shape[i];
                remainder /= new_shape[i];
            }

            // Map new indices back to old positions
            let mut old_flat = 0;
            for (new_pos, &old_pos) in perm.iter().enumerate() {
                old_flat += new_idx[new_pos] * old_strides[old_pos];
            }
            new_data[flat] = self.data.data()[old_flat];
        }

        Ok(Self {
            indices: new_indices,
            data: Tensor::new(new_shape, new_data)?,
        })
    }
}

impl std::fmt::Display for IndexedTensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndexedTensor(")?;
        for (i, idx) in self.indices.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            let arrow = match idx.variance {
                IndexVariance::Contravariant => "↑",
                IndexVariance::Covariant => "↓",
            };
            write!(f, "{}{}{}", idx.label, arrow, idx.dim)?;
        }
        write!(f, ")")
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
    fn create_indexed_tensor() {
        let t = IndexedTensor::new(
            vec![TensorIndex::upper("μ", 4), TensorIndex::lower("ν", 4)],
            vec![0.0; 16],
        )
        .unwrap();
        assert_eq!(t.rank(), 2);
        assert_eq!(t.shape(), &[4, 4]);
    }

    #[test]
    fn kronecker_delta_trace() {
        // δᵘᵤ contracted = dimension
        let delta = IndexedTensor::kronecker_delta("μ", "ν", 4).unwrap();
        let trace = delta.contract(0, 1).unwrap();
        assert!(approx(trace.data()[0], 4.0));
    }

    #[test]
    fn minkowski_signature() {
        let eta = IndexedTensor::minkowski("μ", "ν");
        assert!(approx(eta.get(&[0, 0]).unwrap(), 1.0));
        assert!(approx(eta.get(&[1, 1]).unwrap(), -1.0));
        assert!(approx(eta.get(&[2, 2]).unwrap(), -1.0));
        assert!(approx(eta.get(&[3, 3]).unwrap(), -1.0));
        assert!(approx(eta.get(&[0, 1]).unwrap(), 0.0));
    }

    #[test]
    fn minkowski_trace() {
        let eta = IndexedTensor::minkowski("μ", "ν");
        let trace = eta.contract(0, 1).unwrap();
        // η_μμ = 1 + (-1) + (-1) + (-1) = -2
        assert!(approx(trace.data()[0], -2.0));
    }

    #[test]
    fn outer_product_shape() {
        let v =
            IndexedTensor::new(vec![TensorIndex::upper("μ", 4)], vec![1.0, 0.0, 0.0, 0.0]).unwrap();
        let w =
            IndexedTensor::new(vec![TensorIndex::lower("ν", 4)], vec![0.0, 1.0, 0.0, 0.0]).unwrap();
        let outer = v.outer(&w);
        assert_eq!(outer.rank(), 2);
        assert_eq!(outer.shape(), &[4, 4]);
        assert!(approx(outer.get(&[0, 1]).unwrap(), 1.0));
        assert!(approx(outer.get(&[0, 0]).unwrap(), 0.0));
    }

    #[test]
    fn contract_with_einstein() {
        // v^μ η_{μν} = v_ν (lowering)
        let v =
            IndexedTensor::new(vec![TensorIndex::upper("μ", 4)], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let eta = IndexedTensor::minkowski("μ", "ν");
        let result = v.contract_with(&eta).unwrap();
        // v_0 = η_{0μ} v^μ = v^0 = 1
        // v_1 = η_{1μ} v^μ = -v^1 = -2
        // v_2 = η_{2μ} v^μ = -v^2 = -3
        // v_3 = η_{3μ} v^μ = -v^3 = -4
        assert_eq!(result.rank(), 1);
        assert!(approx(result.get(&[0]).unwrap(), 1.0));
        assert!(approx(result.get(&[1]).unwrap(), -2.0));
        assert!(approx(result.get(&[2]).unwrap(), -3.0));
        assert!(approx(result.get(&[3]).unwrap(), -4.0));
    }

    #[test]
    fn raise_lower_roundtrip() {
        let eta = IndexedTensor::minkowski("μ", "ν");
        let eta_inv = IndexedTensor::minkowski_inverse("μ", "ν");

        // Start with v_μ = (1, -2, -3, -4) (covariant)
        let v = IndexedTensor::new(
            vec![TensorIndex::lower("α", 4)],
            vec![1.0, -2.0, -3.0, -4.0],
        )
        .unwrap();

        // Raise: v^α = g^{αβ} v_β
        let v_up = v.raise_index(0, &eta_inv).unwrap();
        assert_eq!(v_up.indices()[0].variance, IndexVariance::Contravariant);
        // v^0 = η^{00} v_0 = 1
        // v^1 = η^{11} v_1 = -(-2) = 2
        assert!(approx(v_up.get(&[0]).unwrap(), 1.0));
        assert!(approx(v_up.get(&[1]).unwrap(), 2.0));
        assert!(approx(v_up.get(&[2]).unwrap(), 3.0));
        assert!(approx(v_up.get(&[3]).unwrap(), 4.0));

        // Lower again: should get back original
        let v_back = v_up.lower_index(0, &eta).unwrap();
        for i in 0..4 {
            assert!(approx(v_back.get(&[i]).unwrap(), v.get(&[i]).unwrap()));
        }
    }

    #[test]
    fn levi_civita_3d() {
        let eps = IndexedTensor::levi_civita(3, IndexVariance::Contravariant).unwrap();
        // ε^{012} = +1
        assert!(approx(eps.get(&[0, 1, 2]).unwrap(), 1.0));
        // ε^{021} = -1
        assert!(approx(eps.get(&[0, 2, 1]).unwrap(), -1.0));
        // ε^{011} = 0
        assert!(approx(eps.get(&[0, 1, 1]).unwrap(), 0.0));
    }

    #[test]
    fn levi_civita_4d_antisymmetric() {
        let eps = IndexedTensor::levi_civita(4, IndexVariance::Covariant).unwrap();
        // ε_{0123} = +1
        assert!(approx(eps.get(&[0, 1, 2, 3]).unwrap(), 1.0));
        // ε_{1023} = -1
        assert!(approx(eps.get(&[1, 0, 2, 3]).unwrap(), -1.0));
        // ε_{1032} = +1
        assert!(approx(eps.get(&[1, 0, 3, 2]).unwrap(), 1.0));
    }

    #[test]
    fn permute_transpose() {
        let t = IndexedTensor::new(
            vec![TensorIndex::upper("μ", 3), TensorIndex::lower("ν", 3)],
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        )
        .unwrap();
        let tp = t.permute(&[1, 0]).unwrap();
        // Transposed: tp[i][j] = t[j][i]
        assert!(approx(tp.get(&[0, 0]).unwrap(), 1.0));
        assert!(approx(tp.get(&[0, 1]).unwrap(), 4.0));
        assert!(approx(tp.get(&[1, 0]).unwrap(), 2.0));
    }

    #[test]
    fn index_variance_flip() {
        assert_eq!(
            IndexVariance::Contravariant.flip(),
            IndexVariance::Covariant
        );
        assert_eq!(
            IndexVariance::Covariant.flip(),
            IndexVariance::Contravariant
        );
    }

    #[test]
    fn tensor_display() {
        let t = IndexedTensor::zeros(vec![TensorIndex::upper("μ", 4), TensorIndex::lower("ν", 4)]);
        let s = format!("{t}");
        assert!(s.contains("μ↑4"));
        assert!(s.contains("ν↓4"));
    }

    #[test]
    fn add_sub() {
        let a = IndexedTensor::new(vec![TensorIndex::upper("μ", 3)], vec![1.0, 2.0, 3.0]).unwrap();
        let b =
            IndexedTensor::new(vec![TensorIndex::upper("μ", 3)], vec![10.0, 20.0, 30.0]).unwrap();
        let c = a.add(&b).unwrap();
        assert!(approx(c.get(&[0]).unwrap(), 11.0));
        let d = a.sub(&b).unwrap();
        assert!(approx(d.get(&[0]).unwrap(), -9.0));
    }

    #[test]
    fn add_index_mismatch() {
        let a = IndexedTensor::new(vec![TensorIndex::upper("μ", 4)], vec![1.0; 4]).unwrap();
        let b = IndexedTensor::new(vec![TensorIndex::lower("μ", 4)], vec![1.0; 4]).unwrap();
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn new_size_mismatch() {
        assert!(IndexedTensor::new(vec![TensorIndex::upper("μ", 4)], vec![1.0; 3]).is_err());
    }

    #[test]
    fn contract_invalid_positions() {
        let t = IndexedTensor::zeros(vec![TensorIndex::upper("μ", 4), TensorIndex::lower("ν", 4)]);
        assert!(t.contract(0, 5).is_err());
    }

    #[test]
    fn contract_same_position() {
        let t = IndexedTensor::zeros(vec![TensorIndex::upper("μ", 4), TensorIndex::lower("ν", 4)]);
        assert!(t.contract(1, 1).is_err());
    }

    #[test]
    fn raise_index_already_contravariant() {
        let t = IndexedTensor::zeros(vec![TensorIndex::upper("μ", 4)]);
        let metric = IndexedTensor::minkowski_inverse("α", "β");
        assert!(t.raise_index(0, &metric).is_err());
    }

    #[test]
    fn lower_index_already_covariant() {
        let t = IndexedTensor::zeros(vec![TensorIndex::lower("μ", 4)]);
        let metric = IndexedTensor::minkowski("α", "β");
        assert!(t.lower_index(0, &metric).is_err());
    }

    #[test]
    fn permute_invalid() {
        let t = IndexedTensor::zeros(vec![TensorIndex::upper("μ", 3), TensorIndex::lower("ν", 3)]);
        assert!(t.permute(&[0, 0]).is_err());
    }

    #[test]
    fn permute_wrong_length() {
        let t = IndexedTensor::zeros(vec![TensorIndex::upper("μ", 3), TensorIndex::lower("ν", 3)]);
        assert!(t.permute(&[0]).is_err());
    }

    #[test]
    fn sub_index_mismatch() {
        let a = IndexedTensor::new(vec![TensorIndex::upper("μ", 4)], vec![1.0; 4]).unwrap();
        let b = IndexedTensor::new(vec![TensorIndex::lower("μ", 4)], vec![1.0; 4]).unwrap();
        assert!(a.sub(&b).is_err());
    }
}
