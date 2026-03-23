# Dependency Watch

Tracked dependency version constraints, known incompatibilities, and upgrade paths.

## glam

**Status:** Pinned to `glam` 0.29

**Note:** Ganit re-exports glam types (`Vec2`, `Vec3`, `Mat4`, `Quat`, etc.) as public API. A major glam version bump is a breaking change for ganit consumers. Coordinate glam upgrades with impetus, kiran, and joshua.

**SIMD:** glam 0.29 uses SIMD by default on x86_64 and aarch64. No special feature flags needed — ganit benefits automatically.

## reqwest (AI feature)

**Status:** `reqwest` 0.12 with `json` feature, optional behind `ai`

**Note:** reqwest brings in tokio, hyper, and rustls as transitive deps. These are only compiled when `ai` feature is enabled. No impact on core library size.

## thiserror

**Status:** `thiserror` 2

**Note:** thiserror v2 dropped the `#[error(transparent)]` attribute requirement for `#[from]` variants. Our `DaimonError::Http(#[from] reqwest::Error)` relies on this. Compatible with MSRV 1.89.

## tracing / tracing-subscriber

**Status:** `tracing` 0.1 (always), `tracing-subscriber` 0.3 (optional, `logging` feature)

**Note:** `tracing` 0.1 is a lightweight facade — it adds near-zero overhead when no subscriber is active. The `logging` feature adds `tracing-subscriber` with env-filter for `GANIT_LOG` support.

## criterion (dev-dependency)

**Status:** `criterion` 0.5

**Note:** criterion 0.5 is the last version with the `criterion_group!`/`criterion_main!` macro API. Version 0.8+ changed to a builder pattern. Pinned for stability. Upgrade when convenient — not urgent.
