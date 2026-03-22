//! Unified error types for ganit.

use thiserror::Error;

/// Top-level error type for ganit operations.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum GanitError {
    /// Invalid transform parameters.
    #[error("invalid transform: {0}")]
    InvalidTransform(String),

    /// Matrix is singular and cannot be inverted.
    #[error("singular matrix — cannot invert")]
    SingularMatrix,

    /// Value is outside the acceptable range.
    #[error("value out of range: {0}")]
    OutOfRange(String),

    /// Division by zero.
    #[error("division by zero")]
    DivisionByZero,

    /// Degenerate geometry (e.g. zero-length edge, coincident points).
    #[error("degenerate geometry: {0}")]
    Degenerate(String),

    /// Integration/differentiation interval is invalid.
    #[error("invalid interval: a must be less than b")]
    InvalidInterval,

    /// Step count must be positive.
    #[error("step count must be positive")]
    ZeroSteps,

    /// Iterative solver did not converge.
    #[error("no convergence after {0} iterations")]
    NoConvergence(usize),

    /// Pivot element is zero during elimination.
    #[error("singular matrix — pivot is zero")]
    SingularPivot,

    /// Invalid input to a numerical method.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// Daimon/hoosh AI client error.
    #[cfg(feature = "ai")]
    #[error("daimon: {0}")]
    Daimon(#[from] DaimonError),
}

/// Errors from daimon/hoosh interaction.
#[cfg(feature = "ai")]
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DaimonError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Agent registration failed.
    #[error("registration failed: {0}")]
    Registration(String),

    /// Heartbeat failed.
    #[error("heartbeat failed: {0}")]
    Heartbeat(String),

    /// Hoosh (LLM gateway) query failed.
    #[error("hoosh query failed: {0}")]
    HooshQuery(String),
}

/// Convenience alias for `Result<T, GanitError>`.
pub type Result<T> = std::result::Result<T, GanitError>;
