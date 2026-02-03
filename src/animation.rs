use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::{Error, Metadata, Result};

/// A complete animation file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimationFile {
    /// Metadata about the animation
    pub metadata: Metadata,

    /// The animation data
    pub animation: Animation,
}

/// Animation data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Animation {
    /// Frames per second
    pub fps: u32,

    /// Whether to loop the animation
    #[serde(default = "default_loop", rename = "loop")]
    pub loop_animation: bool,

    /// The frames of the animation
    pub frames: Vec<Frame>,
}

/// A single frame of animation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Frame {
    /// Optional duration override in milliseconds
    /// If not specified, uses the animation's FPS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,

    /// The ASCII art data for this frame
    pub data: String,
}

fn default_loop() -> bool {
    true
}

impl AnimationFile {
    /// Create a new animation file
    pub fn new(metadata: Metadata, animation: Animation) -> Self {
        Self {
            metadata,
            animation,
        }
    }

    /// Load an animation from a TOML file
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_toml_str(&content)
    }

    /// Save an animation to a TOML file
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let content = self.to_toml_string()?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Convert to a pretty-printed TOML string
    pub fn to_toml_string(&self) -> Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }

    /// Parse from a TOML string
    pub fn from_toml_str(s: &str) -> Result<Self> {
        Ok(toml::from_str(s)?)
    }

    /// Validate the animation
    pub fn validate(&self) -> Result<()> {
        // Check for frames
        if self.animation.frames.is_empty() {
            return Err(Error::NoFrames);
        }

        // Check dimensions
        if self.metadata.width == 0 || self.metadata.height == 0 {
            return Err(Error::InvalidDimensions(
                self.metadata.width,
                self.metadata.height,
            ));
        }

        // Check FPS
        if self.animation.fps == 0 || self.animation.fps > 120 {
            return Err(Error::InvalidFps(self.animation.fps));
        }

        Ok(())
    }

    /// Get the duration of a specific frame in milliseconds
    pub fn frame_duration(&self, frame_index: usize) -> u64 {
        self.animation
            .frames
            .get(frame_index)
            .and_then(|f| f.duration)
            .unwrap_or_else(|| 1000 / self.animation.fps as u64)
    }

    /// Get the total number of frames
    pub fn frame_count(&self) -> usize {
        self.animation.frames.len()
    }

    /// Check if the animation should loop
    pub fn should_loop(&self) -> bool {
        self.animation.loop_animation
    }
}

impl Animation {
    /// Create a new animation
    pub fn new(fps: u32) -> Self {
        Self {
            fps,
            loop_animation: true,
            frames: Vec::new(),
        }
    }

    /// Set whether the animation should loop
    pub fn with_loop(mut self, should_loop: bool) -> Self {
        self.loop_animation = should_loop;
        self
    }

    /// Add a frame to the animation
    pub fn add_frame(mut self, frame: Frame) -> Self {
        self.frames.push(frame);
        self
    }

    /// Add multiple frames to the animation
    pub fn add_frames(mut self, frames: impl IntoIterator<Item = Frame>) -> Self {
        self.frames.extend(frames);
        self
    }
}

impl Frame {
    /// Create a new frame with ASCII art data
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            duration: None,
            data: data.into(),
        }
    }

    /// Create a frame with a custom duration
    pub fn with_duration(data: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            duration: Some(duration_ms),
            data: data.into(),
        }
    }
}