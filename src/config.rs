use serde::{Deserialize, Serialize};

/// Configuration options for the formatting.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    /// The number of spaces to indent by.
    pub indent: usize,
    /// Whether to manipulate spacing.
    pub spacing: bool,
    /// Whether to format items over multiple lines.
    pub multiline: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            indent: 2,
            spacing: true,
            multiline: true,
        }
    }
}

impl Config {
    /// Get a config that doesn't change the input.
    ///
    /// This is primarily used for fuzzing and testing.
    pub fn no_changes() -> Self {
        Config {
            indent: 0,
            spacing: false,
            multiline: false,
        }
    }
}
