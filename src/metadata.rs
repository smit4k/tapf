use serde::{Deserialize, Serialize};

/// Metadata about an animation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    /// Name of the animation
    #[serde(default = "default_name")]
    pub name: String,

    /// Author/creator of the animation
    #[serde(default = "default_author")]
    pub author: String,

    /// Format version (defaults to "1.0")
    #[serde(default = "default_version")]
    pub version: String,

    /// Creation date (ISO 8601 format recommended)
    #[serde(default = "default_created")]
    pub created: String,

    /// Width of the animation in characters
    #[serde(default = "default_width")]
    pub width: u16,

    /// Height of the animation in characters
    #[serde(default = "default_height")]
    pub height: u16,

    /// Optional description of the animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

fn default_version() -> String {
    "1.0".to_string()
}

fn default_width() -> u16 {
    40
}

fn default_height() -> u16 {
    10
}

fn default_author() -> String {
    "tapf".to_string()
}

fn default_name() -> String {
    "My Animation".to_string()
}

fn default_created() -> String {
    Metadata::current_timestamp()
}

impl Metadata {
    /// Create a new Metadata instance
    pub fn new(
        name: impl Into<String>,
        author: impl Into<String>,
        width: u16,
        height: u16,
    ) -> Self {
        Self {
            name: name.into(),
            author: author.into(),
            version: default_version(),
            created: Self::current_timestamp(),
            width,
            height,
            description: None,
        }
    }

    /// Get current timestamp
    #[cfg(feature = "timestamps")]
    fn current_timestamp() -> String {
        chrono::Utc::now().to_rfc3339()
    }

    #[cfg(not(feature = "timestamps"))]
    fn current_timestamp() -> String {
        "2026-01-01T00:00:00Z".to_string()
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the creation date
    pub fn with_created(mut self, created: impl Into<String>) -> Self {
        self.created = created.into();
        self
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            name: default_name(),
            author: default_author(),
            version: default_version(),
            created: default_created(),
            width: default_width(),
            height: default_height(),
            description: None,
        }
    }
}
