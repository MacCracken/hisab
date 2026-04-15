//! Structured logging and tracing initialisation.
//!
//! Provides [`init`] to set up a `tracing-subscriber` with env-based filtering
//! via the `HISAB_LOG` environment variable. Requires the `logging` feature.
//!
//! # Log levels
//!
//! Set `HISAB_LOG` to control verbosity:
//!
//! | Value | Shows |
//! |-------|-------|
//! | `error` | Errors only |
//! | `warn` | Warnings and above |
//! | `info` | Lifecycle events |
//! | `debug` | Detailed operations |
//! | `trace` | Per-call tracing (high volume) |
//!
//! Supports per-module filtering: `HISAB_LOG=hisab::num=debug,hisab::geo=trace`
//!
//! # Example
//!
//! ```rust,no_run
//! // At the start of your application:
//! hisab::logging::init();
//!
//! // Or with a specific default level:
//! hisab::logging::init_with_level("debug");
//! ```

/// Initialise hisab logging with the `HISAB_LOG` environment variable.
///
/// Falls back to `info` if `HISAB_LOG` is not set.
/// Safe to call multiple times — subsequent calls are no-ops.
pub fn init() {
    init_with_level("info");
}

/// Initialise hisab logging with a specific default level.
///
/// The `HISAB_LOG` environment variable overrides `default_level` if set.
/// Safe to call multiple times — subsequent calls are no-ops.
pub fn init_with_level(default_level: &str) {
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    let filter =
        EnvFilter::try_from_env("HISAB_LOG").unwrap_or_else(|_| EnvFilter::new(default_level));

    let _ = tracing_subscriber::registry()
        .with(fmt::layer().with_target(true).with_thread_ids(true))
        .with(filter)
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_does_not_panic() {
        init();
        init();
    }

    #[test]
    fn init_with_level_does_not_panic() {
        init_with_level("trace");
        init_with_level("error");
    }
}
