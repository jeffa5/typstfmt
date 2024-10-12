use serde::{Deserialize, Serialize};

/// Configuration options for the formatting.
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    /// The number of spaces to indent by.
    pub indent: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config { indent: 2 }
    }
}

impl Config {
    /// Get a config that doesn't change the input.
    ///
    /// This is primarily used for fuzzing and testing.
    pub fn no_changes() -> Self {
        Config { indent: 0 }
    }
}
