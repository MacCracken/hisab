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

## 2. Differential Geometry (`diffgeo.cyr`)

Tensors are flat `f64` arrays: metric `g[i·n+j]`, Christoffel `Γ[(a·n+μ)·n+ν]`,
Riemann `R[((ρ·n+σ)·n+μ)·n+ν]` (dim `n` capped at 16). Index helpers `_dg_idx2/3/4`.

### 2.1 Curvature tensors

```
Γ^a_{μν} = ½ g^{aλ} (∂_μ g_{λν} + ∂_ν g_{λμ} − ∂_λ g_{μν})
R^ρ_{σμν} = ∂_μ Γ^ρ_{νσ} − ∂_ν Γ^ρ_{μσ} + Γ^ρ_{μλ}Γ^λ_{νσ} − Γ^ρ_{νλ}Γ^λ_{μσ}
```

`R^ρ_{σμν}` is antisymmetric in the **last two** indices (μ,ν). **Sign convention**
(load-bearing for everything below): a sphere has *positive* curvature — verified
numerically, `R^θ_{φθφ} = +1` at the equator of the unit 2-sphere. Then

```
Ricci   R_{μν} = R^λ_{μλν}            (contract 1st & 3rd)
scalar  R      = g^{μν} R_{μν}
Einstein G_{μν} = R_{μν} − ½ R g_{μν}
```

### 2.2 Sectional curvature (`sectional_curvature`)

```
K(u,v) = R(u,v,v,u) / (⟨u,u⟩⟨v,v⟩ − ⟨u,v⟩²),   R(u,v,v,u) = R_{ασμν} u^α v^σ u^μ v^ν
```

with the first index lowered `R_{ασμν} = g_{αρ} R^ρ_{σμν}` and `⟨a,b⟩ = g_{ij}a^i b^j`
(`_dg_inner`). A degenerate plane (`|denominator| < EPSILON`) returns 0. For a
constant-curvature space form `K = K₀` on every plane; a radius-`r` sphere gives
`K = 1/r²`. **Pinned:** space-form `K=1` (axis + skew planes), radius-2 sphere `1/4`,
flat `0`, degenerate `0`.

### 2.3 Weyl conformal-curvature tensor (`weyl_tensor`)

The trace-free part of Riemann (conformally invariant), for `n ≥ 3`:

```
C_{ρσμν} = R_{ρσμν}
         − 1/(n−2) (g_{ρμ}R_{σν} − g_{ρν}R_{σμ} − g_{σμ}R_{ρν} + g_{σν}R_{ρμ})
         + R/((n−1)(n−2)) (g_{ρμ}g_{σν} − g_{ρν}g_{σμ})
```

Returns all-zero for `n ≤ 2` (the `1/(n−2)` factor guards it). **Pinned:** vanishes
for 3D and 4D space forms (conformally flat, Riemann ≠ 0), nonzero for a
non-space-form Riemann, and trace-free `g^{ρμ} C_{ρσμν} = 0`.

### 2.4 Geodesic deviation / Jacobi (`geodesic_deviation`)

```
D²J^ρ/dτ² = −R^ρ_{σμν} u^σ J^μ u^ν
```

(J and the second `u` occupy Riemann's antisymmetric pair). For a space form this
is `−(|u|² J − ⟨u,J⟩ u)`. **Pinned:** unit sphere `J⊥u → J'' = −J` (geodesics
converge), `J∥u → 0`, `|u|²` scaling, flat `0`, linear in `J`.

### 2.5 Parallel transport (`parallel_transport`)

```
dV^a/dt = −Γ^a_{μν} V^μ ẋ^ν     (RK4; Γ constant per step, like geodesic_rk4)
```

Linear in V (`dV/dt = M·V` for fixed `ẋ`). Metric-compatible: `⟨V,V⟩` is preserved.
**Pinned:** flat space leaves V unchanged; a unit-sphere latitude circle (θ=π/4)
preserves `⟨V,V⟩` and rotates the vector.

### 2.6 Exterior algebra (`wedge_*`, `hodge_star_2form_4d`)

Forms use the **reduced strictly-increasing basis**: in 4D a 1-form is `[0,1,2,3]`,
2-form `[01,02,03,12,13,23]`, 3-form `[012,013,023,123]`, 4-form `[0123]`.

```
(α∧β)_{i<j}     = α_i β_j − α_j β_i                         (wedge_1_1)
(ω∧α)_{i<j<k}   = ω_{ij}α_k − ω_{ik}α_j + ω_{jk}α_i          (wedge_2_1, 2∧1→3)
(β∧α)_{0123}    = β_{012}α_3 − β_{013}α_2 + β_{023}α_1 − β_{123}α_0   (wedge_3_1, 3∧1→4)
```

Graded antisymmetry `α∧β = (−1)^{pq} β∧α`, nilpotence `α∧α = 0` (odd degree),
repeated factor → 0. `hodge_star_2form_4d` dualises a 4D 2-form (Lorentzian /
Euclidean via the sign flag). `wedge_2_1` / `wedge_3_1` / the Hodge star are 4D-specific.

### References

- M. do Carmo — *Riemannian Geometry*, Birkhäuser, 1992.
- J. M. Lee — *Introduction to Riemannian Manifolds*, 2nd ed., Springer, 2018.
- R. M. Wald — *General Relativity*, Univ. of Chicago Press, 1984.
- C. Misner, K. Thorne, J. Wheeler — *Gravitation*, Freeman, 1973.

---

## 3. Catalogue index — where the rest of the equation material lives

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
| Differential geometry | Christoffel → Riemann/Ricci/Einstein, sectional/Weyl curvature, geodesic deviation, parallel transport, exterior algebra | **§2 above** + `diffgeo.cyr` |
| Tensors | Kronecker δ, Minkowski η, Levi-Civita ε, einsum | `tensor.cyr`, `einsum.cyr` |
| Verified constants | `EPSILON_F64`, `F64_PI`, `F64_E`, BDF-5 coefficients (IEEE-754 verified) | [`../development/threat-model.md`](../development/threat-model.md) |

Module map and data flow: [`overview.md`](overview.md).
