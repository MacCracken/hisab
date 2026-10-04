# Migrating to hisab 3.3.0

> 3.3.0 makes the **public surface enforced**. Since 3.1.0 every name meant for callers has
> carried `public`. From 3.3.0 the bundle is `private`, so `dist/hisab.cyr` refuses every
> other name: `'X' is private to its file`, and no binary. (Enum constants are the
> language's exception: they carry no visibility, so a non-public enum's members stay
> readable, and they are `_`-named.) The release also removes the names that were public
> only by accident, and gives seven internal helpers real public names. Results change only
> in number theory and in one tolerance (below): six functions that were silently wrong
> above 2^62, `num_crt`'s returned x for negative remainders, and `sym_const_eq`.

## Does this affect me?

Probably not, if you use hisab through its documented API. The surface the ten live
consumers reach (34 public functions in 8 modules, re-counted 2026-09-30; 35 at the 2026-09-14 count) is unchanged. **No consumer's source
references any name this release removes or hides**: all 14 repos carrying a hisab bundle
were checked on 2026-09-30. You are affected if you:

- call or read a name starting with `_` — those are private now;
- use one of the non-underscore names in the table below;
- `include` individual `src/*.cyr` files instead of `dist/hisab.cyr`, and rely on which
  file defines `mpr_intersect`, `mpr_penetration` or `su2_adjoint`.

The minimum toolchain is unchanged: **cyrius ≥ 6.6.3**. The private bundle was built and
run from dirs pinned to 6.6.3, 6.6.6, 6.6.10 and 6.6.12. On each, a program using the public
API runs correctly. All 511 non-public names the public-surface gate probes (every private
fn and global, plus the private structs' accessors) are refused when named directly. ⚠ On
**6.6.3 only**, a private function can still be reached through its address
(`fncall1(&_name, …)`). That is an upstream defect, fixed in 6.6.4. Do not rely on it: it is
not API, and it goes away when you move your pin.

## What was removed, and what to use instead

| removed | use instead |
|---|---|
| `F64_THREE`, `F64_FOUR`, `F64_FIVE`, `F64_SIX`, `F64_TEN`, `F64_FIFTEEN` (calc), `F64_NINE`, `F64_1E_NEG30` (calc_ext), `F64_SIX_DG` (diffgeo) | your own constant, or `f64_from(n)`. These were module internals squatting the stdlib's `F64_*` namespace; a future stdlib `F64_THREE` would have collided in every consumer |
| `EPSILON_F32` | nothing: it was unused (all math is f64) |
| `AdPowLimit` / `AD_POW_MAX_K` | nothing: unused since 3.2.2 retired the loop it bounded |
| `RenderLayout` / `FLOAT_RENDER_BUF` / `INT_EXACT_BUF` | `sym_const_to_str(val)` renders a constant; you never need the buffer sizes. ⚠ Enum constants carry no visibility in Cyrius, so the renamed `_FLOAT_RENDER_BUF` / `_INT_EXACT_BUF` are still readable, but the `_` says they are not API |
| `GeoJet_*` getters and `GeoJet_set_*` setters | `geo_jet_kind(j)`, `geo_jet_t(j)`, `geo_jet_dorigin(j)` (was `GeoJet_dt_do`), `geo_jet_ddirection(j)` (was `GeoJet_dt_dd`), and the per-primitive accessors (`geo_jet_sphere_dcenter`, …). Read a jet only through these accessors. ⚠ 3.3.0 said the private struct stopped a caller replacing a slot; it did not, because a typed variable (`var j: _GeoJet`) could still write any field. Since 3.3.1 there is no struct type to name, so that code no longer compiles. The vector accessors return the jet's own `HVec3`s by reference, so copy one before changing it |
| `_COL_SENTINEL` | `HALFEDGE_NONE` (collision_mesh): the half-edge "no such index" value |
| `_num_mulmod` | `num_mulmod(a, b, m)`, which returns `Result`: `Ok(r)` with r in [0, m), `Err(HSB_ERR_DIVISION_BY_ZERO)` for m = 0, `Err(HSB_ERR_INVALID_INPUT)` for m < 0. It is exact for every i64 operand and every modulus up to 2^63 − 1 |
| `_num_is_pow2` | `num_is_pow2(n)` |
| `_lie_norm3` | `lie_norm3(x, y, z)` |
| `_lext_sort_desc` | `linalg_sort_desc(...)`, same arguments and tie rule |
| `_perm_init` + `_perm` | `noise_perm(i)`, which builds its table on first use |
| `_su2_alloc`, `_su2_x/_y/_z` | `su2_adjoint(g, v)` for the rotation they were used for; `su2_to_quat(g)` for components |
| `_sym_int_exact_buf`, `_sym_render_f64`, `_SYM_2_POW_63` | `sym_const_to_str(val)` |
| `_sym_is_zero`, `_SYM_EPS` | `sym_const_eq(a, b)`: \|a − b\| < 1e-15, absolute |
| `_noise_fade` | `ease_in_out_smooth(t)`: the same operations in the same order, bit-identical |
| `_GEO_F64_POS_INF`, `_COL_F64_*` | `F64_POS_INF`, `F64_ONE`, `0` |
| `_epa_seed_gjk`, `_epa_seed_portal`, `_epa_refine`, `_epa_touch_probe` | `gjk_epa_3d`, `mpr_penetration` (pipeline internals, not API) |

## What moved

`mpr_intersect` and `mpr_penetration` moved from `collision_core.cyr` to `geo_advanced.cyr`,
beside `gjk_epa_3d` and the EPA helpers they call. `su2_adjoint` moved from `lie_ext.cyr` to
`lie.cyr`. **For a bundle consumer nothing changes.** If you include `src/` files one at a time,
include `geo_advanced.cyr` for MPR. `collision_core.cyr` now needs only `error` and `vec2`, and
`collision_mesh.cyr` only those plus `collision_core.cyr` (island detection reads its `ColContact`).
Through 3.2.x both also needed `geo`, `geo_advanced`, `quat` and `vec3`. The per-module sets in
[`../architecture/overview.md`](../architecture/overview.md) were re-derived for this release by
compiling each module against exactly its listed set.

## Behaviour that changed

These were silent wrong answers on 3.2.2 for inputs above 2^62, measured against exact
arithmetic:

| call | 3.2.2 | 3.3.0 |
|---|---|---|
| `num_modpow(2^62, 2, 2^63 − 1)` | 0 | 2^61 |
| `num_is_prime(2^63 − 25)` (a prime) | 0 | 1 |
| `num_pollard_rho(3037000493 · 3037000453)` | never returned | 3037000453, in 75 ms |
| `num_crt` with M = 3037000493 · 3037000453 | Ok, wrong x | Ok, exact x |
| `num_modinv(5, 2^63 − 1)` | −5534023222112865486 (not an inverse) | 3689348814741910323 |
| `num_modinv(1, 2^63 − 1)` | −1 ("not invertible") | 1 |
| `num_crt` with the single modulus 2^63 − 1 | `Err` | Ok |
| `num_divisor_sigma(2^63 − 1, 0)` | never returned | 96 |
| `num_divisor_sigma(9223372033000000000, 0)` | 440 | 400 |

`num_crt` also now refuses, with `Err(HSB_ERR_INVALID_INPUT)`, a modulus product that does
not fit in i64. Before, it returned Ok with a negative M. It refuses a non-positive modulus up
front as well.

Three changes reach ordinary inputs:

- **`num_crt` returns x in [0, M) at every modulus size.** For a negative remainder, 3.2.2
  returned the same congruence class as a negative number: `(−1, 0) mod (5, 7)` gave −21, and
  gives 14 now. If you normalised x yourself, that still works. If you compared against a
  negative value, update it.
- **`sym_const_eq`'s tolerance is the documented 1e-15.** The constant behind it was
  mis-encoded as 2^-50 (8.88e-16). Differences in [8.88e-16, 1e-15) now compare equal, and the
  symbolic simplifier folds them, as its documentation always said.
- **`linalg_sort_desc` is new public API returning `Result`.** It is `Err(HSB_ERR_INVALID_INPUT)`
  when `vals` is null or a matrix is smaller than the region it swaps.

## The tests keep white-box access

`dist/hisab.cyr` is private because its first module, `src/visibility.cyr`, holds a single
`private` line. The modules themselves are not flipped, so hisab's own suites, which include
`src/*.cyr` one at a time, can still test internals directly.
`scripts/check-public-surface.sh` flips every module in a scratch copy on every CI run. It
proves that no module reaches another's internals, and it checks that the shipped bundle
carries the marker and refuses a private read.

## 3.3.4: `f64_tan` is no longer hisab's

cyrius 6.6.13 ships ganita 1.2.11, which defines `f64_tan` itself (an alias of the fdlibm
`ganita_f64_tan`, within 1 ulp). hisab's own `public fn f64_tan`, a sin/cos quotient, then drew
`duplicate fn 'f64_tan'` on every build, and the last definition (hisab's) replaced ganita's for
the whole program. 3.3.4 removes hisab's definition. hisab's three internal callers
(`m4_perspective_rh`, `m4_perspective_reverse_z`, `se3_log`) inline the same quotient, so their
results do not change on any pin, and the minimum toolchain stays cyrius 6.6.3.

| your pin | `f64_tan(x)` in your code |
|---|---|
| ≥ 6.6.13 | ganita's: same meaning, ≤ 1 ulp; bits differ from hisab's quotient on about a third of [−π/2, π/2] |
| 6.6.3 – 6.6.12 | not defined: a reachable call is refused (`refusing to emit binary with 1 reachable undefined function(s)`). Define a local `f64_div(f64_sin(x), f64_cos(x))` |

goonj and naad call it today (both on hisab 2.22.1).
