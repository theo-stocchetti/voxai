//! VoxAI - Real-time Audio Transcription
//!
//! A desktop application for real-time audio transcription using Whisper AI.
//! Supports Windows, macOS, and Linux with hardware acceleration.

use anyhow::Result;
use log::info;

mod audio;
mod config;
mod gpu;
mod hotkeys;
mod output;
mod transcription;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

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
    println!("\n===========================================");
    println!("  VoxAI is running!");
    println!("  Press Ctrl+C to quit");
    println!("===========================================\n");

    // Wait for Ctrl+C signal
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            info!("Received Ctrl+C signal, shutting down...");
            println!("\nShutting down VoxAI...");
        }
        Err(err) => {
            eprintln!("Error waiting for Ctrl+C: {}", err);
        }
    }

    info!("VoxAI shutdown complete");
    Ok(())
}
