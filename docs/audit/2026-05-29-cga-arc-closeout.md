# 2.5.x Closeout — P(-1) / Security / Documentation Audit — 2026-05-29 (v2.5.4)

Closeout pass for the **2.5.x arc** (CGA depth + matrix guard): 2.5.0 contraction,
2.5.1 dual + pseudoscalar inverse, 2.5.2 projection/rejection, 2.5.3
`mat_new_guarded`. Scope: cleanliness, a memory-safety / numerical review of the
six new functions, and the documentation deliverable (the equation catalogue).

**Verdict: posture solid.** No new vulnerability; no source fix required. The
arc's deliverable was additive feature code (new public functions, no signature
changes), all bounded and guarded. The closeout adds the mathematical reference
(`docs/architecture/math.md`) and pins the equation material with the per-patch
GA-identity tests already shipped.

## Cleanliness

- `cyrius lint` / `cyrius fmt --check` across all `src/` + `tests/` + `examples/`:
  **0 warnings**, no drift. No line exceeds 120 chars.
- `cyrius vet src/main.cyr`: 2 deps, **0 untrusted**, 0 missing.
- `cyrius deps --verify`: lockfile intact.
- Tests **929/929** (foundation 307 · modules 312 · hisab 147 · edge_cases 163);
  fuzz harness ok. `cyrius distlib` regenerates clean (no drift).

## Security review — the six 2.5.x functions

### Memory safety — PASS
- `cga_left_contraction`, `cga_right_contraction`, `cga_dual`, `cga_project`,
  `cga_reject` allocate results via `cga_mv_zero()` — a **fixed 256-byte** (32×8)
  multivector. All loops iterate blade indices `0..31`; every `load64`/`store64`
  is `base + idx*8` with `idx ∈ [0,31]`, in-bounds by construction. No
  attacker-influenced or computed-unbounded index, no `alloc` sized from input.
- `mat_new_guarded` is the only allocation sized from caller dimensions, and it
  caps them (see CWE-190 below).

### Division by zero — PASS
- `cga_blade_inverse` (`~B / ⟨B~B⟩₀`): **guarded** — returns the zero multivector
  when `|⟨B~B⟩₀| < EPSILON_F64` (null blades, e.g. conformal points). Pinned by
  the 2.5.2 "project onto null blade = 0 (no trap)" assertion.
- `cga_pseudoscalar_inv` (`~I / ⟨I~I⟩₀`): the denominator is `⟨I~I⟩₀ = −1`, a
  **structural constant** of the unit pseudoscalar the function builds itself —
  no external input flows in, so it can never be zero. Documented; no guard
  needed (a guard here would be unreachable dead code).
- `cga_dual` / `cga_project` / `cga_reject` perform no division of their own.

### Integer overflow (CWE-190) — PASS (mitigated)
- `mat_new_guarded` (2.5.3) caps dimensions at `_MAT_MAX_ELEMS = 16M`, keeping
  `rows*cols` well below 2⁶⁰ so stdlib `mat_new`'s `16 + rows*cols*8` cannot wrap
  i64; non-positive / overflow-prone dims return null. Pinned by 4 regression
  assertions. The underlying stdlib `mat_new` is still unguarded on the pinned
  6.0.14 — **upstream, deferred** until the cyrius pin moves; hisab's internal
  callers stay mitigated (dims from already-allocated matrices).

### Surface — PASS
No new shell/exec/FFI/libc/syscall surface (the arc added pure in-memory math).

## Coverage

CGA grew from **1 smoke assertion → 29** across the arc. The tests are
identity-based: contraction grade rules + `v⌋v = |v|²`; dual grade-flip and
involution; projection idempotence and `project + reject = X`; null-blade and
overflow guards. These pin the load-bearing equations now catalogued in
`docs/architecture/math.md`.

## Documentation

- **`docs/architecture/math.md`** — new. Equation catalogue: the conformal model
  / metric / blade layout, the product family (geometric / outer / left+right
  contraction with grade rules), reverse / norm / blade inverse, dual +
  pseudoscalar, projection / rejection — each with the pinned identities and
  literature references (Dorst-Fontijne-Mann; Hestenes-Sobczyk; Doran-Lasenby) —
  plus a catalogue index pointing at the rest of the library's formula material.
  This earns the `architecture/math.md` doc the CLAUDE.md structure reserved for
  "mathematical reference for algorithms/formulas."
- **`threat-model.md`** — CGA division guards recorded; `mat_new` row cites
  `mat_new_guarded` as the safe constructor.

## Open items (not regressions)

- Stdlib `mat_new` overflow guard — **upstream**, re-verify when the cyrius pin
  advances (tracked in the roadmap; hisab's `mat_new_guarded` is the mitigation).
