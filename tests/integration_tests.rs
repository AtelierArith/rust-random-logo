//! Integration tests for the rust-random-logo library

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use rust_random_logo::{Config, render, render_from_config, rand_sigma_factor_ifs};

#[test]
fn test_render_with_config() {
    // Create a configuration
    let config = Config {
        height: 100,
        width: 100,
        npoints: 1000,
        ifs_name: "SigmaFactorIFS".to_string(),
        ndims: 2,
        rng_name: "Xoshiro256PlusPlus".to_string(),
        seed: 42,
    };

    // Create a random number generator with a seed
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(config.seed);

    // Create a random IFS
    let ifs = rand_sigma_factor_ifs(&mut rng);

    // Render the image
    let image = render(rng, &ifs, &config);

    // Check that the image has the correct dimensions
    assert_eq!(image.width(), config.width as u32);
    assert_eq!(image.height(), config.height as u32);
}

#[test]
fn test_generate_ifs() {
    // Create a random number generator with a seed
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    // Create a random IFS
    let ifs = rand_sigma_factor_ifs(&mut rng);

    // Check that the IFS has at least one transformation
    assert!(!ifs.transforms.is_empty());

    // Check that the weights sum to approximately 1.0
    let sum: f64 = ifs.weights.iter().sum();
    assert!((sum - 1.0).abs() < 1e-10);
}

#[test]
fn test_config_serialization() {
    // Create a configuration
    let config = Config {
        height: 100,
        width: 100,
        npoints: 1000,
        ifs_name: "SigmaFactorIFS".to_string(),
        ndims: 2,
        rng_name: "Xoshiro256PlusPlus".to_string(),
        seed: 42,
    };

    // Serialize to TOML
    let toml = toml::to_string_pretty(&config).unwrap();

    // Deserialize from TOML
    let deserialized: Config = toml::from_str(&toml).unwrap();

    // Check that the deserialized config matches the original
    assert_eq!(deserialized.height, config.height);
    assert_eq!(deserialized.width, config.width);
    assert_eq!(deserialized.npoints, config.npoints);
    assert_eq!(deserialized.ifs_name, config.ifs_name);
    assert_eq!(deserialized.ndims, config.ndims);
    assert_eq!(deserialized.rng_name, config.rng_name);
    assert_eq!(deserialized.seed, config.seed);
}

/// Test that the images generated by the basic example and the main program are identical
#[test]
fn test_basic_example_and_main_program_consistency() {
    // Create a configuration with the default seed (99)
    let config = Config {
        height: 100,
        width: 100,
        npoints: 1000,
        ifs_name: "SigmaFactorIFS".to_string(),
        ndims: 2,
        rng_name: "Xoshiro256PlusPlus".to_string(),
        seed: 99,
    };

    // Method 1: Generate image using the basic example approach
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(config.seed);
    let ifs = rand_sigma_factor_ifs(&mut rng);
    let image1 = render(rng, &ifs, &config);

    // Method 2: Generate image using the main program approach
    let image2 = render_from_config(&config).unwrap();

    // Check that the images have the same dimensions
    assert_eq!(image1.width(), image2.width());
    assert_eq!(image1.height(), image2.height());

    // Check that all pixels are identical
    for x in 0..image1.width() {
        for y in 0..image1.height() {
            assert_eq!(image1.get_pixel(x, y), image2.get_pixel(x, y));
        }
    }
}
