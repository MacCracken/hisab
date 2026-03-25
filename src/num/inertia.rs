// ---------------------------------------------------------------------------
// Inertia tensor computation
// ---------------------------------------------------------------------------

/// Compute the inertia tensor of a solid sphere.
#[must_use]
#[inline]
pub fn inertia_sphere(mass: f64, radius: f64) -> Vec<Vec<f64>> {
    let i = 0.4 * mass * radius * radius;
    vec![vec![i, 0.0, 0.0], vec![0.0, i, 0.0], vec![0.0, 0.0, i]]
}

/// Compute the inertia tensor of a solid box (cuboid).
#[must_use]
#[inline]
pub fn inertia_box(mass: f64, hx: f64, hy: f64, hz: f64) -> Vec<Vec<f64>> {
    let w = 2.0 * hx;
    let h = 2.0 * hy;
    let d = 2.0 * hz;
    let c = mass / 12.0;
    vec![
        vec![c * (h * h + d * d), 0.0, 0.0],
        vec![0.0, c * (w * w + d * d), 0.0],
        vec![0.0, 0.0, c * (w * w + h * h)],
    ]
}

/// Compute the inertia tensor of a triangle mesh (solid body).
///
/// Uses the divergence theorem method. The mesh must be closed with
/// consistent outward-facing winding.
///
/// Returns `(volume, center_of_mass, inertia_tensor_3x3)`.
#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn inertia_mesh(
    triangles: &[([f64; 3], [f64; 3], [f64; 3])],
) -> (f64, [f64; 3], Vec<Vec<f64>>) {
    let mut volume = 0.0;
    let mut com = [0.0; 3];
    let mut ii = [[0.0f64; 3]; 3];

    for &(v0, v1, v2) in triangles {
        let d = v0[0] * (v1[1] * v2[2] - v1[2] * v2[1]) - v0[1] * (v1[0] * v2[2] - v1[2] * v2[0])
            + v0[2] * (v1[0] * v2[1] - v1[1] * v2[0]);
        let vol = d / 6.0;
        volume += vol;

        for i in 0..3 {
            com[i] += vol * (v0[i] + v1[i] + v2[i]) / 4.0;
        }

        for i in 0..3 {
            for j in 0..3 {
                ii[i][j] += vol
                    * (v0[i] * v0[j]
                        + v1[i] * v1[j]
                        + v2[i] * v2[j]
                        + (v0[i] + v1[i] + v2[i]) * (v0[j] + v1[j] + v2[j]))
                    / 20.0;
            }
        }
    }

    if volume.abs() > crate::EPSILON_F64 {
        for c in &mut com {
            *c /= volume;
        }
    }

    let trace = ii[0][0] + ii[1][1] + ii[2][2];
    let inertia = vec![
        vec![trace - ii[0][0], -ii[0][1], -ii[0][2]],
        vec![-ii[1][0], trace - ii[1][1], -ii[1][2]],
        vec![-ii[2][0], -ii[2][1], trace - ii[2][2]],
    ];

    (volume, com, inertia)
}
