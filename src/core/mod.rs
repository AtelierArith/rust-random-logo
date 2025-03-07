// Core module for the rust-random-logo library
// This module contains the core components for generating fractal images

pub mod affine;
pub mod ifs;
pub mod types;
pub mod renderer;
pub mod config;
pub mod utils;

// Re-export commonly used items
pub use affine::Affine;
pub use ifs::{SigmaFactorIFS, sample_svs, rand_sigma_factor_ifs};
pub use types::*;
pub use renderer::{generate_points, render};
pub use config::Config;
