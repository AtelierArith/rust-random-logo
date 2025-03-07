//! # Rust Random Logo
//!
//! A Rust implementation of the RandomLogos fractal generation library, based on the
//! [Improving Fractal Pre-training](http://catalys1.github.io/fractal-pretraining/) approach.
//!
//! This library generates fractal-like images using Iterated Function Systems (IFS)
//! with a sigma-factor approach.
//!
//! ## Example
//!
//! ```rust,no_run
//! use rust_random_logo::{Config, render_from_config};
//! use std::path::PathBuf;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load configuration from a TOML file
//!     let config = Config::from_file("examples/config.toml")?;
//!
//!     // Generate the image
//!     let image = render_from_config(&config)?;
//!
//!     // Save the image
//!     let output_path = PathBuf::from("fractal.png");
//!     image.save(&output_path)?;
//!
//!     Ok(())
//! }
//! ```

pub mod core;
pub mod error;

// Re-export commonly used items
pub use core::affine::Affine;
pub use core::config::Config;
pub use core::ifs::{rand_sigma_factor_ifs, sample_svs, SigmaFactorIFS};
pub use core::renderer::{generate_points, render, render_from_config};
pub use error::{Error, Result};
