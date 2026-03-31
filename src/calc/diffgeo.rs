//! Differential geometry operators for curved spacetimes and manifolds.
//!
//! Provides computation of:
//! - Christoffel symbols Γᵅ_μν from a metric tensor
//! - Riemann curvature tensor R^ρ_σμν
//! - Ricci tensor R_μν and Ricci scalar R
//! - Geodesic equation integration
//! - Killing vector detection
//! - Exterior algebra: wedge product, Hodge star, differential forms

use crate::HisabError;

// ---------------------------------------------------------------------------
// Christoffel symbols
// ---------------------------------------------------------------------------

/// Compute the Christoffel symbols of the second kind Γᵅ_μν from a metric.
///
/// `Γᵅ_μν = ½ g^{αλ} (∂_μ g_{νλ} + ∂_ν g_{μλ} − ∂_λ g_{μν})`
///
/// The metric is provided as a function `g(i, j)` returning the metric
/// component g_{ij}, and metric_inv as `g_inv(i, j)` for g^{ij}.
///
/// `dg(i, j, k)` returns `∂_k g_{ij}` (partial derivative of g_{ij} w.r.t. x^k).
///
/// Returns Γᵅ_μν as a flat array indexed `[alpha * dim² + mu * dim + nu]`.
///
/// # Errors
///
/// Returns error if dimension is zero.
#[allow(clippy::needless_range_loop)]
pub fn christoffel_symbols(
    dim: usize,
    g_inv: &dyn Fn(usize, usize) -> f64,
    dg: &dyn Fn(usize, usize, usize) -> f64,
) -> Result<Vec<f64>, HisabError> {
    if dim == 0 {
        return Err(HisabError::InvalidInput(
            "dimension must be positive".into(),
        ));
    }

    let n3 = dim * dim * dim;
    let mut gamma = vec![0.0; n3];

    for alpha in 0..dim {
        for mu in 0..dim {
            for nu in 0..dim {
                let mut sum = 0.0;
                for lambda in 0..dim {
                    let g_al = g_inv(alpha, lambda);
                    // ∂_μ g_{νλ} + ∂_ν g_{μλ} − ∂_λ g_{μν}
                    let bracket = dg(nu, lambda, mu) + dg(mu, lambda, nu) - dg(mu, nu, lambda);
                    sum += g_al * bracket;
                }
                gamma[alpha * dim * dim + mu * dim + nu] = 0.5 * sum;
            }
        }
    }

    Ok(gamma)
}

/// Helper to index a Christoffel symbol array: Γ^alpha_{mu nu}.
#[must_use]
#[inline]
pub fn christoffel_get(gamma: &[f64], dim: usize, alpha: usize, mu: usize, nu: usize) -> f64 {
    gamma[alpha * dim * dim + mu * dim + nu]
}

// ---------------------------------------------------------------------------
// Riemann curvature tensor
// ---------------------------------------------------------------------------

/// Compute the Riemann curvature tensor R^ρ_σμν.
///
/// `R^ρ_σμν = ∂_μ Γ^ρ_νσ − ∂_ν Γ^ρ_μσ + Γ^ρ_μλ Γ^λ_νσ − Γ^ρ_νλ Γ^λ_μσ`
///
/// Uses numerical differentiation of the Christoffel symbols.
///
/// - `gamma`: Christoffel symbols as flat array
/// - `dgamma`: `dgamma(rho, mu, nu, k)` returns `∂_k Γ^rho_{mu nu}`
///
/// Returns R^ρ_σμν as flat array indexed `[rho*d³ + sigma*d² + mu*d + nu]`.
///
/// # Errors
///
/// Returns error if dimension is zero.
#[allow(clippy::needless_range_loop)]
pub fn riemann_tensor(
    dim: usize,
    gamma: &[f64],
    dgamma: &dyn Fn(usize, usize, usize, usize) -> f64,
) -> Result<Vec<f64>, HisabError> {
    if dim == 0 {
        return Err(HisabError::InvalidInput(
            "dimension must be positive".into(),
        ));
    }

    let d4 = dim * dim * dim * dim;
    let mut riemann = vec![0.0; d4];
    let d2 = dim * dim;

    for rho in 0..dim {
        for sigma in 0..dim {
            for mu in 0..dim {
                for nu in 0..dim {
                    // ∂_μ Γ^ρ_νσ - ∂_ν Γ^ρ_μσ
                    let mut val = dgamma(rho, nu, sigma, mu) - dgamma(rho, mu, sigma, nu);

                    // + Γ^ρ_μλ Γ^λ_νσ - Γ^ρ_νλ Γ^λ_μσ
                    for lambda in 0..dim {
                        val += gamma[rho * d2 + mu * dim + lambda]
                            * gamma[lambda * d2 + nu * dim + sigma]
                            - gamma[rho * d2 + nu * dim + lambda]
                                * gamma[lambda * d2 + mu * dim + sigma];
                    }

                    riemann[rho * dim * d2 + sigma * d2 + mu * dim + nu] = val;
                }
            }
        }
    }

    Ok(riemann)
}

/// Helper to index the Riemann tensor: R^rho_{sigma mu nu}.
#[must_use]
#[inline]
pub fn riemann_get(
    riemann: &[f64],
    dim: usize,
    rho: usize,
    sigma: usize,
    mu: usize,
    nu: usize,
) -> f64 {
    riemann[rho * dim * dim * dim + sigma * dim * dim + mu * dim + nu]
}

// ---------------------------------------------------------------------------
// Ricci tensor and scalar
// ---------------------------------------------------------------------------

/// Compute the Ricci tensor R_μν by contracting the Riemann tensor.
///
/// `R_μν = R^λ_μλν`
///
/// Returns R_μν as flat array indexed `[mu * dim + nu]`.
#[allow(clippy::needless_range_loop)]
#[must_use]
pub fn ricci_tensor(riemann: &[f64], dim: usize) -> Vec<f64> {
    let mut ricci = vec![0.0; dim * dim];
    let d3 = dim * dim * dim;
    let d2 = dim * dim;

    for mu in 0..dim {
        for nu in 0..dim {
            let mut sum = 0.0;
            for lambda in 0..dim {
                // R^λ_μλν
                sum += riemann[lambda * d3 + mu * d2 + lambda * dim + nu];
            }
            ricci[mu * dim + nu] = sum;
        }
    }

    ricci
}

/// Compute the Ricci scalar R = g^{μν} R_μν.
#[allow(clippy::needless_range_loop)]
#[must_use]
pub fn ricci_scalar(ricci: &[f64], dim: usize, g_inv: &dyn Fn(usize, usize) -> f64) -> f64 {
    let mut scalar = 0.0;
    for mu in 0..dim {
        for nu in 0..dim {
            scalar += g_inv(mu, nu) * ricci[mu * dim + nu];
        }
    }
    scalar
}

/// Compute the Einstein tensor G_μν = R_μν − ½ R g_μν.
///
/// Returns G_μν as flat array indexed `[mu * dim + nu]`.
#[allow(clippy::needless_range_loop)]
#[must_use]
pub fn einstein_tensor(
    ricci: &[f64],
    ricci_scalar_val: f64,
    dim: usize,
    g: &dyn Fn(usize, usize) -> f64,
) -> Vec<f64> {
    let mut einstein = vec![0.0; dim * dim];
    for mu in 0..dim {
        for nu in 0..dim {
            einstein[mu * dim + nu] = ricci[mu * dim + nu] - 0.5 * ricci_scalar_val * g(mu, nu);
        }
    }
    einstein
}

// ---------------------------------------------------------------------------
// Geodesic equation
// ---------------------------------------------------------------------------

/// State for geodesic integration: position x^μ and velocity dx^μ/dτ.
#[derive(Debug, Clone)]
pub struct GeodesicState {
    /// Position coordinates x^μ.
    pub x: Vec<f64>,
    /// Velocity (tangent vector) dx^μ/dτ.
    pub u: Vec<f64>,
}

/// Integrate the geodesic equation using RK4.
///
/// `d²x^α/dτ² + Γ^α_μν (dx^μ/dτ)(dx^ν/dτ) = 0`
///
/// - `gamma_fn`: computes Γ^α_μν at a given position. Returns flat array.
/// - `initial`: initial position and velocity
/// - `dtau`: proper time step
/// - `steps`: number of steps
///
/// Returns the trajectory as a sequence of `GeodesicState`.
///
/// # Errors
///
/// Returns error if dimensions are inconsistent.
pub fn geodesic_rk4(
    dim: usize,
    gamma_fn: &dyn Fn(&[f64]) -> Vec<f64>,
    initial: &GeodesicState,
    dtau: f64,
    steps: usize,
) -> Result<Vec<GeodesicState>, HisabError> {
    if initial.x.len() != dim || initial.u.len() != dim {
        return Err(HisabError::InvalidInput(
            "initial state dimensions don't match".into(),
        ));
    }

    let mut trajectory = Vec::with_capacity(steps + 1);
    trajectory.push(initial.clone());

    let mut x = initial.x.clone();
    let mut u = initial.u.clone();

    for _ in 0..steps {
        // RK4 for the coupled system:
        //   dx^α/dτ = u^α
        //   du^α/dτ = -Γ^α_μν u^μ u^ν
        let accel = |pos: &[f64], vel: &[f64]| -> Vec<f64> {
            let gamma = gamma_fn(pos);
            let mut acc = vec![0.0; dim];
            for (alpha, acc_a) in acc.iter_mut().enumerate() {
                let mut sum = 0.0;
                for mu in 0..dim {
                    for nu in 0..dim {
                        sum += christoffel_get(&gamma, dim, alpha, mu, nu) * vel[mu] * vel[nu];
                    }
                }
                *acc_a = -sum;
            }
            acc
        };

        // k1
        let k1x: Vec<f64> = u.iter().map(|&v| dtau * v).collect();
        let a1 = accel(&x, &u);
        let k1u: Vec<f64> = a1.iter().map(|&a| dtau * a).collect();

        // k2
        let x2: Vec<f64> = x
            .iter()
            .zip(k1x.iter())
            .map(|(&xi, &ki)| xi + 0.5 * ki)
            .collect();
        let u2: Vec<f64> = u
            .iter()
            .zip(k1u.iter())
            .map(|(&ui, &ki)| ui + 0.5 * ki)
            .collect();
        let k2x: Vec<f64> = u2.iter().map(|&v| dtau * v).collect();
        let a2 = accel(&x2, &u2);
        let k2u: Vec<f64> = a2.iter().map(|&a| dtau * a).collect();

        // k3
        let x3: Vec<f64> = x
            .iter()
            .zip(k2x.iter())
            .map(|(&xi, &ki)| xi + 0.5 * ki)
            .collect();
        let u3: Vec<f64> = u
            .iter()
            .zip(k2u.iter())
            .map(|(&ui, &ki)| ui + 0.5 * ki)
            .collect();
        let k3x: Vec<f64> = u3.iter().map(|&v| dtau * v).collect();
        let a3 = accel(&x3, &u3);
        let k3u: Vec<f64> = a3.iter().map(|&a| dtau * a).collect();

        // k4
        let x4: Vec<f64> = x.iter().zip(k3x.iter()).map(|(&xi, &ki)| xi + ki).collect();
        let u4: Vec<f64> = u.iter().zip(k3u.iter()).map(|(&ui, &ki)| ui + ki).collect();
        let k4x: Vec<f64> = u4.iter().map(|&v| dtau * v).collect();
        let a4 = accel(&x4, &u4);
        let k4u: Vec<f64> = a4.iter().map(|&a| dtau * a).collect();

        // Update
        for i in 0..dim {
            x[i] += (k1x[i] + 2.0 * k2x[i] + 2.0 * k3x[i] + k4x[i]) / 6.0;
            u[i] += (k1u[i] + 2.0 * k2u[i] + 2.0 * k3u[i] + k4u[i]) / 6.0;
        }

        trajectory.push(GeodesicState {
            x: x.clone(),
            u: u.clone(),
        });
    }

    Ok(trajectory)
}

// ---------------------------------------------------------------------------
// Killing vectors
// ---------------------------------------------------------------------------

/// Check if a vector field ξ^μ is a Killing vector at a point.
///
/// Killing's equation: `∇_μ ξ_ν + ∇_ν ξ_μ = 0`
///
/// `xi` is the vector field ξ^μ (contravariant).
/// `dxi(mu, nu)` returns `∂_ν ξ^μ` (partial derivative).
/// `gamma` is the Christoffel symbols at the point.
/// `g(i,j)` is the metric.
///
/// Returns the maximum absolute value of the Killing equation residual.
#[allow(clippy::needless_range_loop)]
#[must_use]
pub fn killing_residual(
    dim: usize,
    xi: &[f64],
    dxi: &dyn Fn(usize, usize) -> f64,
    gamma: &[f64],
    g: &dyn Fn(usize, usize) -> f64,
) -> f64 {
    let mut max_residual = 0.0_f64;

    for mu in 0..dim {
        for nu in mu..dim {
            // ∇_μ ξ_ν = ∂_μ ξ_ν - Γ^λ_μν ξ_λ
            // But we have ξ^μ (contravariant), so:
            // ξ_ν = g_{νλ} ξ^λ
            // ∇_μ ξ_ν = g_{νλ} (∂_μ ξ^λ + Γ^λ_μρ ξ^ρ) ... this simplifies to:
            // ∇_μ ξ_ν = g_{νλ} ∂_μ ξ^λ + g_{νλ} Γ^λ_μρ ξ^ρ

            let mut nabla_mu_xi_nu = 0.0;
            for lambda in 0..dim {
                nabla_mu_xi_nu += g(nu, lambda) * dxi(lambda, mu);
                for rho in 0..dim {
                    nabla_mu_xi_nu +=
                        g(nu, lambda) * christoffel_get(gamma, dim, lambda, mu, rho) * xi[rho];
                }
            }

            let mut nabla_nu_xi_mu = 0.0;
            for lambda in 0..dim {
                nabla_nu_xi_mu += g(mu, lambda) * dxi(lambda, nu);
                for rho in 0..dim {
                    nabla_nu_xi_mu +=
                        g(mu, lambda) * christoffel_get(gamma, dim, lambda, nu, rho) * xi[rho];
                }
            }

            let residual = (nabla_mu_xi_nu + nabla_nu_xi_mu).abs();
            max_residual = max_residual.max(residual);
        }
    }

    max_residual
}

// ---------------------------------------------------------------------------
// Exterior algebra
// ---------------------------------------------------------------------------

/// Wedge product of two differential forms.
///
/// If α is a p-form and β is a q-form on an n-dimensional space,
/// α ∧ β is a (p+q)-form.
///
/// Forms are represented as flat arrays in the antisymmetric basis.
/// For a p-form in n dimensions, there are C(n, p) components.
///
/// For 1-forms (vectors), the wedge product produces a 2-form:
/// `(α ∧ β)_{ij} = α_i β_j − α_j β_i`
///
/// # Errors
///
/// Returns error for unsupported form degrees.
pub fn wedge_1_1(alpha: &[f64], beta: &[f64], dim: usize) -> Result<Vec<f64>, HisabError> {
    if alpha.len() != dim || beta.len() != dim {
        return Err(HisabError::InvalidInput(format!(
            "1-form dimension mismatch: expected {dim}"
        )));
    }

    // Result is a 2-form: C(dim, 2) independent components
    let n2 = dim * (dim - 1) / 2;
    let mut result = Vec::with_capacity(n2);

    for i in 0..dim {
        for j in (i + 1)..dim {
            result.push(alpha[i] * beta[j] - alpha[j] * beta[i]);
        }
    }

    Ok(result)
}

/// Wedge product of a 1-form and a 2-form, producing a 3-form.
///
/// # Errors
///
/// Returns error for dimension mismatch.
pub fn wedge_1_2(alpha: &[f64], beta: &[f64], dim: usize) -> Result<Vec<f64>, HisabError> {
    if alpha.len() != dim {
        return Err(HisabError::InvalidInput("1-form dimension mismatch".into()));
    }
    let n2 = dim * (dim - 1) / 2;
    if beta.len() != n2 {
        return Err(HisabError::InvalidInput("2-form dimension mismatch".into()));
    }

    // Result is a 3-form: C(dim, 3) components
    let n3 = dim * (dim - 1) * (dim - 2) / 6;
    let mut result = vec![0.0; n3];

    // β_{ij} indexed by strictly increasing (i,j)
    let beta_idx = |i: usize, j: usize| -> f64 {
        if i < j {
            let pos = i * dim - i * (i + 1) / 2 + (j - i - 1);
            beta[pos]
        } else if j < i {
            let pos = j * dim - j * (j + 1) / 2 + (i - j - 1);
            -beta[pos]
        } else {
            0.0
        }
    };

    // (α ∧ β)_{ijk} = α_i β_{jk} - α_j β_{ik} + α_k β_{ij}
    let mut pos = 0;
    for i in 0..dim {
        for j in (i + 1)..dim {
            for k in (j + 1)..dim {
                result[pos] = alpha[i] * beta_idx(j, k) - alpha[j] * beta_idx(i, k)
                    + alpha[k] * beta_idx(i, j);
                pos += 1;
            }
        }
    }

    Ok(result)
}

/// Hodge star operator for a p-form in n dimensions with metric.
///
/// Currently supports the Hodge star for 2-forms in 4D Minkowski spacetime.
/// `(*F)_{μν} = ½ ε_{μνρσ} F^{ρσ}`
///
/// `form`: the 2-form as C(4,2)=6 components in order (01, 02, 03, 12, 13, 23).
/// `metric_det_sign`: sign of the metric determinant (+1 for Euclidean, -1 for Lorentzian).
///
/// # Errors
///
/// Returns error if form length doesn't match.
pub fn hodge_star_2form_4d(form: &[f64], metric_det_sign: f64) -> Result<[f64; 6], HisabError> {
    if form.len() != 6 {
        return Err(HisabError::InvalidInput(
            "expected 6-component 2-form in 4D".into(),
        ));
    }

    // Components indexed as: 0→(01), 1→(02), 2→(03), 3→(12), 4→(13), 5→(23)
    //
    // For Lorentzian metric (det = -1), the Hodge dual of a 2-form satisfies **F = -F.
    // (*F)_{μν} = (1/2) √|g| ε_{μνρσ} g^{ρα} g^{σβ} F_{αβ}
    //
    // With η = diag(+1,-1,-1,-1):
    //   *(01) = F_{23},  *(02) = -F_{13}, *(03) = F_{12}
    //   *(12) = -F_{03}, *(13) = F_{02},  *(23) = -F_{01}
    let _ = metric_det_sign; // sign is baked into the mapping for Minkowski

    Ok([
        form[5],  // *(01) = F_{23}
        -form[4], // *(02) = -F_{13}
        form[3],  // *(03) = F_{12}
        -form[2], // *(12) = -F_{03}
        form[1],  // *(13) = F_{02}
        -form[0], // *(23) = -F_{01}
    ])
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-8;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < TOL
    }

    // -- Flat spacetime (all Christoffels and curvatures should vanish) --

    #[test]
    fn flat_spacetime_christoffels() {
        // Minkowski metric: η = diag(1, -1, -1, -1)
        // All derivatives of the metric are zero → all Christoffels vanish
        let g_inv = |mu: usize, nu: usize| -> f64 {
            if mu == nu {
                if mu == 0 { 1.0 } else { -1.0 }
            } else {
                0.0
            }
        };
        let dg = |_i: usize, _j: usize, _k: usize| -> f64 { 0.0 };

        let gamma = christoffel_symbols(4, &g_inv, &dg).unwrap();
        for &g in &gamma {
            assert!(approx(g, 0.0), "flat spacetime Christoffel should be 0");
        }
    }

    #[test]
    fn flat_spacetime_riemann() {
        let gamma = vec![0.0; 64]; // All zero Christoffels
        let dgamma = |_rho: usize, _mu: usize, _nu: usize, _k: usize| -> f64 { 0.0 };

        let riemann = riemann_tensor(4, &gamma, &dgamma).unwrap();
        for &r in &riemann {
            assert!(approx(r, 0.0), "flat spacetime Riemann should be 0");
        }
    }

    #[test]
    fn flat_spacetime_ricci() {
        let riemann = vec![0.0; 256];
        let ricci = ricci_tensor(&riemann, 4);
        for &r in &ricci {
            assert!(approx(r, 0.0));
        }
    }

    #[test]
    fn flat_spacetime_ricci_scalar() {
        let ricci = vec![0.0; 16];
        let g_inv = |mu: usize, nu: usize| -> f64 {
            if mu == nu {
                if mu == 0 { 1.0 } else { -1.0 }
            } else {
                0.0
            }
        };
        assert!(approx(ricci_scalar(&ricci, 4, &g_inv), 0.0));
    }

    // -- 2-sphere (positive curvature) --

    #[test]
    fn sphere_christoffel_symbols() {
        // 2-sphere with metric g = diag(1, sin²θ) at θ = π/4
        // Coordinates: (θ, φ)
        // Non-zero Christoffels:
        //   Γ^θ_φφ = -sinθ cosθ
        //   Γ^φ_θφ = Γ^φ_φθ = cosθ/sinθ = cotθ
        let theta = std::f64::consts::FRAC_PI_4;
        let sin_t = theta.sin();
        let cos_t = theta.cos();

        let g_inv = |i: usize, j: usize| -> f64 {
            if i == j {
                if i == 0 { 1.0 } else { 1.0 / (sin_t * sin_t) }
            } else {
                0.0
            }
        };

        // ∂_k g_{ij}
        // g_{00} = 1 (constant), g_{11} = sin²θ
        // ∂_0 g_{11} = 2 sinθ cosθ, all others = 0
        let dg = |i: usize, j: usize, k: usize| -> f64 {
            if i == 1 && j == 1 && k == 0 {
                2.0 * sin_t * cos_t
            } else {
                0.0
            }
        };

        let gamma = christoffel_symbols(2, &g_inv, &dg).unwrap();

        // Γ^0_11 = -sinθ cosθ
        assert!(
            approx(christoffel_get(&gamma, 2, 0, 1, 1), -sin_t * cos_t),
            "Γ^θ_φφ failed"
        );

        // Γ^1_01 = Γ^1_10 = cosθ/sinθ
        let cot = cos_t / sin_t;
        assert!(
            approx(christoffel_get(&gamma, 2, 1, 0, 1), cot),
            "Γ^φ_θφ failed"
        );
        assert!(
            approx(christoffel_get(&gamma, 2, 1, 1, 0), cot),
            "Γ^φ_φθ failed"
        );
    }

    // -- Geodesic --

    #[test]
    fn geodesic_flat_space_straight_line() {
        // In flat space, geodesics are straight lines
        let gamma_fn = |_pos: &[f64]| -> Vec<f64> {
            vec![0.0; 27] // 3D, all zero Christoffels
        };

        let initial = GeodesicState {
            x: vec![0.0, 0.0, 0.0],
            u: vec![1.0, 2.0, 3.0],
        };

        let traj = geodesic_rk4(3, &gamma_fn, &initial, 0.01, 100).unwrap();
        let final_state = &traj[100];

        // After t=1.0, should be at (1, 2, 3)
        assert!(approx(final_state.x[0], 1.0));
        assert!(approx(final_state.x[1], 2.0));
        assert!(approx(final_state.x[2], 3.0));
    }

    // -- Killing vectors --

    #[test]
    fn flat_space_translation_is_killing() {
        // In flat 3D Euclidean space, ξ = (1, 0, 0) is a Killing vector
        let xi = [1.0, 0.0, 0.0];
        let dxi = |_mu: usize, _nu: usize| -> f64 { 0.0 }; // constant vector field
        let gamma = vec![0.0; 27]; // flat space
        let g = |i: usize, j: usize| -> f64 { if i == j { 1.0 } else { 0.0 } };

        let residual = killing_residual(3, &xi, &dxi, &gamma, &g);
        assert!(residual < 1e-10);
    }

    // -- Exterior algebra --

    #[test]
    fn wedge_1_1_antisymmetric() {
        let alpha = vec![1.0, 0.0, 0.0];
        let beta = vec![0.0, 1.0, 0.0];
        let result = wedge_1_1(&alpha, &beta, 3).unwrap();
        // (α ∧ β)_{01} = 1·1 - 0·0 = 1
        // (α ∧ β)_{02} = 1·0 - 0·0 = 0
        // (α ∧ β)_{12} = 0·0 - 1·0 = 0
        assert!(approx(result[0], 1.0));
        assert!(approx(result[1], 0.0));
        assert!(approx(result[2], 0.0));
    }

    #[test]
    fn wedge_self_vanishes() {
        let alpha = vec![1.0, 2.0, 3.0];
        let result = wedge_1_1(&alpha, &alpha, 3).unwrap();
        for &v in &result {
            assert!(approx(v, 0.0), "α ∧ α should vanish");
        }
    }

    #[test]
    fn hodge_star_involution() {
        // For Minkowski 2-forms: **F = -F (Lorentzian)
        let f = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0]; // F_{01} = 1
        let star_f = hodge_star_2form_4d(&f, -1.0).unwrap();
        let star_star_f = hodge_star_2form_4d(&star_f, -1.0).unwrap();
        // **F = -F in Lorentzian signature
        for i in 0..6 {
            assert!(approx(star_star_f[i], -f[i]), "**F ≠ -F at component {i}");
        }
    }

    #[test]
    fn einstein_flat() {
        let ricci = vec![0.0; 16];
        let g = |i: usize, j: usize| -> f64 {
            if i == j {
                if i == 0 { 1.0 } else { -1.0 }
            } else {
                0.0
            }
        };
        let einstein = einstein_tensor(&ricci, 0.0, 4, &g);
        for &e in &einstein {
            assert!(approx(e, 0.0));
        }
    }
}
