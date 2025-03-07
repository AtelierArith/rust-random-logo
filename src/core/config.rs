//! Configuration handling for the rust-random-logo library
//!
//! This module provides the Config struct and related functions for loading
//! configuration from TOML files.

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use toml;

use crate::error::Result;

/// Configuration for generating fractal images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Height of the output image
    pub height: usize,

    /// Width of the output image
    pub width: usize,

    /// Number of points to generate
    pub npoints: usize,

    /// Name of the IFS to use
    pub ifs_name: String,

    /// Dimension of the IFS
    pub ndims: usize,

    /// Name of the random number generator to use
    pub rng_name: String,

    /// Seed for the random number generator
    pub seed: u64,
}

impl Config {
    /// Create a new Config with default values
    pub fn new() -> Self {
        Self {
            height: 384,
            width: 384,
            npoints: 100_000,
            ifs_name: "SigmaFactorIFS".to_string(),
            ndims: 2,
            rng_name: "Xoshiro256PlusPlus".to_string(),
            seed: 42,
        }
    }

    /// Load configuration from a TOML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML file
    ///
    /// # Returns
    ///
    /// A Result containing the Config if successful, or an Error if not
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML file
    ///
    /// # Returns
    ///
    /// A Result containing () if successful, or an Error if not
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
