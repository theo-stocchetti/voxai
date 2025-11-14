//! Configuration schema definitions
//!
//! Defines all configuration structures with serde support for JSON serialization

use serde::{Deserialize, Serialize};

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Configuration schema version for migration support
    #[serde(default = "default_version")]
    pub version: String,

    /// Audio capture and processing settings
    #[serde(default)]
    pub audio: AudioConfig,

    /// Transcription and Whisper model settings
    #[serde(default)]
    pub transcription: TranscriptionConfig,

    /// Global hotkey configuration
    #[serde(default)]
    pub hotkeys: HotkeyConfig,

    /// UI and visual settings
    #[serde(default)]
    pub ui: UiConfig,
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Input device name ("default" for system default)
    #[serde(default = "default_input_device")]
    pub input_device: String,

    /// Enable noise reduction (RNNoise)
    #[serde(default = "default_true")]
    pub noise_reduction: bool,

    /// Sample rate in Hz (16000 for Whisper compatibility)
    #[serde(default = "default_sample_rate")]
    pub sample_rate: u32,
}

/// Transcription configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionConfig {
    /// Whisper model size: "tiny", "base", "small", "medium"
    #[serde(default = "default_model")]
    pub model: String,

    /// Language code or "auto" for automatic detection
    #[serde(default = "default_language")]
    pub language: String,

    /// Enable GPU acceleration if available
    #[serde(default = "default_true")]
    pub enable_gpu: bool,

    /// Voice Activity Detection aggressiveness (0-3)
    #[serde(default = "default_vad_mode")]
    pub vad_aggressiveness: u8,
}

/// Hotkey configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    /// Hotkey to toggle recording
    #[serde(default = "default_hotkey")]
    pub toggle_recording: String,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Show status overlay during recording
    #[serde(default = "default_true")]
    pub show_overlay: bool,

    /// Enable system notifications
    #[serde(default = "default_true")]
    pub system_notifications: bool,

    /// Automatically capitalize first letter
    #[serde(default = "default_true")]
    pub auto_capitalization: bool,
}

// Default value functions
fn default_version() -> String {
    "1.0.0".to_string()
}

fn default_input_device() -> String {
    "default".to_string()
}

fn default_sample_rate() -> u32 {
    16000
}

fn default_model() -> String {
    "base".to_string()
}

fn default_language() -> String {
    "auto".to_string()
}

fn default_hotkey() -> String {
    #[cfg(target_os = "windows")]
    return "Ctrl+Shift+Space".to_string();
    #[cfg(target_os = "macos")]
    return "Cmd+Shift+Space".to_string();
    #[cfg(target_os = "linux")]
    return "Ctrl+Shift+Space".to_string();
}

fn default_vad_mode() -> u8 {
    1
}

fn default_true() -> bool {
    true
}

// Implement Default trait for all structs
impl Default for Config {
    fn default() -> Self {
        Self {
            version: default_version(),
            audio: AudioConfig::default(),
            transcription: TranscriptionConfig::default(),
            hotkeys: HotkeyConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            input_device: default_input_device(),
            noise_reduction: default_true(),
            sample_rate: default_sample_rate(),
        }
    }
}

impl Default for TranscriptionConfig {
    fn default() -> Self {
        Self {
            model: default_model(),
            language: default_language(),
            enable_gpu: default_true(),
            vad_aggressiveness: default_vad_mode(),
        }
    }
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            toggle_recording: default_hotkey(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_overlay: default_true(),
            system_notifications: default_true(),
            auto_capitalization: default_true(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.audio.sample_rate, 16000);
        assert_eq!(config.transcription.model, "base");
    }

    #[test]
    fn test_serialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"version\":\"1.0.0\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"version":"1.0.0"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.version, "1.0.0");
        // Other fields should use defaults
        assert_eq!(config.audio.sample_rate, 16000);
    }
}
