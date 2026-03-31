//! Tensor algebra for numerical and theoretical physics computation.
//!
//! - [`Tensor`] — N-dimensional dense tensor (existing)
//! - [`IndexedTensor`] — tensor with covariant/contravariant index tracking
//! - [`SymmetricTensor`] — storage-efficient symmetric tensor
//! - [`AntisymmetricTensor`] — storage-efficient antisymmetric tensor
//! - [`SparseTensor`] — sparse tensor for high-rank objects

mod dense;
mod indexed;
mod sparse;
mod symmetric;

pub use dense::Tensor;
pub use indexed::*;
pub use sparse::*;
pub use symmetric::*;
