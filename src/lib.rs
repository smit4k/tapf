//! # TAPF - Terminal Animation Pet Format
//!
//! A TOML-based format for defining terminal animations.
//!
//! ## Overview
//!
//! TAPF provides a simple, human-readable way to define ASCII/Unicode animations
//! for terminal applications. Each animation is stored as a TOML file containing
//! metadata and frame data.
//!
//! ## Example
//!
//! ```toml
//! [metadata]
//! name = "Idle Blink"
//! author = "username"
//! version = "1.0"
//! created = "2026-02-02T12:00:00Z"
//! width = 15
//! height = 8
//!
//! [animation]
//! fps = 10
//! loop = true
//!
//! [[animation.frames]]
//! data = """
//!     /\_/\
//!    ( o.o )
//!     > ^ <
//! """
//!
//! [[animation.frames]]
//! data = """
//!     /\_/\
//!    ( -.- )
//!     > ^ <
//! """
//! ```
//!
//! ## Usage
//!
//! ```rust,no_run
//! use tapf::AnimationFile;
//!
//! # fn main() -> tapf::Result<()> {
//! // Load an animation
//! let anim = AnimationFile::load("idle.toml")?;
//!
//! // Validate it
//! anim.validate()?;
//!
//! // Access frames
//! for (i, frame) in anim.animation.frames.iter().enumerate() {
//!     println!("Frame {}: \n{}", i, frame.data);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Creating Animations Programmatically
//!
//! ```rust
//! use tapf::{AnimationFile, Animation, Frame, Metadata};
//!
//! # fn main() -> tapf::Result<()> {
//! let metadata = Metadata::new("Test Animation", "Author", 10, 5)
//!     .with_description("A simple test animation");
//!
//! let animation = Animation::new(10)
//!     .with_loop(true)
//!     .add_frame(Frame::new("Frame 1 data"))
//!     .add_frame(Frame::new("Frame 2 data"));
//!
//! let anim_file = AnimationFile::new(metadata, animation);
//! anim_file.save("output.toml")?;
//! # Ok(())
//! # }
//! ```

mod animation;
mod error;
mod metadata;

pub use animation::{Animation, AnimationFile, Frame};
pub use error::{Error, Result};
pub use metadata::Metadata;

// Re-export serde for convenience
pub use serde::{Deserialize, Serialize};