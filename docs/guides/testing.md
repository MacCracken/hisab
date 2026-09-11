# Testing Guide

## Running Tests

```bash
# All test suites
cyrius test tests/hisab.tcyr        # 536 smoke/integration tests
cyrius test tests/foundation.tcyr   # 413 exhaustive foundation type tests
cyrius test tests/modules.tcyr      # 2073 per-module tests
cyrius test tests/edge_cases.tcyr   # 233 edge case + boundary tests
cyrius test tests/abuse.tcyr        # 784 hostile-input tests

# Benchmarks (74 operations)
cyrius bench tests/hisab.bcyr

# Fuzz self-test — `cyrius fuzz` walks tests/ as of cyrius 6.5.6; before that it
# looked only in fuzz/ (a directory this repo does not have) and silently found
# nothing while still exiting 0. Use the verb on >= 6.5.6; the manual build is
# the fallback on older toolchains.
cyrius fuzz
cyrius build tests/hisab.fcyr build/hisab_fuzz && build/hisab_fuzz
```

## Test Suites

| Suite | Assertions | Covers |
|-------|-----------|--------|
| `foundation.tcyr` | 413 | Vec2/3/4, Quat, Mat4 — construction, arithmetic, products, norms, interpolation, rotation, inverse, determinant, SRT, projections |
| `modules.tcyr` | 2073 | Per-module — geo, calc, num, complex, Lie, diffgeo, symbolic, autodiff (forward-mode duals AND the 2.11.0 reverse-mode tape), interval, tensor, einsum, mat3, noise, color, arena, spatial (BVH, kd-tree, spatial hash), differentiable geometry (`geo_diff` — ray/surface jets for all six primitives; plane/sphere/triangle in 2.10.0, aabb/obb/capsule in 2.10.1, the OBB rotation partial in 2.10.2) and collision (convex hull, triangulation, Delaunay, half-edge, GJK/EPA, MPR, time-of-impact, island detection, sequential-impulse) |
| `hisab.tcyr` | 536 | Cross-module integration — ODE, optimization, sparse, PGS/LCP, ray-sphere, Newton, Euler identity, CGA (contraction/dual/projection), mat_new_guarded, diffgeo (sectional/Weyl/transport/Jacobi/forms), decomposition (SVD, QR, eigen), Krylov (GMRES) |
| `edge_cases.tcyr` | 233 | Degenerate inputs (zero-length normalize, singular inverse, parallel ray, division by zero, undefined variables) plus pinned invariants (bit-math/overflow/determinism, allocation-overflow guards — including the **upstream stdlib `mat_new`** CWE-190 contract, added 2.6.11) |
| `abuse.tcyr` | 784 | Hostile input, added 2.9.0 — negative indices and counts, zero/one/overflow-prone dimensions, non-conformable operands, the designed-0 return of every capped constructor, degenerate geometry (zero extents/radii, coincident points, NaN/±Inf coordinates), bounded-work guarantees, and **canary checks** (a guard block allocated immediately after each out-buffer, asserted untouched — the only way a write-past-the-end shows up as a failure rather than as luck). Surfaced 11 real defects on public entry points, held in a known-defect register rather than deleted |
| **Total** | **4039** | |

⛔ **2.21.0 — THREE OF THOSE FIVE ROWS HAD BEEN STALE FOR RELEASES AND THE TABLE
COULD NOT ANNOUNCE IT.** They summed to **3574** against a published total of
**3989**: `foundation` read 351 for 413, `modules` 1798 for 2072, `hisab` 416 for
497. The total was being updated on each release and the rows were not, so the
document was internally inconsistent and externally wrong at the same time.
⭐ CI now asserts **each row against the live suite AND that the rows sum to the
total**, so neither half can drift alone; verified fail-closed against a stale
row, a total that does not sum, and a deleted row.

## Benchmarks (74 operations)

| Category | Benchmarks |
|----------|-----------|
| Vec/Quat/Mat | vec3_add, vec3_cross, vec3_normalize, quat_mul, quat_slerp, quat_rotate_vec3, m4_mul, m4_inverse, m4_transform_point, t3d_compose |
| SIMD batches | vec3_dot_x64, vec4_dot_x64, m4_mul_x16, m4_transform_x64 (amplified — single-op timings sit below the harness floor) |
| Geometry | ray_sphere, ray_aabb, ray_aabb_diag, ray_obb, ray_triangle, ray_capsule, ray_capsule_diag |
| Scale covariance (not a benchmark — a test shape) | ⚠ 2.10.2's four threshold repairs are all asserted by *scale covariance*: the same scene in different units must give the same answer. A single-scale fixture cannot see a bad threshold however many samples it takes, and every other geometry fixture in the suite is single-scale |
| Differentiable geometry | jet_sphere, jet_plane, jet_triangle, jet_aabb, jet_obb, jet_capsule — each paired with a primal row **at the same fixture**, so the ratio is readable straight off the CSV. ⚠ 2.10.1 added `ray_aabb_diag`, `ray_obb` and `ray_capsule_diag` precisely for that: the axis-aligned `ray_aabb` row skips two of three slabs, and `ray_capsule` lands on a cap while `jet_capsule` lands on the barrel — dividing across fixtures would have reported the capsule jet at 2.6x when it is 1.38x |
| Color | srgb_to_linear, tonemap_reinhard |
| Calculus | calc_derivative, calc_integral_simpson |
| Numerical | num_gcd, num_is_prime, cx_mul |
| Tensor | einsum_matmul_2x2, einsum_trace_2x2 |
| Autodiff | grad_fwd_16, grad_rev_16 — the SAME 16-input gradient by n forward passes and by one reverse sweep. ⚠ The forward row computes all 16 partials; timing one dual pass against a full sweep would flatter reverse mode 16x for free |
| Hull / mesh | convex_hull_2d_2k, halfedge_2k_tris, delaunay_2d_400, delaunay_2d_circle_150, triangulate_600gon, triangulate_comb_600, triangulate_hex_6 |
| Spatial indices | bvh_degenerate_4k, bvh_scatter_4k, bvh_query_ray_200x4k, kdtree_build_4k, kdtree_build_octave_512, kdtree_radius_4k, spatial_hash_query_2k |
| Narrowphase | gjk_epa_boxes, gjk_epa_spheres, gjk_epa_sphere_box, gjk_epa_3d_cyl_box, gjk_intersect_box_hit, gjk_intersect_box_miss, gjk_intersect_sph_miss, gjk_intersect_tangent, mpr_intersect_box_hit, mpr_intersect_box_miss, mpr_intersect_tangent, mpr_penetration_boxes |
| Decomposition | svd_golub_kahan_12, eigen_qr_12 |
| Transforms | num_dct_1024, num_dst_1024, num_dct_1023, num_dst_1023 |
| Other | ease_in_out, perlin_2d |

`./scripts/bench-history.sh` appends one CSV row per benchmark to `./bench-history.csv`
and `./benchmarks.md` renders the trend. Each harness line reads
`name: <avg> avg (min=<min> max=<max>) [N iters]`; the script parses all four and records
`stat=avg`. Through 2.9.1 it took the *last* number on the line, so 44 of 55 rows tracked
**max**, not average — every row it ever wrote. ⚠ **A second measurement defect was fixed in 2.10.0**: `bench_run` wraps a `clock_gettime` PAIR around every call (~240 ns), so 17 sub-microsecond benchmarks were ~95% instrumentation — all 17 recorded 1,445–1,765 ns against true costs of 27–287 ns, flattening a 10x spread into 1.2x. They now use `bench_batch(…, 2000, 200)`; `benchmarks.md` carries a dated *Measurement changes* header, emitted by the generator so it survives regeneration. **Reach for `bench_batch` for anything under ~1 us** — `bench()` cannot see it. Rows are only compared against rows sharing
the newest run's `stat`, and the significance threshold is ±10%: three back-to-back runs of
the same binary put the median avg spread at 3.5% (worst 6.9%, none over 20%), against
−93%…+742% swings on the old max-based rows.

## Fuzz Targets (5)

| Target | Input bytes | Invariant checked |
|--------|------------|-------------------|
| vec3 ops | 48 | normalize length finite |
| quat rotation | 56 | rotation preserves length |
| ray intersections | 72 | no crash |
| num_gcd | 16 | gcd divides both inputs |
| m4_inverse | 128 | M * M^-1 ≈ I when det > 0 |

## Testing Patterns

### ⭐ Compare floats bit-exactly unless you can say why not (2.13.0)

`f64_to` **truncates**. `assert_eq(f64_to(v), 3)` accepts every `v` in **[3, 4)** — an absolute error
of up to 1.0. That was 27.1% of this suite until 2.13.0, and the cost was concrete: `hvec2_add` could
return `a + b + 0.9` and **all 3572 assertions passed**.

| use | when |
|---|---|
| `assert_eq(x, f64_from(3), …)` | **the default.** The value is exact — most are, and 809 of 813 converted sites proved it by passing unchanged |
| `assert_f64_eq(x, y, …)` / `_ab_f64_eq` | a transcendental or an iteration makes exactness untrue. Every suite now has one |
| `assert_eq(f64_to(f64_round(x)), n, …)` | you are deliberately testing the rounding |

⭐ **A conversion to bit-exact is self-verifying**: any site whose value was not already exact fails
the moment you convert it. That is what turned this migration into a search — the four sites that
failed were four different real defects, all of which had been passing for the life of the file.

⚠ **And prove the mutant is installed before believing a mutation count.** Two numbers here were
nearly published wrong when a harness's `replace` silently no-op'd and the run returned `0 failing`
— which reads exactly like *"the suite is blind"*. The harness now greps for a marker first.

### ⚠ An assertion written against a defect stops testing anything once the defect is fixed

Added 2.11.4, from two live examples. When a test pins a *dependency's* bug as expected behaviour,
the upstream repair is what breaks it — and how it breaks depends entirely on how the assertion
compares:

| assertion style | what happens when upstream fixes the defect |
|---|---|
| exact / bit compare | **FAILS loudly.** You are told. |
| tolerance, round, or truncate | **KEEPS PASSING**, silently, while the property it claimed to test evaporates |

2.11.3 caught two assertions of the first kind (`f64_pow(-2,4)` read 15, upstream made it 16;
`cx_exp(-inf)` returned NaN, upstream made it +0). 2.11.4 found two of the second kind that had been
green the whole time:

- `tests/abuse.tcyr` asserted `(-2)^3 -> -8` and labelled it *"truncation-discriminating: the stdlib
  path gives -7"*. ganita 1.2.4's binary exponentiation makes the stdlib return exactly −8, so the
  pair discriminates **nothing** — but it compares through `f64_to`, so it still passed.
- `tests/modules.tcyr` compared `dual_pow(2,3)` against 8 through `LOOSE_TOL_M`, with a comment
  explaining that `f64_pow` goes through exp/ln so the value is 7.999…. It is now exactly 8. The
  tolerance hid the improvement **and** would have admitted any future regression up to the
  tolerance.

**The rule:** if an assertion's comment explains *why* the value is imprecise, that comment is a
dependency claim with an expiry date. Prefer a bit-exact compare wherever the value is genuinely
exact, and when a tolerance is truly needed, say what sets its size — so the next reader can tell a
loosened bound from a required one. Neither of these was found by running the suite; both were found
by re-reading the rationale beside them.

⚠ Related and already measured: the 2026-08-11 audit found **836 of 3510 assertions (23.8%) compare
through `f64_to`, which TRUNCATES**. That is the same hazard at scale.

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

See [docs/benchmarks-rust-v-cyrius.md](../benchmarks-rust-v-cyrius.md) for Rust vs Cyrius benchmark comparison with 90 Rust + 21 Cyrius data points.
