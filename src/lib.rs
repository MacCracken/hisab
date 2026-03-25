//! # Hisab
//!
//! حساب (Arabic: calculation, reckoning) — Higher mathematics library for the AGNOS ecosystem.
//!
//! Provides typed mathematical operations — linear algebra, geometry, calculus,
//! and numerical methods — built on [glam](https://docs.rs/glam).
//!
//! ## Feature flags
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `transforms` | yes | 2D/3D transforms, projections, slerp, lerp, glam re-exports |
//! | `geo` | yes | Primitives, intersections, BVH, k-d tree, quadtree, octree, spatial hash, GJK/EPA |
//! | `calc` | yes | Differentiation, integration, Bezier 2D/3D, splines, easing, Gauss-Legendre |
//! | `num` | yes | Root finding, LU/Cholesky/QR/SVD, FFT, optimization, ODE solvers |
//! | `autodiff` | no | Forward-mode automatic differentiation (dual numbers) |
//! | `interval` | no | Interval arithmetic for verified numerics |
//! | `symbolic` | no | Symbolic expression tree with differentiation and simplification |
//! | `tensor` | no | N-dimensional dense tensor type |
//! | `parallel` | no | Rayon-powered parallel batch operations |
//! | `ai` | no | Daimon/hoosh AI client (requires network deps) |
//! | `logging` | no | Structured logging via tracing-subscriber |
//! | `full` | — | Enables all features |

pub mod error;
pub use error::HisabError;

#[cfg(feature = "ai")]
pub use error::DaimonError;

/// Convenience alias for `Result<T, HisabError>`.
pub type Result<T> = std::result::Result<T, HisabError>;

/// Default tolerance for f32 comparisons.
pub const EPSILON_F32: f32 = 1e-7;

/// Default tolerance for f64 comparisons.
pub const EPSILON_F64: f64 = 1e-12;

#[cfg(feature = "transforms")]
pub mod transforms;

#[cfg(feature = "geo")]
pub mod geo;

#[cfg(feature = "calc")]
pub mod calc;

#[cfg(feature = "num")]
pub mod num;

#[cfg(feature = "autodiff")]
pub mod autodiff;

#[cfg(feature = "interval")]
pub mod interval;

#[cfg(feature = "symbolic")]
pub mod symbolic;

#[cfg(feature = "tensor")]
pub mod tensor;

#[cfg(feature = "parallel")]
pub mod parallel;

#[cfg(feature = "ai")]
pub mod ai;

#[cfg(feature = "logging")]
pub mod logging;

// ---------------------------------------------------------------------------
// Convenience re-exports
// ---------------------------------------------------------------------------

#[cfg(feature = "transforms")]
pub use transforms::{EulerOrder, Transform2D, Transform3D};

// f32 types
#[cfg(feature = "transforms")]
pub use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

// f64 types
#[cfg(feature = "transforms")]
pub use glam::{DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};

#[cfg(feature = "geo")]
pub use geo::{
    Aabb, Bvh, Capsule, ContactConstraint, ConvexDecomposition, ConvexHull3D, ConvexPolygon,
    ConvexSupport, ConvexSupport3D, Frustum, KdTree, Line, Obb, Octree, Penetration3D, Plane,
    Quadtree, Ray, Rect, Segment, SpatialHash, Sphere, TriMesh, Triangle,
};

#[cfg(feature = "num")]
pub use num::{Complex, CsrMatrix, EigenDecomposition, OptResult, Pcg32, Svd};

#[cfg(feature = "autodiff")]
pub use autodiff::{Dual, Tape, Var};

#[cfg(feature = "interval")]
pub use interval::Interval;

#[cfg(feature = "symbolic")]
pub use symbolic::Expr;

#[cfg(feature = "tensor")]
pub use tensor::Tensor;

#[cfg(feature = "ai")]
pub use ai::DaimonClient;

// ---------------------------------------------------------------------------
// Compile-time Send + Sync assertions
// ---------------------------------------------------------------------------

#[cfg(test)]
mod assert_traits {
    fn _assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn public_types_are_send_sync() {
        #[cfg(feature = "transforms")]
        {
            _assert_send_sync::<super::Transform2D>();
            _assert_send_sync::<super::Transform3D>();
        }

        #[cfg(feature = "geo")]
        {
            _assert_send_sync::<super::Ray>();
            _assert_send_sync::<super::Aabb>();
            _assert_send_sync::<super::Sphere>();
            _assert_send_sync::<super::Plane>();
            _assert_send_sync::<super::Triangle>();
            _assert_send_sync::<super::Line>();
            _assert_send_sync::<super::Segment>();
            _assert_send_sync::<super::Frustum>();
            _assert_send_sync::<super::Obb>();
            _assert_send_sync::<super::Capsule>();
            _assert_send_sync::<super::Penetration3D>();
        }

        #[cfg(feature = "num")]
        {
            _assert_send_sync::<super::Complex>();
        }
    }
}
