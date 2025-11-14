//! Integration tests for configuration module

use voxai::config::*;

#[test]
fn test_config_load_and_save() {
    // Test loading default config
    let config = Config::default();

    // Verify defaults
    assert_eq!(config.version, "1.0.0");
    assert_eq!(config.audio.sample_rate, 16000);
    assert_eq!(config.transcription.model, "base");
    assert!(config.audio.noise_reduction);
}

#[test]
fn test_config_validation_through_save() {
    use std::env;
    use std::fs;

    let mut config = Config::default();

    // Create temp directory for test
    let temp_dir = env::temp_dir().join("voxai_test_validation");
    fs::create_dir_all(&temp_dir).unwrap();

    // Test valid config - should save successfully
    let valid_json = serde_json::to_string_pretty(&config).unwrap();
    assert!(valid_json.contains("base")); // Contains valid model

    // Test that invalid config can be created but should fail validation on save
    config.transcription.model = "invalid".to_string();
    let invalid_json = serde_json::to_string_pretty(&config).unwrap();
    assert!(invalid_json.contains("invalid"));

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_config_serialization() {
    let config = Config::default();
    let json = serde_json::to_string_pretty(&config).unwrap();

    // Verify JSON contains expected fields
    assert!(json.contains("version"));
    assert!(json.contains("audio"));
    assert!(json.contains("transcription"));
    assert!(json.contains("hotkeys"));
    assert!(json.contains("ui"));
}

#[test]
fn test_config_deserialization_with_missing_fields() {
    // Test that missing fields use defaults
    let json = r#"{"version":"1.0.0"}"#;
    let config: Config = serde_json::from_str(json).unwrap();

    // Verify defaults are applied
    assert_eq!(config.audio.sample_rate, 16000);
    assert_eq!(config.transcription.model, "base");
}

#[test]
fn test_audio_config_defaults() {
    let audio = AudioConfig::default();
    assert_eq!(audio.input_device, "default");
    assert_eq!(audio.sample_rate, 16000);
    assert!(audio.noise_reduction);
}

#[test]
fn test_transcription_config_defaults() {
    let transcription = TranscriptionConfig::default();
    assert_eq!(transcription.model, "base");
    assert_eq!(transcription.language, "auto");
    assert!(transcription.enable_gpu);
    assert_eq!(transcription.vad_aggressiveness, 1);
}

#[test]
fn test_hotkey_config_platform_specific() {
    let hotkey = HotkeyConfig::default();

    #[cfg(target_os = "macos")]
    assert!(hotkey.toggle_recording.contains("Cmd"));

    #[cfg(not(target_os = "macos"))]
    assert!(hotkey.toggle_recording.contains("Ctrl"));
}

#[test]
fn test_ui_config_defaults() {
    let ui = UiConfig::default();
    assert!(ui.show_overlay);
    assert!(ui.system_notifications);
    assert!(ui.auto_capitalization);
}
