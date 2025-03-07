//! Renderer implementation for the rust-random-logo library
//!
//! This module provides functions for generating points and rendering images
//! based on Iterated Function Systems.

use image::{ImageBuffer, RgbImage};
use rand::Rng;

use crate::core::config::Config;
use crate::core::ifs::SigmaFactorIFS;
use crate::core::types::{Vector2f, IFS};
use crate::core::utils::random_julia_color;
use crate::error::{Error, Result};

/// Generate points using an Iterated Function System
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `ifs` - The Iterated Function System
/// * `n` - Number of points to generate
/// * `height` - Height of the output space
/// * `width` - Width of the output space
///
/// # Returns
///
/// A tuple of vectors containing the x and y coordinates of the generated points
pub fn generate_points<R: Rng + Clone>(
    rng: &mut R,
    ifs: &SigmaFactorIFS,
    n: usize,
    height: usize,
    width: usize,
) -> (Vec<f64>, Vec<f64>) {
    // Initialize point
    let mut point = Vector2f::zeros();

    // Generate points
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);

    for _ in 0..n {
        // Apply a random transformation
        point = ifs.apply_random(rng, &point);

        // Store the point
        xs.push(point.x);
        ys.push(point.y);
    }

    // Normalize points to fit within the output space
    normalize_points(&mut xs, &mut ys, height, width);

    (xs, ys)
}

/// Normalize points to fit within the output space
///
/// # Arguments
///
/// * `xs` - X coordinates
/// * `ys` - Y coordinates
/// * `height` - Height of the output space
/// * `width` - Width of the output space
fn normalize_points(xs: &mut Vec<f64>, ys: &mut Vec<f64>, height: usize, width: usize) {
    // Find min and max values
    let x_min = xs.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let x_max = xs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = ys.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let y_max = ys.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // To prevent bounds errors when drawing points on a canvas,
    // an offset value of 5 is used.
    let offset = 5.0;
    let width_range = (width as f64 - offset) - offset;
    let height_range = (height as f64 - offset) - offset;

    // Normalize points
    for x in xs.iter_mut() {
        *x = width_range * (*x - x_min) / (x_max - x_min) + offset;
    }

    for y in ys.iter_mut() {
        *y = height_range * (*y - y_min) / (y_max - y_min) + offset;
    }
}

/// Render an image using an Iterated Function System
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `ifs` - The Iterated Function System
/// * `config` - Configuration for rendering
///
/// # Returns
///
/// An RGB image
pub fn render<R: Rng + Clone>(
    mut rng: R,
    ifs: &SigmaFactorIFS,
    config: &Config,
) -> RgbImage {
    let height = config.height;
    let width = config.width;
    let npoints = config.npoints;

    // Generate points
    let (xs, ys) = generate_points(&mut rng, ifs, npoints, height, width);

    // Create image
    let mut image = ImageBuffer::new(width as u32, height as u32);

    // Draw points
    let color = random_julia_color(&mut rng);
    for (x, y) in xs.iter().zip(ys.iter()) {
        let x = x.trunc() as u32;
        let y = y.trunc() as u32;

        // Check bounds
        if x < width as u32 && y < height as u32 {
            image.put_pixel(x, y, color);
        }
    }

    image
}

/// Render an image using a configuration file
///
/// # Arguments
///
/// * `config` - Configuration for rendering
///
/// # Returns
///
/// A Result containing an RGB image if successful, or an Error if not
pub fn render_from_config(config: &Config) -> Result<RgbImage> {
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    // Validate IFS configuration
    if config.ifs_name != "SigmaFactorIFS" {
        return Err(Error::ConfigError(format!("Unknown IFS: {}", config.ifs_name)));
    }

    if config.ndims != 2 {
        return Err(Error::ConfigError(format!("Unsupported dimension: {}", config.ndims)));
    }

    // Create RNG
    let mut rng = match config.rng_name.as_str() {
        "Xoshiro256PlusPlus" => Xoshiro256PlusPlus::seed_from_u64(config.seed),
        _ => return Err(Error::ConfigError(format!("Unknown RNG: {}", config.rng_name))),
    };

    // Create IFS
    let ifs = crate::core::ifs::rand_sigma_factor_ifs(&mut rng);

    // Render image
    Ok(render(rng, &ifs, config))
}
