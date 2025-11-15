//! Configuration management
//!
//! Handles persistent configuration and settings with JSON serialization.
//! Configuration is stored in platform-specific directories:
//! - Windows: %APPDATA%\VoxAI\config.json
//! - macOS: ~/Library/Application Support/VoxAI\config.json
//! - Linux: ~/.config/voxai/config.json

pub mod language;
mod schema;

pub use language::Language;
pub use schema::*;

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Get the platform-specific configuration directory path
pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Failed to get config directory")?
        .join("voxai");

    // Ensure directory exists
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
    }

    Ok(config_dir)
}

/// Get the full path to the configuration file
pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.json"))
}

/// Load configuration from file, or create default if not exists
pub fn load_config() -> Result<Config> {
    let path = get_config_path()?;

    if !path.exists() {
        // Create default config on first run
        log::info!("Config file not found, creating default configuration");
        let default_config = Config::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }

    // Read config file
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    // Parse JSON
    let config: Config =
        serde_json::from_str(&contents).context("Failed to parse config file (invalid JSON)")?;

    // Validate
    validate_config(&config).context("Config validation failed")?;

    log::info!("Configuration loaded successfully from: {}", path.display());
    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()> {
    let path = get_config_path()?;

    // Validate before saving
    validate_config(config).context("Cannot save invalid configuration")?;

    // Serialize to pretty JSON
    let json = serde_json::to_string_pretty(config).context("Failed to serialize config")?;

    // Write to file
    std::fs::write(&path, json)
        .with_context(|| format!("Failed to write config file: {}", path.display()))?;

    log::info!("Configuration saved to: {}", path.display());
    Ok(())
}

/// Validate configuration values
fn validate_config(config: &Config) -> Result<()> {
    // Validate model name
    let valid_models = ["tiny", "base", "small", "medium"];
    anyhow::ensure!(
        valid_models.contains(&config.transcription.model.as_str()),
        "Invalid model '{}'. Valid models: {:?}",
        config.transcription.model,
        valid_models
    );

    // Validate sample rate (Whisper requires 16kHz)
    anyhow::ensure!(
        config.audio.sample_rate == 16000,
        "Invalid sample rate {}. Only 16000 Hz is supported",
        config.audio.sample_rate
    );

    // Validate VAD mode (0-3)
    anyhow::ensure!(
        config.transcription.vad_aggressiveness <= 3,
        "Invalid VAD aggressiveness {}. Must be 0-3",
        config.transcription.vad_aggressiveness
    );

    // Validate hotkey is not empty
    anyhow::ensure!(
        !config.hotkeys.toggle_recording.is_empty(),
        "Hotkey cannot be empty"
    );

    Ok(())
}

/// Reset configuration to defaults
#[allow(dead_code)]
pub fn reset_config() -> Result<()> {
    let default_config = Config::default();
    save_config(&default_config)?;
    log::info!("Configuration reset to defaults");
    Ok(())
}

/// Check if configuration file exists
#[allow(dead_code)]
pub fn config_exists() -> bool {
    get_config_path().map(|p| p.exists()).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_path() {
        let path = get_config_path().unwrap();
        assert!(path.to_str().unwrap().contains("voxai"));
        assert!(path.to_str().unwrap().ends_with("config.json"));
    }

    #[test]
    fn test_validate_config_valid() {
        let config = Config::default();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_invalid_model() {
        let mut config = Config::default();
        config.transcription.model = "invalid".to_string();
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_config_invalid_sample_rate() {
        let mut config = Config::default();
        config.audio.sample_rate = 48000;
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_config_invalid_vad() {
        let mut config = Config::default();
        config.transcription.vad_aggressiveness = 5;
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_save_and_load_config() {
        // This test creates a real config file in a temp directory
        // Use a custom config dir for testing
        let temp_dir = std::env::temp_dir().join("voxai_test");
        std::fs::create_dir_all(&temp_dir).unwrap();

        // Note: This test is limited as we can't easily override config_dir
        // In production, we'd use dependency injection or env vars for testing
        let config = Config::default();
        let json = serde_json::to_string_pretty(&config).unwrap();

        // Just verify serialization/deserialization works
        let loaded: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.version, config.version);
        assert_eq!(loaded.audio.sample_rate, config.audio.sample_rate);
    }
}
