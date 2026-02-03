use thiserror::Error;

/// Errors that can occur when working with TAPF files
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse TOML: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Failed to serialize TOML: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Animation has no frames")]
    NoFrames,

    #[error("Invalid dimensions: width={0}, height={1}")]
    InvalidDimensions(u16, u16),

    #[error("Invalid FPS: {0} (must be between 1 and 120)")]
    InvalidFps(u32),
}

/// Result type alias for TAPF operations
pub type Result<T> = std::result::Result<T, Error>;