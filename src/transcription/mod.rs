//! Transcription module using Whisper.cpp
//!
//! This module handles:
//! - Whisper context management
//! - Model downloading and verification
//! - Transcription pipeline
//! - Audio chunking

pub mod models;
pub mod downloader;
pub mod whisper;

// Future modules (to be implemented)
// pub mod pipeline;
// pub mod chunking;

// Re-exports for convenience
pub use models::{ModelSize, get_model_path, get_models_dir, is_model_downloaded};
pub use downloader::{download_model, verify_model, delete_model, list_downloaded_models};
pub use whisper::WhisperContext;
