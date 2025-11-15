//! VoxAI - Real-time Audio Transcription Library
//!
//! This is the library crate for VoxAI, exposing all public APIs for testing and potential reuse.

// Public modules
pub mod audio;
pub mod config;
pub mod gpu;
pub mod hotkeys;
pub mod output;
pub mod transcription;
pub mod ui;

// Re-export commonly used types for convenience
pub use config::{AudioConfig, Config, HotkeyConfig, TranscriptionConfig, UiConfig};
