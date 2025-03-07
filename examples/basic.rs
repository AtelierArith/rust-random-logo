//! Basic example of using the rust-random-logo library

use std::path::PathBuf;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use rust_random_logo::{Config, render, rand_sigma_factor_ifs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a random number generator with a seed
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(99);

    // Create a random IFS
    let ifs = rand_sigma_factor_ifs(&mut rng);

    // Create a configuration
    let config = Config {
        height: 384,
        width: 384,
        npoints: 100_000,
        ifs_name: "SigmaFactorIFS".to_string(),
        ndims: 2,
        rng_name: "Xoshiro256PlusPlus".to_string(),
        seed: 99,
    };

    // Render the image
    println!("Rendering fractal with {} points...", config.npoints);
    let image = render(rng, &ifs, &config);

    // Save the image
    let output_path = PathBuf::from("fractal_basic.png");
    println!("Saving image to {}...", output_path.display());
    image.save(&output_path)?;

    println!("Done!");
    Ok(())
}
