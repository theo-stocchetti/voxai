//! Configuration management
//!
//! Handles persistent configuration and settings

use serde::{Deserialize, Serialize};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub hotkey: String,
    pub model_size: String,
    pub auto_start: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hotkey: "Ctrl+Shift+Space".to_string(),
            model_size: "base".to_string(),
            auto_start: false,
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> anyhow::Result<Self> {
        // TODO: Implement config loading
        Ok(Self::default())
    }

    /// Save configuration to file
    #[allow(dead_code)]
    pub fn save(&self) -> anyhow::Result<()> {
        // TODO: Implement config saving
        Ok(())
    }
}
