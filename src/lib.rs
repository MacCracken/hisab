//! # Ganit
//!
//! गणित (Sanskrit: mathematics) — Higher mathematics library for the AGNOS ecosystem.
//!
//! Provides typed mathematical operations — linear algebra, geometry, calculus,
//! and numerical methods — built on [glam](https://docs.rs/glam).
//!
//! ## Feature flags
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `transforms` | yes | 2D/3D transforms, projections, lerp, glam re-exports |
//! | `geo` | yes | Geometric primitives and ray intersection tests |
//! | `calc` | yes | Differentiation, integration, Bezier curves |
//! | `num` | yes | Newton-Raphson, bisection, Gaussian elimination |
//! | `ai` | no | Daimon/hoosh AI client (requires network deps) |
//! | `logging` | no | Structured logging via tracing-subscriber |
//! | `full` | — | Enables all features |

pub mod error;
pub use error::GanitError;

#[cfg(feature = "ai")]
pub use error::DaimonError;

/// Convenience alias for `Result<T, GanitError>`.
pub type Result<T> = std::result::Result<T, GanitError>;

#[cfg(feature = "transforms")]
pub mod transforms;

#[cfg(feature = "geo")]
pub mod geo;

#[cfg(feature = "calc")]
pub mod calc;

#[cfg(feature = "num")]
pub mod num;

#[cfg(feature = "ai")]
pub mod ai;

#[cfg(feature = "logging")]
pub mod logging;

// ---------------------------------------------------------------------------
// Convenience re-exports
// ---------------------------------------------------------------------------

#[cfg(feature = "transforms")]
pub use transforms::{Transform2D, Transform3D};

#[cfg(feature = "transforms")]
pub use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

#[cfg(feature = "geo")]
pub use geo::{Aabb, Plane, Ray, Sphere};

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
        }
    }
}
