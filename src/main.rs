//! VoxAI - Real-time Audio Transcription
//!
//! A desktop application for real-time audio transcription using Whisper AI.
//! Supports Windows, macOS, and Linux with hardware acceleration.

use anyhow::Result;
use log::{info, error};

mod audio;
mod transcription;
mod ui;
mod config;
mod hotkeys;
mod output;
mod gpu;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    info!("VoxAI starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = config::Config::load()?;
    info!("Configuration loaded");
    info!("Hotkey: {}", config.hotkey);
    info!("Model size: {}", config.model_size);

    // TODO: Initialize components
    // - Audio capture
    // - Whisper transcription engine
    // - System tray UI
    // - Global hotkeys
    // - Text output handler

    info!("VoxAI initialized successfully");
    println!("VoxAI is running. Press Ctrl+C to quit.");

    // TODO: Run main event loop

    Ok(())
}
