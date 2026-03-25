//! Parallel batch operations powered by [rayon](https://docs.rs/rayon).
//!
//! Provides parallel versions of computationally expensive operations.
//! Enable with the `parallel` feature flag.

use rayon::prelude::*;

/// Apply a 3D transform to a batch of points in parallel.
///
/// Returns a new vector of transformed points.
#[cfg(feature = "transforms")]
#[must_use]
pub fn par_transform_points(
    transform: &crate::transforms::Transform3D,
    points: &[glam::Vec3],
) -> Vec<glam::Vec3> {
    points
        .par_iter()
        .map(|p| transform.apply_to_point(*p))
        .collect()
}

/// Parallel ray-AABB intersection test against a batch of AABBs.
///
/// Returns a vector of `(index, t)` for each AABB that the ray hits.
#[cfg(feature = "geo")]
#[must_use]
pub fn par_ray_aabb_batch(ray: &crate::geo::Ray, aabbs: &[crate::geo::Aabb]) -> Vec<(usize, f32)> {
    aabbs
        .par_iter()
        .enumerate()
        .filter_map(|(i, aabb)| crate::geo::ray_aabb(ray, aabb).map(|t| (i, t)))
        .collect()
}

/// Parallel ray-sphere intersection test against a batch of spheres.
///
/// Returns a vector of `(index, t)` for each sphere that the ray hits.
#[cfg(feature = "geo")]
#[must_use]
pub fn par_ray_sphere_batch(
    ray: &crate::geo::Ray,
    spheres: &[crate::geo::Sphere],
) -> Vec<(usize, f32)> {
    spheres
        .par_iter()
        .enumerate()
        .filter_map(|(i, s)| crate::geo::ray_sphere(ray, s).map(|t| (i, t)))
        .collect()
}

/// Parallel matrix-vector multiply for dense matrices.
///
/// Each row is computed independently in parallel.
///
/// # Errors
///
/// Returns [`crate::HisabError::InvalidInput`] if dimensions are incompatible.
#[cfg(feature = "num")]
#[must_use = "returns the product vector or an error"]
pub fn par_matrix_vector_multiply(
    a: &[Vec<f64>],
    x: &[f64],
) -> Result<Vec<f64>, crate::HisabError> {
    let m = a.len();
    if m == 0 {
        return Err(crate::HisabError::InvalidInput("empty matrix".into()));
    }
    let n = a[0].len();
    if x.len() != n {
        return Err(crate::HisabError::InvalidInput(format!(
            "x length {} != matrix columns {n}",
            x.len()
        )));
    }

    let y: Vec<f64> = a
        .par_iter()
        .map(|row| row.iter().zip(x.iter()).map(|(a, b)| a * b).sum())
        .collect();
    Ok(y)
}

/// Parallel element-wise operation on a vector.
///
/// Applies `f` to each element in parallel and returns the results.
#[must_use]
pub fn par_map(data: &[f64], f: impl Fn(f64) -> f64 + Sync) -> Vec<f64> {
    data.par_iter().map(|&x| f(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "transforms")]
    fn par_transform_basic() {
        use crate::transforms::Transform3D;
        let t = Transform3D::new(
            glam::Vec3::new(1.0, 0.0, 0.0),
            glam::Quat::IDENTITY,
            glam::Vec3::ONE,
        );
        let points: Vec<glam::Vec3> = (0..100).map(|i| glam::Vec3::splat(i as f32)).collect();
        let result = par_transform_points(&t, &points);
        assert_eq!(result.len(), 100);
        // First point: (0,0,0) + (1,0,0) = (1,0,0)
        assert!((result[0].x - 1.0).abs() < 1e-5);
    }

    #[test]
    #[cfg(feature = "geo")]
    fn par_ray_aabb_basic() {
        use crate::geo::{Aabb, Ray};
        let ray = Ray::new(glam::Vec3::ZERO, glam::Vec3::X).unwrap();
        let aabbs: Vec<Aabb> = (0..10)
            .map(|i| {
                let x = i as f32 * 3.0;
                Aabb::new(
                    glam::Vec3::new(x + 1.0, -1.0, -1.0),
                    glam::Vec3::new(x + 2.0, 1.0, 1.0),
                )
            })
            .collect();
        let hits = par_ray_aabb_batch(&ray, &aabbs);
        assert_eq!(hits.len(), 10); // All should be hit
    }

    #[test]
    #[cfg(feature = "num")]
    fn par_matvec() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let x = [1.0, 1.0];
        let y = par_matrix_vector_multiply(&a, &x).unwrap();
        assert!((y[0] - 3.0).abs() < 1e-12);
        assert!((y[1] - 7.0).abs() < 1e-12);
    }

    #[test]
    fn par_map_basic() {
        let data: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let result = par_map(&data, |x| x * x);
        assert!((result[10] - 100.0).abs() < 1e-12);
    }
}
