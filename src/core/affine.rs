//! Affine transformation implementation
//!
//! This module provides the Affine transformation struct and related functions.
//! An affine transformation is defined as f(x) = Wx + b, where W is a matrix and b is a vector.

// No imports needed here
use crate::core::types::{Matrix2f, Vector2f};

/// Affine transformation struct
///
/// Represents an affine transformation f(x) = Wx + b
#[derive(Debug, Clone)]
pub struct Affine {
    /// The linear transformation matrix
    pub w: Matrix2f,

    /// The translation vector
    pub b: Vector2f,
}

impl Affine {
    /// Create a new affine transformation
    ///
    /// # Arguments
    ///
    /// * `w` - The linear transformation matrix
    /// * `b` - The translation vector
    ///
    /// # Returns
    ///
    /// A new Affine transformation
    pub fn new(w: Matrix2f, b: Vector2f) -> Self {
        Self { w, b }
    }

    /// Apply the affine transformation to a point
    ///
    /// # Arguments
    ///
    /// * `point` - The point to transform
    ///
    /// # Returns
    ///
    /// The transformed point
    pub fn apply(&self, point: &Vector2f) -> Vector2f {
        self.w * point + self.b
    }

    /// Get the determinant of the transformation matrix
    ///
    /// # Returns
    ///
    /// The determinant of the transformation matrix
    pub fn determinant(&self) -> f64 {
        self.w.determinant()
    }
}

// Implement function-like behavior for Affine
// This allows using an Affine instance like a function: affine(&point)
impl Affine {
    /// Call the affine transformation as a function
    ///
    /// # Arguments
    ///
    /// * `point` - The point to transform
    ///
    /// # Returns
    ///
    /// The transformed point
    pub fn call(&self, point: &Vector2f) -> Vector2f {
        self.apply(point)
    }
}
