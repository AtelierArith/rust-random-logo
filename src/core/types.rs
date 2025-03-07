//! Common types used throughout the library

use nalgebra::{Matrix2, Vector2};

/// Type alias for a 2D matrix
pub type Matrix2f = Matrix2<f64>;

/// Type alias for a 2D vector
pub type Vector2f = Vector2<f64>;

/// Trait for Iterated Function Systems
pub trait IFS {
    /// The dimension of the IFS
    const DIM: usize;

    /// The type of the IFS
    type Scalar: nalgebra::RealField;

    /// Apply a random transformation to a point
    fn apply_random<R: rand::Rng>(&self, rng: &mut R, point: &Vector2f) -> Vector2f;
}
