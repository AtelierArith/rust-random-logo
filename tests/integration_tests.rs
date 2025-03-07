//! Integration tests for the rust-random-logo library

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use rust_random_logo::{Config, render, rand_sigma_factor_ifs};

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
