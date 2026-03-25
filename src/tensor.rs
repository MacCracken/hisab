//! N-dimensional tensor type for numerical computation.
//!
//! Provides a dense [`Tensor`] with arbitrary shape, supporting element access,
//! arithmetic, reshape, transpose (2D), and matrix multiplication.

use crate::HisabError;

/// An N-dimensional dense tensor.
///
/// # Examples
///
/// ```
/// use hisab::tensor::Tensor;
///
/// let a = Tensor::new(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
/// let b = Tensor::ones(vec![2, 2]);
/// let c = a.add(&b).unwrap();
/// assert!((c.get(&[0, 0]).unwrap() - 2.0).abs() < 1e-12);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Tensor {
    /// Shape of the tensor (e.g. `[2, 3]` for a 2×3 matrix).
    shape: Vec<usize>,
    /// Flat data in row-major order.
    data: Vec<f64>,
}

impl Tensor {
    /// Create a tensor from shape and data.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if `data.len()` doesn't match the product of `shape`.
    pub fn new(shape: Vec<usize>, data: Vec<f64>) -> Result<Self, HisabError> {
        let expected: usize = shape.iter().product();
        if data.len() != expected {
            return Err(HisabError::InvalidInput(format!(
                "data length {} != shape product {expected}",
                data.len()
            )));
        }
        Ok(Self { shape, data })
    }

    /// Create a tensor of zeros.
    #[must_use]
    pub fn zeros(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        Self {
            shape,
            data: vec![0.0; size],
        }
    }

    /// Create a tensor of ones.
    #[must_use]
    pub fn ones(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        Self {
            shape,
            data: vec![1.0; size],
        }
    }

    /// Create a tensor from a flat vector with the given shape.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if lengths don't match.
    pub fn from_vec(shape: Vec<usize>, data: Vec<f64>) -> Result<Self, HisabError> {
        Self::new(shape, data)
    }

    /// The shape of the tensor.
    #[must_use]
    #[inline]
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    /// Number of dimensions.
    #[must_use]
    #[inline]
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// Total number of elements.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Whether the tensor is empty.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Raw data slice.
    #[must_use]
    #[inline]
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    /// Mutable data slice.
    #[inline]
    pub fn data_mut(&mut self) -> &mut [f64] {
        &mut self.data
    }

    /// Compute flat index from multi-dimensional indices.
    fn flat_index(&self, indices: &[usize]) -> Result<usize, HisabError> {
        if indices.len() != self.shape.len() {
            return Err(HisabError::InvalidInput(format!(
                "expected {} indices, got {}",
                self.shape.len(),
                indices.len()
            )));
        }
        let mut idx = 0;
        let mut stride = 1;
        for i in (0..self.shape.len()).rev() {
            if indices[i] >= self.shape[i] {
                return Err(HisabError::OutOfRange(format!(
                    "index {} out of range for dimension {} (size {})",
                    indices[i], i, self.shape[i]
                )));
            }
            idx += indices[i] * stride;
            stride *= self.shape[i];
        }
        Ok(idx)
    }

    /// Get an element by multi-dimensional index.
    ///
    /// # Errors
    ///
    /// Returns error if indices are out of range.
    pub fn get(&self, indices: &[usize]) -> Result<f64, HisabError> {
        let idx = self.flat_index(indices)?;
        Ok(self.data[idx])
    }

    /// Set an element by multi-dimensional index.
    ///
    /// # Errors
    ///
    /// Returns error if indices are out of range.
    pub fn set(&mut self, indices: &[usize], value: f64) -> Result<(), HisabError> {
        let idx = self.flat_index(indices)?;
        self.data[idx] = value;
        Ok(())
    }

    /// Reshape the tensor (must preserve total element count).
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if the new shape has a different total size.
    pub fn reshape(&self, new_shape: Vec<usize>) -> Result<Self, HisabError> {
        let new_size: usize = new_shape.iter().product();
        if new_size != self.data.len() {
            return Err(HisabError::InvalidInput(format!(
                "cannot reshape: {} elements into shape with {} elements",
                self.data.len(),
                new_size
            )));
        }
        Ok(Self {
            shape: new_shape,
            data: self.data.clone(),
        })
    }

    /// Element-wise addition.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if shapes don't match.
    pub fn add(&self, other: &Tensor) -> Result<Self, HisabError> {
        if self.shape != other.shape {
            return Err(HisabError::InvalidInput("shape mismatch".into()));
        }
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Ok(Self {
            shape: self.shape.clone(),
            data,
        })
    }

    /// Element-wise subtraction.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if shapes don't match.
    pub fn sub(&self, other: &Tensor) -> Result<Self, HisabError> {
        if self.shape != other.shape {
            return Err(HisabError::InvalidInput("shape mismatch".into()));
        }
        let data: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        Ok(Self {
            shape: self.shape.clone(),
            data,
        })
    }

    /// Scalar multiplication.
    #[must_use]
    pub fn scale(&self, scalar: f64) -> Self {
        Self {
            shape: self.shape.clone(),
            data: self.data.iter().map(|x| x * scalar).collect(),
        }
    }

    /// Matrix multiplication for 2D tensors.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if tensors are not 2D or inner dimensions don't match.
    pub fn matmul(&self, other: &Tensor) -> Result<Self, HisabError> {
        if self.ndim() != 2 || other.ndim() != 2 {
            return Err(HisabError::InvalidInput(
                "matmul requires 2D tensors".into(),
            ));
        }
        let m = self.shape[0];
        let k = self.shape[1];
        let n = other.shape[1];
        if k != other.shape[0] {
            return Err(HisabError::InvalidInput(format!(
                "matmul: inner dimensions {k} != {}",
                other.shape[0]
            )));
        }

        let mut data = vec![0.0; m * n];
        for i in 0..m {
            for p in 0..k {
                let a_val = self.data[i * k + p];
                for j in 0..n {
                    data[i * n + j] += a_val * other.data[p * n + j];
                }
            }
        }

        Ok(Self {
            shape: vec![m, n],
            data,
        })
    }

    /// Transpose a 2D tensor.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if the tensor is not 2D.
    pub fn transpose(&self) -> Result<Self, HisabError> {
        if self.ndim() != 2 {
            return Err(HisabError::InvalidInput(
                "transpose requires 2D tensor".into(),
            ));
        }
        let m = self.shape[0];
        let n = self.shape[1];
        let mut data = vec![0.0; m * n];
        for i in 0..m {
            for j in 0..n {
                data[j * m + i] = self.data[i * n + j];
            }
        }
        Ok(Self {
            shape: vec![n, m],
            data,
        })
    }
}

impl std::fmt::Display for Tensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tensor({:?}, [{} elements])",
            self.shape,
            self.data.len()
        )
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
    fn tensor_new() {
        let t = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        assert_eq!(t.shape(), &[2, 3]);
        assert_eq!(t.ndim(), 2);
        assert_eq!(t.len(), 6);
    }

    #[test]
    fn tensor_new_mismatch() {
        assert!(Tensor::new(vec![2, 3], vec![1.0, 2.0]).is_err());
    }

    #[test]
    fn tensor_zeros_ones() {
        let z = Tensor::zeros(vec![2, 2]);
        assert!(z.data().iter().all(|&x| approx(x, 0.0)));
        let o = Tensor::ones(vec![3]);
        assert!(o.data().iter().all(|&x| approx(x, 1.0)));
    }

    #[test]
    fn tensor_get_set() {
        let mut t = Tensor::zeros(vec![2, 3]);
        t.set(&[1, 2], 42.0).unwrap();
        assert!(approx(t.get(&[1, 2]).unwrap(), 42.0));
        assert!(approx(t.get(&[0, 0]).unwrap(), 0.0));
    }

    #[test]
    fn tensor_get_out_of_range() {
        let t = Tensor::zeros(vec![2, 3]);
        assert!(t.get(&[5, 0]).is_err());
    }

    #[test]
    fn tensor_reshape() {
        let t = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let r = t.reshape(vec![3, 2]).unwrap();
        assert_eq!(r.shape(), &[3, 2]);
        assert_eq!(r.data(), t.data());
    }

    #[test]
    fn tensor_reshape_mismatch() {
        let t = Tensor::zeros(vec![2, 3]);
        assert!(t.reshape(vec![2, 2]).is_err());
    }

    #[test]
    fn tensor_add() {
        let a = Tensor::ones(vec![2, 2]);
        let b = Tensor::ones(vec![2, 2]);
        let c = a.add(&b).unwrap();
        assert!(c.data().iter().all(|&x| approx(x, 2.0)));
    }

    #[test]
    fn tensor_add_mismatch() {
        let a = Tensor::zeros(vec![2, 3]);
        let b = Tensor::zeros(vec![3, 2]);
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn tensor_sub() {
        let a = Tensor::new(vec![3], vec![5.0, 3.0, 1.0]).unwrap();
        let b = Tensor::new(vec![3], vec![1.0, 2.0, 3.0]).unwrap();
        let c = a.sub(&b).unwrap();
        assert!(approx(c.data()[0], 4.0));
        assert!(approx(c.data()[1], 1.0));
        assert!(approx(c.data()[2], -2.0));
    }

    #[test]
    fn tensor_scale() {
        let t = Tensor::new(vec![2], vec![3.0, 4.0]).unwrap();
        let r = t.scale(2.0);
        assert!(approx(r.data()[0], 6.0));
        assert!(approx(r.data()[1], 8.0));
    }

    #[test]
    fn tensor_matmul() {
        // [[1,2],[3,4]] * [[5,6],[7,8]] = [[19,22],[43,50]]
        let a = Tensor::new(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = Tensor::new(vec![2, 2], vec![5.0, 6.0, 7.0, 8.0]).unwrap();
        let c = a.matmul(&b).unwrap();
        assert!(approx(c.get(&[0, 0]).unwrap(), 19.0));
        assert!(approx(c.get(&[0, 1]).unwrap(), 22.0));
        assert!(approx(c.get(&[1, 0]).unwrap(), 43.0));
        assert!(approx(c.get(&[1, 1]).unwrap(), 50.0));
    }

    #[test]
    fn tensor_matmul_non_square() {
        // [2x3] * [3x1] = [2x1]
        let a = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let b = Tensor::new(vec![3, 1], vec![1.0, 1.0, 1.0]).unwrap();
        let c = a.matmul(&b).unwrap();
        assert_eq!(c.shape(), &[2, 1]);
        assert!(approx(c.data()[0], 6.0));
        assert!(approx(c.data()[1], 15.0));
    }

    #[test]
    fn tensor_matmul_mismatch() {
        let a = Tensor::zeros(vec![2, 3]);
        let b = Tensor::zeros(vec![2, 2]);
        assert!(a.matmul(&b).is_err());
    }

    #[test]
    fn tensor_transpose() {
        let t = Tensor::new(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let r = t.transpose().unwrap();
        assert_eq!(r.shape(), &[3, 2]);
        assert!(approx(r.get(&[0, 0]).unwrap(), 1.0));
        assert!(approx(r.get(&[1, 0]).unwrap(), 2.0));
        assert!(approx(r.get(&[0, 1]).unwrap(), 4.0));
    }

    #[test]
    fn tensor_transpose_not_2d() {
        let t = Tensor::zeros(vec![2, 3, 4]);
        assert!(t.transpose().is_err());
    }

    #[test]
    fn tensor_display() {
        let t = Tensor::zeros(vec![2, 3]);
        let s = format!("{t}");
        assert!(s.contains("[2, 3]"));
    }

    #[test]
    fn tensor_is_empty() {
        let t = Tensor::zeros(vec![0]);
        assert!(t.is_empty());
    }
}
