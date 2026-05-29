# Mathematical Reference

> Equation catalogue for hisab's algorithms. The **deep section** is Conformal
> Geometric Algebra (the focus of the 2.5.x arc); the **catalogue index** at the
> end points at where the rest of the library's formula material lives (source
> headers, the threat-model constants table, the architecture overview).
>
> Conventions: all scalars are IEEE 754 `f64`; near-zero comparisons use
> `EPSILON_F64 = 1e-12`. Blades/multivectors are heap arrays of `f64` components
> indexed by blade index (see the layout table). Vector basis is `{e1, e2, e3}`
> for 3D Euclidean and `{e1, e2, e3, ep, em}` for the 5D conformal model.

---

## 1. Conformal Geometric Algebra (`geo_advanced.cyr`)

The conformal model of 3D Euclidean space is the geometric algebra **Cl(4,1)**
represented here on a 5D basis with metric

```
e1² = e2² = e3² = ep² = +1,   em² = −1
```

Multivectors are 32 = 2⁵ `f64` components (256 bytes). Blade index → basis blade
(lexicographic), with grade and the bit-mask used by the product:

| Grade | Indices | Blades |
|-------|---------|--------|
| 0 | 0 | scalar |
| 1 | 1–5 | e1, e2, e3, ep, em |
| 2 | 6–15 | e12, e13, e1p, e1m, e23, e2p, e2m, e3p, e3m, epm |
| 3 | 16–25 | e123, e12p, …, e3pm |
| 4 | 26–30 | e123p, e123m, e12pm, e13pm, e23pm |
| 5 | 31 | e123pm = **I** (pseudoscalar) |

### 1.1 Products

For basis blades `a`, `b` with bit-masks, the **geometric product** sign is
`(−1)^s · μ` where `s` counts the transpositions needed to merge the masks and
`μ = −1` once for each shared `em` (metric), `+1` otherwise; the result blade is
`bits(a) XOR bits(b)`. For general multivectors the product distributes over
components (`cga_geometric_product`).

Grade-selected products keep only the terms of a target grade:

| Product | Symbol | Result grade kept | Function |
|---------|--------|-------------------|----------|
| Outer (wedge) | a ∧ b | `g(a) + g(b)` | `cga_outer_product` |
| Left contraction | a ⌋ b | `g(b) − g(a)` | `cga_left_contraction` |
| Right contraction | a ⌊ b | `g(a) − g(b)` | `cga_right_contraction` |

A negative target grade selects nothing (the contraction is then zero) — e.g.
`g(a) > g(b)` ⟹ `a ⌋ b = 0`. **Identities** (pinned in `tests/hisab.tcyr`):

- `e1 ⌋ e12 = e2`  (grade 2 → 1)
- `v ⌋ v = |v|²`  for a vector `v` (a scalar; e.g. `(2e1+3e2) ⌋ self = 13`)
- `e1 ⌋ e23 = 0`  (orthogonal blades)
- `s ⌋ B = sB`  for a scalar `s` (contraction by grade-0 is scaling)
- `e12 ⌊ e1 = −e2`  (handedness sign)

### 1.2 Reverse, norm, inverse

The **reverse** `~A` negates grades where `k(k−1)/2` is odd — i.e. grades 2 and 3
(`cga_mv_reverse`). The **scalar norm²** is `⟨A ~A⟩₀` (`cga_norm_sq`), and
`‖A‖ = √|⟨A ~A⟩₀|` (`cga_norm`).

The **blade inverse** (`cga_blade_inverse`) is

```
B⁻¹ = ~B / ⟨B ~B⟩₀
```

with a guard: if `|⟨B ~B⟩₀| < EPSILON_F64` (a **null** blade — e.g. a conformal
point) the inverse is undefined and the function returns the zero multivector
rather than dividing by zero.

### 1.3 Dual

The unit pseudoscalar is `I = e1∧e2∧e3∧ep∧em` (blade [31]). In this metric

```
I² = (−1)^{5·4/2} · (e1²e2²e3²ep²em²) = (+1)(−1) = −1   ⟹   I⁻¹ = −I
```

`cga_pseudoscalar_inv` computes `I⁻¹ = ~I / ⟨I ~I⟩₀`; the denominator
`⟨I ~I⟩₀ = −1` is a structural constant of the unit pseudoscalar (no external
input), so the division is always defined. The **dual** is

```
x* = x · I⁻¹            (cga_dual)
```

Because `I` is the top blade, this geometric product is exactly the contraction
onto the pseudoscalar. It maps **grade k → grade 5 − k**, and in this metric the
involution is

```
dual(dual(x)) = x · I⁻² = −x
```

**Identities** (pinned): `I·I⁻¹ = 1`; `dual(1) = −I` (0→5); `dual(I) = 1` (5→0);
`dual(e1) = −e23pm` (1→4); `dual(dual(e1)) = −e1`.

### 1.4 Projection and rejection

The projection of a blade `X` onto a blade `B` and its orthogonal complement:

```
project(X, B) = (X ⌋ B) ⌋ B⁻¹        (cga_project)
reject(X, B)  = X − project(X, B)     (cga_reject)
```

The projection **preserves grade(X)** (the inner contraction lowers by `g(X)`,
the outer raises it back), so the rejection-by-subtraction is grade-consistent.
**Identities** (pinned): `project(B,B) = B`; `project + reject = X`; idempotence
`project(project(X,B),B) = project(X,B)`; `reject = X` when `X ⊥ B`; and
`project = 0` (no trap) when `B` is null.

### 1.5 Conformal embedding (existing constructors)

A 3D point `(x,y,z)` embeds as a null vector; spheres/planes are grade-1 (dual)
or grade-4 (direct) blades; rigid motions are versors applied by the sandwich
`V x ~V` (`cga_sandwich`). See `cga_point` / `cga_sphere` / `cga_plane` /
`cga_translator` / `cga_rotor` in `geo_advanced.cyr`.

### References

- L. Dorst, D. Fontijne, S. Mann — *Geometric Algebra for Computer Science: An
  Object-Oriented Approach to Geometry*, Morgan Kaufmann, 2007. (Contraction,
  duality, projection conventions §3.) 
- D. Hestenes, G. Sobczyk — *Clifford Algebra to Geometric Calculus*, 1984.
- C. Doran, A. Lasenby — *Geometric Algebra for Physicists*, CUP, 2003.

---

## 2. Catalogue index — where the rest of the equation material lives

hisab's other formula families are documented at their source (header comments)
and in the references below. This index keeps them findable from one place.

| Area | Formulas | Where |
|------|----------|-------|
| Quaternions / rotation | slerp, axis-angle, `q v q⁻¹`, to/from matrix | `quat.cyr`, `mat3.cyr` (`m3_from_quat`) |
| Transforms / projection | SRT compose, Euler orders, perspective/ortho, world↔screen | `transforms.cyr`, `mat4.cyr` |
| Geometry | 6 ray tests, closest-point, GJK/EPA simplex, MPR portal refinement | `geo.cyr`, `geo_advanced.cyr`, `collision_core.cyr` |
| Collision algorithms | monotone-chain hull, ear-clipping, Bowyer-Watson in-circle predicate, sequential-impulse `Δλ = −(1+e)v_eff/m`, PGS/LCP | `collision_core.cyr`, `collision_mesh.cyr`, `linalg_ext.cyr` (`solve_pgs`) |
| Calculus | Simpson / Gauss-Legendre nodes, Bezier/B-spline/NURBS bases, gradient/Jacobian/Hessian, Perlin/simplex | `calc.cyr`, `calc_ext.cyr`, `noise_simplex.cyr` |
| ODE | RK4, DOPRI45 / BDF-2..5 tableaux, symplectic (Verlet/Yoshida), SDE (Euler-Maruyama/Milstein) | `ode.cyr` (BDF coefficients verified in the threat-model) |
| Numerical LA | CSR, GMRES/BiCGSTAB, SVD (Golub-Kahan), eigen (power/Jacobi), Lyapunov, inertia | `linalg_ext.cyr`, `linalg_precision.cyr` |
| Number theory | extended GCD, totient, Möbius, CRT, Pollard-rho, Russian-peasant `_num_mulmod` | `num.cyr`, `num_ext.cyr` |
| Complex / physics | matrix exponential, Pauli/Dirac γ, einsum contraction | `complex.cyr`, `einsum.cyr` |
| Lie groups | U(1)/SU(2)/SU(3) generators, SO(3,1), SE(3)/SO(3) exp/log, adjoint, BCH | `lie.cyr`, `lie_ext.cyr` |
| Differential geometry | Christoffel → Riemann/Ricci/Einstein, geodesic RK4, Killing, exterior algebra | `diffgeo.cyr` (depth expansion → roadmap 2.6.0) |
| Tensors | Kronecker δ, Minkowski η, Levi-Civita ε, einsum | `tensor.cyr`, `einsum.cyr` |
| Verified constants | `EPSILON_F64`, `F64_PI`, `F64_E`, BDF-5 coefficients (IEEE-754 verified) | [`../development/threat-model.md`](../development/threat-model.md) |

Module map and data flow: [`overview.md`](overview.md).
