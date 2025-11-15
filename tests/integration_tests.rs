//! Integration tests for VoxAI
//!
//! Tests end-to-end workflows and component integration

mod common;

use std::env;
use std::fs;
use voxai::config::{load_config, save_config, Config};

#[test]
fn test_full_config_lifecycle() {
    // Create temp directory
    let temp_dir = env::temp_dir().join("voxai_integration_test");
    fs::create_dir_all(&temp_dir).unwrap();

    // Test 1: Create default config
    let config1 = Config::default();
    assert_eq!(config1.version, "1.0.0");

    // Test 2: Modify config
    let mut config2 = config1.clone();
    config2.transcription.model = "small".to_string();
    config2.audio.noise_reduction = false;

    // Test 3: Serialize to JSON
    let json = serde_json::to_string_pretty(&config2).unwrap();
    assert!(json.contains("small"));
    assert!(json.contains("false")); // noise_reduction = false

    // Test 4: Deserialize from JSON
    let config3: Config = serde_json::from_str(&json).unwrap();
    assert_eq!(config3.transcription.model, "small");
    assert!(!config3.audio.noise_reduction);

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_config_validation_integration() {
    // Test valid configurations
    let valid_models = vec!["tiny", "base", "small", "medium"];

    for model in valid_models {
        let mut config = Config::default();
        config.transcription.model = model.to_string();

        // Should serialize without errors
        let json = serde_json::to_string_pretty(&config).unwrap();
        assert!(json.contains(model));
    }
}

#[test]
fn test_audio_config_integration() {
    let mut config = Config::default();

    // Test audio settings
    config.audio.input_device = "test-device".to_string();
    config.audio.sample_rate = 16000; // Fixed for Whisper

    // Serialize and deserialize
    let json = serde_json::to_string_pretty(&config).unwrap();
    let loaded: Config = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.audio.input_device, "test-device");
    assert_eq!(loaded.audio.sample_rate, 16000);
}

#[test]
fn test_transcription_config_integration() {
    let mut config = Config::default();

    // Test transcription settings
    config.transcription.model = "medium".to_string();
    config.transcription.language = "en".to_string();
    config.transcription.vad_aggressiveness = 2;

    // Serialize and deserialize
    let json = serde_json::to_string_pretty(&config).unwrap();
    let loaded: Config = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.transcription.model, "medium");
    assert_eq!(loaded.transcription.language, "en");
    assert_eq!(loaded.transcription.vad_aggressiveness, 2);
}

#[test]
fn test_hotkey_config_integration() {
    let mut config = Config::default();

    // Test hotkey settings
    config.hotkeys.toggle_recording = "Ctrl+Alt+R".to_string();

    // Serialize and deserialize
    let json = serde_json::to_string_pretty(&config).unwrap();
    let loaded: Config = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.hotkeys.toggle_recording, "Ctrl+Alt+R");
}

#[test]
fn test_ui_config_integration() {
    let mut config = Config::default();

    // Test UI settings
    config.ui.show_overlay = false;
    config.ui.system_notifications = true;
    config.ui.auto_capitalization = false;

    // Serialize and deserialize
    let json = serde_json::to_string_pretty(&config).unwrap();
    let loaded: Config = serde_json::from_str(&json).unwrap();

    assert!(!loaded.ui.show_overlay);
    assert!(loaded.ui.system_notifications);
    assert!(!loaded.ui.auto_capitalization);
}

#[test]
fn test_mock_audio_processing() {
    // Use common test utilities
    let sample = common::mock_audio_sample(1);

    // Verify sample properties
    assert_eq!(sample.len(), 16000); // 1 second at 16kHz

    // Simple audio processing simulation
    let max_amplitude = sample.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
    assert!(max_amplitude > 0.0 && max_amplitude <= 1.0);
}

#[test]
fn test_version_migration() {
    // Test that future versions can load old configs
    let old_config_json = r#"{
        "version": "0.9.0",
        "audio": {"input_device": "old-device", "noise_reduction": true, "sample_rate": 16000},
        "transcription": {"model": "base", "language": "auto", "enable_gpu": true, "vad_aggressiveness": 1},
        "hotkeys": {"toggle_recording": "Ctrl+Space"},
        "ui": {"show_overlay": true, "system_notifications": true, "auto_capitalization": true}
    }"#;

    // Should still deserialize (with version updated)
    let config: Config = serde_json::from_str(old_config_json).unwrap();
    assert_eq!(config.audio.input_device, "old-device");
}
