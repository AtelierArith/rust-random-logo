// Core module for the rust-random-logo library
// This module contains the core components for generating fractal images

pub mod affine;
pub mod config;
pub mod ifs;
pub mod renderer;
pub mod types;
pub mod utils;

// Re-export commonly used items
pub use affine::Affine;
pub use config::Config;
pub use ifs::{rand_sigma_factor_ifs, sample_svs, SigmaFactorIFS};
pub use renderer::{generate_points, render};
pub use types::*;
