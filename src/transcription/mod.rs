//! Transcription module using Whisper.cpp
//!
//! This module handles:
//! - Whisper context management
//! - Model downloading and verification
//! - Transcription pipeline
//! - Audio chunking

pub mod downloader;
pub mod models;
pub mod whisper;

// Future modules (to be implemented)
// pub mod pipeline;
// pub mod chunking;

// Re-exports for convenience
pub use downloader::{delete_model, download_model, list_downloaded_models, verify_model};
pub use models::{get_model_path, get_models_dir, is_model_downloaded, ModelSize};
pub use whisper::WhisperContext;
