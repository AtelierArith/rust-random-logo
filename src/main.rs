//! Command-line interface for the rust-random-logo library

use std::env;
use std::path::PathBuf;
use std::process;

use rust_random_logo::{Config, render_from_config};

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <config_file> [output_file]", args[0]);
        process::exit(1);
    }

    // Load configuration
    let config_path = &args[1];
    let config = match Config::from_file(config_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error loading configuration: {}", err);
            process::exit(1);
        }
    };

    // Determine output path
    let output_path = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        let mut path = PathBuf::from("fractal.png");
        if let Some(parent) = PathBuf::from(config_path).parent() {
            path = parent.join(path);
        }
        path
    };

    // Render image
    println!("Rendering fractal with {} points...", config.npoints);
    let image = match render_from_config(&config) {
        Ok(image) => image,
        Err(err) => {
            eprintln!("Error rendering image: {}", err);
            process::exit(1);
        }
    };

    // Save image
    println!("Saving image to {}...", output_path.display());
    if let Err(err) = image.save(&output_path) {
        eprintln!("Error saving image: {}", err);
        process::exit(1);
    }

    println!("Done!");
}
