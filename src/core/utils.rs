//! Utility functions for the rust-random-logo library

use image::{Rgb, Rgba};
use rand::Rng;

/// Generate a random number in the range [a, b]
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `a` - Lower bound
/// * `b` - Upper bound
///
/// # Returns
///
/// A random number in the range [a, b]
pub fn uniform<R: Rng>(rng: &mut R, a: f64, b: f64) -> f64 {
    a + (b - a) * rng.gen::<f64>()
}

/// Julia colors from the original Julia logo
pub const JULIA_RED: Rgb<u8> = Rgb([203, 60, 51]);
pub const JULIA_GREEN: Rgb<u8> = Rgb([56, 152, 38]);
pub const JULIA_BLUE: Rgb<u8> = Rgb([64, 99, 216]);
pub const JULIA_PURPLE: Rgb<u8> = Rgb([149, 88, 178]);

/// Julia colors with alpha channel
pub const JULIA_RED_ALPHA: Rgba<u8> = Rgba([203, 60, 51, 255]);
pub const JULIA_GREEN_ALPHA: Rgba<u8> = Rgba([56, 152, 38, 255]);
pub const JULIA_BLUE_ALPHA: Rgba<u8> = Rgba([64, 99, 216, 255]);
pub const JULIA_PURPLE_ALPHA: Rgba<u8> = Rgba([149, 88, 178, 255]);

/// Get a random Julia color
///
/// # Arguments
///
/// * `rng` - Random number generator
///
/// # Returns
///
/// A random Julia color
pub fn random_julia_color<R: Rng>(rng: &mut R) -> Rgb<u8> {
    match rng.gen_range(0..4) {
        0 => JULIA_RED,
        1 => JULIA_GREEN,
        2 => JULIA_BLUE,
        _ => JULIA_PURPLE,
    }
}

/// Get a random Julia color with alpha channel
///
/// # Arguments
///
/// * `rng` - Random number generator
///
/// # Returns
///
/// A random Julia color with alpha channel
pub fn random_julia_color_alpha<R: Rng>(rng: &mut R) -> Rgba<u8> {
    match rng.gen_range(0..4) {
        0 => JULIA_RED_ALPHA,
        1 => JULIA_GREEN_ALPHA,
        2 => JULIA_BLUE_ALPHA,
        _ => JULIA_PURPLE_ALPHA,
    }
}
