//! Iterated Function System implementation
//!
//! This module provides the SigmaFactorIFS struct and related functions.
//! The implementation is based on the SVD approach proposed in the
//! [Improving Fractal Pre-training](http://catalys1.github.io/fractal-pretraining/) paper.

use nalgebra::{Matrix2, Rotation2, Vector2};
use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;

use crate::core::affine::Affine;
use crate::core::types::{Matrix2f, Vector2f, IFS};
use crate::core::utils::uniform;

/// SigmaFactorIFS struct
///
/// Represents an Iterated Function System based on the sigma-factor approach
#[derive(Debug, Clone)]
pub struct SigmaFactorIFS {
    /// The affine transformations
    pub transforms: Vec<Affine>,

    /// The probability distribution for selecting transformations
    pub weights: Vec<f64>,
}

impl SigmaFactorIFS {
    /// Create a new SigmaFactorIFS
    ///
    /// # Arguments
    ///
    /// * `transforms` - The affine transformations
    /// * `weights` - The probability weights for selecting transformations
    ///
    /// # Returns
    ///
    /// A new SigmaFactorIFS
    pub fn new(transforms: Vec<Affine>, weights: Vec<f64>) -> Self {
        assert_eq!(
            transforms.len(),
            weights.len(),
            "Number of transforms must match number of weights"
        );
        Self {
            transforms,
            weights,
        }
    }
}

impl IFS for SigmaFactorIFS {
    const DIM: usize = 2;
    type Scalar = f64;

    fn apply_random<R: Rng>(&self, rng: &mut R, point: &Vector2f) -> Vector2f {
        let dist = WeightedIndex::new(&self.weights).unwrap();
        let idx = dist.sample(rng);
        self.transforms[idx].apply(point)
    }
}

/// Sample singular values for the sigma-factor approach
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `alpha` - The sigma-factor
/// * `n` - The number of transformations
///
/// # Returns
///
/// A vector of (sigma1, sigma2) pairs
pub fn sample_svs<R: Rng>(rng: &mut R, alpha: f64, n: usize) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(n);

    // Sampling bounds
    let mut b_lower = alpha - 3.0 * (n as f64) + 3.0;
    let mut b_upper = alpha;

    // Sample n-1 pairs
    for _ in 0..(n - 1) {
        // Define sigma1
        let sigma1 = uniform(rng, f64::max(0.0, b_lower / 3.0), f64::min(1.0, b_upper));
        b_lower -= sigma1;
        b_upper -= sigma1;

        // Define sigma2
        let sigma2 = uniform(
            rng,
            f64::max(0.0, 0.5 * b_lower),
            f64::min(sigma1, 0.5 * b_upper),
        );
        b_lower = b_lower - 2.0 * sigma2 + 3.0;
        b_upper -= 2.0 * sigma2;

        result.push((sigma1, sigma2));
    }

    // Last pair
    let sigma2 = uniform(rng, f64::max(0.0, 0.5 * (b_upper - 1.0)), b_upper / 3.0);
    let sigma1 = b_upper - 2.0 * sigma2;
    result.push((sigma1, sigma2));

    result
}

/// Create a random rotation matrix
///
/// # Arguments
///
/// * `rng` - Random number generator
///
/// # Returns
///
/// A random rotation matrix
fn random_rotation<R: Rng>(rng: &mut R) -> Matrix2f {
    let angle = uniform(rng, 0.0, 2.0 * std::f64::consts::PI);
    Rotation2::new(angle).into_inner()
}

/// Create a random diagonal matrix with entries in {-1, 1}
///
/// # Arguments
///
/// * `rng` - Random number generator
///
/// # Returns
///
/// A random diagonal matrix with entries in {-1, 1}
fn random_sign_diagonal<R: Rng>(rng: &mut R) -> Matrix2f {
    let d1 = if rng.gen::<bool>() { 1.0 } else { -1.0 };
    let d2 = if rng.gen::<bool>() { 1.0 } else { -1.0 };
    Matrix2::new(d1, 0.0, 0.0, d2)
}

/// Create a random SigmaFactorIFS
///
/// # Arguments
///
/// * `rng` - Random number generator
///
/// # Returns
///
/// A random SigmaFactorIFS
pub fn rand_sigma_factor_ifs<R: Rng>(rng: &mut R) -> SigmaFactorIFS {
    // Number of transformations (2, 3, or 4)
    let n = rng.gen_range(2..=4);

    // Sigma factor bounds
    let alpha_lower = 0.5 * (5.0 + n as f64);
    let alpha_upper = 0.5 * (6.0 + n as f64);
    let sigma_factor = uniform(rng, alpha_lower, alpha_upper);

    // Sample singular values
    let singular_values = sample_svs(rng, sigma_factor, n);

    // Create transformations
    let mut transforms = Vec::with_capacity(n);
    for (sigma1, sigma2) in singular_values {
        // Create rotation matrices
        let r_theta = random_rotation(rng);
        let r_phi = random_rotation(rng);

        // Create diagonal matrix with singular values
        let sigma_mat = Matrix2::new(sigma1, 0.0, 0.0, sigma2);

        // Create diagonal matrix with random signs
        let d = random_sign_diagonal(rng);

        // Combine matrices to form W
        let w = r_theta * sigma_mat * r_phi * d;

        // Create random translation vector
        let b1 = uniform(rng, -1.0, 1.0);
        let b2 = uniform(rng, -1.0, 1.0);
        let b = Vector2::new(b1, b2);

        // Create affine transformation
        transforms.push(Affine::new(w, b));
    }

    // Create probability weights based on determinants
    let mut weights: Vec<f64> = transforms.iter().map(|t| t.determinant().abs()).collect();

    // Normalize weights
    let sum: f64 = weights.iter().sum();
    for w in &mut weights {
        *w /= sum;
    }

    SigmaFactorIFS::new(transforms, weights)
}
