//! Error types for the rust-random-logo library

use thiserror::Error;
use std::io;

/// Error type for the rust-random-logo library
#[derive(Error, Debug)]
pub enum Error {
    /// Error when loading a configuration file
    #[error("Failed to load configuration: {0}")]
    ConfigError(String),

    /// Error when parsing TOML
    #[error("Failed to parse TOML: {0}")]
    TomlDeError(#[from] toml::de::Error),

    /// Error when serializing TOML
    #[error("Failed to serialize TOML: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    /// Error when performing I/O operations
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    /// Error when rendering an image
    #[error("Failed to render image: {0}")]
    RenderError(String),
}

/// Result type for the rust-random-logo library
pub type Result<T> = std::result::Result<T, Error>;
