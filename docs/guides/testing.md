# Testing Guide

## Running Tests

```bash
# Default features (transforms, geo, calc, num)
cargo test

# All features including AI client
cargo test --all-features

# No default features (only error module)
cargo test --no-default-features

# Specific module
cargo test --all-features transforms::
cargo test --all-features geo::
cargo test --all-features calc::
cargo test --all-features num::
```

## Test Categories

| Category | Count | Location |
|----------|-------|----------|
| Unit tests (transforms) | 44 | `src/transforms.rs` |
| Unit tests (geo) | 153 | `src/geo.rs` |
| Unit tests (calc) | 60 | `src/calc.rs` |
| Unit tests (num) | 82 | `src/num.rs` |
| Unit tests (ai) | 6 | `src/ai.rs` |
| Unit tests (logging) | 2 | `src/logging.rs` |
| Send+Sync assertions | 1 | `src/lib.rs` |
| Integration tests | 10 | `tests/integration.rs` |
| Doc tests | 1 | `src/logging.rs` |
| **Total** | **360** | |

## Coverage

Target: 80%+ line coverage.

```bash
# Generate coverage report (requires cargo-llvm-cov)
make coverage

# Or directly
cargo llvm-cov --all-features --html --output-dir coverage/
```

Coverage configuration is in `codecov.yml` (80% project target, 75% patch target).

## Benchmarks

```bash
# Run benchmarks with auto-appending CSV history + markdown report
make bench

# Or directly
./scripts/bench-history.sh

# Just criterion (no history tracking)
cargo bench --bench benchmarks
```

Results:
- `bench-history.csv` — all runs with timestamp, commit, branch, benchmark name, nanoseconds
- `benchmarks.md` — 3-point trend table (baseline → optimized → current)

### Benchmark groups (82 total)

| Group | Count | What it measures |
|-------|-------|-----------------|
| transforms | 8 | to_matrix, apply_point, projections, lerp |
| geo | 7 | ray-sphere/plane/aabb, contains, merge |
| calc | 7 | derivative, integration, bezier |
| num | 4 | newton, bisection, gaussian |
| batch | 4 | 100-item ray/aabb/transform, 10k simpson |
| v02 | 15 | triangle, frustum, overlaps, closest-point, slerp |
| v03 | 9 | 3D bezier, splines, GL quadrature, easing |
| v04a | 6 | LU, cholesky, QR, least squares |
| v04b | 4 | eigenvalue, FFT 64/1024, FFT+IFFT |
| v04c | 3 | RK4 100/1000 steps, oscillator |
| v05a | 6 | BVH build/query, k-d tree build/nearest/radius |
| v05b | 6 | quadtree/octree insert/query, spatial hash |
| v05c | 4 | convex hull, GJK intersect/miss, GJK+EPA |

## Testing Patterns

### Approximate equality

All modules define local helpers:

```rust
const EPSILON: f32 = 1e-4;  // or f64 = 1e-6

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

fn vec3_approx_eq(a: Vec3, b: Vec3) -> bool {
    approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
}
```

### Serde roundtrip

All serializable types have roundtrip tests:

```rust
let json = serde_json::to_string(&value).unwrap();
let restored = serde_json::from_str(&json).unwrap();
assert_eq!(value, restored);
```

### Optimization verification

When optimizing a function, add a test that compares the optimized path against the reference (matrix) path:

```rust
#[test]
fn transform3d_apply_matches_matrix() {
    let t = Transform3D::new(pos, rot, scale);
    let via_apply = t.apply_to_point(point);
    let via_matrix = t.to_matrix() * Vec4::new(point.x, point.y, point.z, 1.0);
    assert!(vec3_approx_eq(via_apply, Vec3::new(via_matrix.x, ...)));
}
```

### Mathematical property tests

Verify known mathematical properties:

```rust
// FFT linearity: FFT(ax+by) = a*FFT(x) + b*FFT(y)
// Parseval's theorem: N * Σ|x[n]|² = Σ|X[k]|²
// Smootherstep symmetry: f(t) + f(1-t) = 1
// Inverse roundtrip: inv(T) * T * p ≈ p
```

## Local CI

```bash
make check   # fmt + clippy + test + audit
```

This matches what CI runs, minus platform matrix and coverage upload.
