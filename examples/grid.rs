//! Grid example of using the rust-random-logo library
//!
//! This example generates 25 different fractal images with different seeds
//! and arranges them in a 5x5 grid, saving the result as a single large image.

use std::path::PathBuf;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use image::ImageBuffer;

use rust_random_logo::{Config, render, rand_sigma_factor_ifs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration for each fractal
    let base_config = Config {
        height: 200,
        width: 200,
        npoints: 50_000,
        ifs_name: "SigmaFactorIFS".to_string(),
        ndims: 2,
        rng_name: "Xoshiro256PlusPlus".to_string(),
        seed: 99, // This will be overridden for each fractal
    };

    // Number of rows and columns in the grid
    let grid_rows = 5;
    let grid_cols = 5;

    // Create a large image to hold the grid
    let grid_width = base_config.width * grid_cols;
    let grid_height = base_config.height * grid_rows;
    let mut grid_image = ImageBuffer::new(grid_width as u32, grid_height as u32);

    println!("Generating {} fractals for a {}x{} grid...", grid_rows * grid_cols, grid_rows, grid_cols);

    // Generate fractals and place them in the grid
    for row in 0..grid_rows {
        for col in 0..grid_cols {
            // Calculate the index and seed for this fractal
            let index = row * grid_cols + col;
            let seed = 100 + index as u64; // Different seed for each fractal

            // Create a configuration for this fractal
            let mut config = base_config.clone();
            config.seed = seed;

            // Create a random number generator with the seed
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

            // Create a random IFS
            let ifs = rand_sigma_factor_ifs(&mut rng);

            // Render the fractal
            println!("  Rendering fractal {}/{} with seed {}...", index + 1, grid_rows * grid_cols, seed);
            let fractal = render(rng, &ifs, &config);

            // Calculate the position in the grid
            let x_offset = col * base_config.width;
            let y_offset = row * base_config.height;

            // Copy the fractal to the grid
            for (x, y, pixel) in fractal.enumerate_pixels() {
                let grid_x = x_offset as u32 + x;
                let grid_y = y_offset as u32 + y;
                grid_image.put_pixel(grid_x, grid_y, *pixel);
            }
        }
    }

    // Save the grid image
    let output_path = PathBuf::from("fractal_grid.png");
    println!("Saving grid image to {}...", output_path.display());
    grid_image.save(&output_path)?;

    println!("Done!");
    Ok(())
}
