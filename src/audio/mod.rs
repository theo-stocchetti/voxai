//! Audio capture and processing module
//!
//! This module handles all audio-related functionality including:
//! - Audio device enumeration and capture (CPAL)
//! - Noise reduction (RNNoise)
//! - Voice Activity Detection (VAD)
//! - Audio buffering and resampling

pub mod buffer;
pub mod capture;
pub mod device;
pub mod noise_reduction;
pub mod vad;
