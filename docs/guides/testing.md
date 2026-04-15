# Testing Guide

## Running Tests

```bash
# All test suites
cyrius test tests/hisab.tcyr        # 116 smoke/integration tests
cyrius test tests/foundation.tcyr   # 307 exhaustive foundation type tests
cyrius test tests/modules.tcyr      # 249 per-module tests
cyrius test tests/edge_cases.tcyr   # 149 edge case + boundary tests

# Benchmarks (22 operations)
cyrius bench tests/hisab.bcyr

# Fuzz self-test
cyrius build tests/hisab.fcyr build/hisab_fuzz && build/hisab_fuzz
```

## Test Suites

| Suite | Assertions | Covers |
|-------|-----------|--------|
| `foundation.tcyr` | 307 | Vec2/3/4, Quat, Mat4 — construction, arithmetic, products, norms, interpolation, rotation, inverse, determinant, SRT, projections |
| `modules.tcyr` | 249 | All 10+ modules — geo, calc, num, complex, Lie, diffgeo, symbolic, autodiff, interval, tensor |
| `hisab.tcyr` | 116 | Cross-module integration — ODE, optimization, sparse, ray-sphere, Newton, Euler identity |
| `edge_cases.tcyr` | 149 | Zero-length normalize, degenerate cross, singular inverse, parallel ray, division by zero, empty sums, undefined variables |
| **Total** | **821** | |

## Benchmarks (22 operations)

| Category | Benchmarks |
|----------|-----------|
| Vec/Quat/Mat | vec3_add, vec3_cross, vec3_normalize, quat_mul, quat_slerp, quat_rotate_vec3, m4_mul, m4_inverse, m4_transform_point, t3d_compose |
| Geometry | ray_sphere, ray_aabb, ray_triangle |
| Color | srgb_to_linear, tonemap_reinhard |
| Calculus | calc_derivative, calc_integral_simpson |
| Numerical | num_gcd, num_is_prime, cx_mul |
| Other | ease_in_out, perlin_2d |

## Fuzz Targets (5)

| Target | Input bytes | Invariant checked |
|--------|------------|-------------------|
| vec3 ops | 48 | normalize length finite |
| quat rotation | 56 | rotation preserves length |
| ray intersections | 72 | no crash |
| num_gcd | 16 | gcd divides both inputs |
| m4_inverse | 128 | M * M^-1 ≈ I when det > 0 |

## Testing Patterns

### Approximate equality

```cyrius
# Check f64 values within tolerance
var diff = f64_abs(f64_sub(actual, expected));
assert(f64_lt(diff, tolerance) == 1, "message");

# Or multiply by 1000 and round for integer comparison
var result_1000 = f64_to(f64_round(f64_mul(value, f64_from(1000))));
assert_eq(result_1000, 1414, "sqrt2 * 1000");
```

### Function pointer tests

```cyrius
# Define helpers BEFORE alloc_init() to avoid compiler issues
fn _test_x2(x) { return f64_mul(x, x); }
fn _test_df(x) { return f64_mul(F64_TWO, x); }

alloc_init();

var out = alloc(8);
num_newton(&_test_x2, &_test_df, F64_ONE, EPSILON_F64, 100, out);
```

### Mathematical property tests

```cyrius
# Euler's identity: |e^(iπ) + 1| ≈ 0
var eipi = cx_exp(cx_new(0, F64_PI));
var euler = cx_add(eipi, cx_one());
assert(f64_lt(cx_abs(euler), f64_from(1)) == 1, "euler identity");

# Quaternion rotation preserves length
var len_before = hvec3_length(v);
var rotated = hquat_rotate_vec3(q, v);
var len_after = hvec3_length(rotated);
# |len_before - len_after| < epsilon
```

## Performance Comparison

See [docs/benchmarks-rust-v-cyrius.md](../benchmarks-rust-v-cyrius.md) for Rust vs Cyrius benchmark comparison with 85 Rust + 22 Cyrius data points.
