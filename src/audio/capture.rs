//! Audio capture using CPAL (Cross-Platform Audio Library)
//!
//! This module handles real-time audio capture from input devices,
//! converting to the format required by Whisper (16kHz mono f32).

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Device, SampleFormat, Stream, StreamConfig};
use std::sync::Arc;

use super::buffer::{AudioBuffer, WHISPER_SAMPLE_RATE};

/// Audio capture configuration
#[derive(Debug, Clone)]
pub struct CaptureConfig {
    /// Target sample rate (default: 16kHz for Whisper)
    pub sample_rate: u32,
    /// Number of channels (default: 1 for mono)
    pub channels: u16,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            sample_rate: WHISPER_SAMPLE_RATE,
            channels: 1,
        }
    }
}

/// Audio capture struct
pub struct AudioCapture {
    device: Device,
    config: StreamConfig,
    stream: Option<Stream>,
    buffer: Arc<AudioBuffer>,
    capture_config: CaptureConfig,
}

impl AudioCapture {
    /// Create a new audio capture instance with default device
    pub fn new() -> Result<Self> {
        let device = super::device::get_default_device()?;
        Self::with_device(device)
    }

    /// Create audio capture with specific device
    pub fn with_device(device: Device) -> Result<Self> {
        Self::with_device_and_config(device, CaptureConfig::default())
    }

    /// Create audio capture with device and custom config
    pub fn with_device_and_config(device: Device, capture_config: CaptureConfig) -> Result<Self> {
        // Get device's default config
        let supported_config = device
            .default_input_config()
            .context("Failed to get default input config")?;

        log::info!(
            "Device default config: {} Hz, {} channels, {:?}",
            supported_config.sample_rate().0,
            supported_config.channels(),
            supported_config.sample_format()
        );

        // Create stream config with device's native sample rate
        // We'll resample to target rate later if needed
        let config = StreamConfig {
            channels: supported_config.channels(),
            sample_rate: supported_config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        let buffer = Arc::new(AudioBuffer::with_default_capacity());

        Ok(Self {
            device,
            config,
            stream: None,
            buffer,
            capture_config,
        })
    }

    /// Start audio capture
    pub fn start(&mut self) -> Result<()> {
        if self.stream.is_some() {
            log::warn!("Audio capture already started");
            return Ok(());
        }

        // Get supported configs to determine sample format
        let supported_config = self
            .device
            .default_input_config()
            .context("Failed to get default input config")?;

        let stream = match supported_config.sample_format() {
            SampleFormat::F32 => self.build_stream_f32()?,
            SampleFormat::I16 => self.build_stream_i16()?,
            SampleFormat::U16 => self.build_stream_u16()?,
            _ => anyhow::bail!("Unsupported sample format"),
        };

        stream.play().context("Failed to start audio stream")?;
        self.stream = Some(stream);

        log::info!("Audio capture started");
        Ok(())
    }

    /// Stop audio capture
    pub fn stop(&mut self) -> Result<()> {
        if let Some(stream) = self.stream.take() {
            drop(stream);
            log::info!("Audio capture stopped");
        }
        Ok(())
    }

    /// Check if capture is currently running
    pub fn is_running(&self) -> bool {
        self.stream.is_some()
    }

    /// Get reference to the audio buffer
    pub fn buffer(&self) -> Arc<AudioBuffer> {
        Arc::clone(&self.buffer)
    }

    /// Read samples from buffer
    pub fn read_samples(&self, buffer: &mut [f32]) -> usize {
        self.buffer.read(buffer)
    }

    /// Build audio stream for f32 samples
    fn build_stream_f32(&self) -> Result<Stream> {
        let buffer = Arc::clone(&self.buffer);
        let config = self.config.clone();
        let config_clone = config.clone();
        let capture_config = self.capture_config.clone();

        let err_fn = |err| log::error!("Audio stream error: {}", err);

        let stream = self
            .device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    Self::process_f32(data, &buffer, &config_clone, &capture_config);
                },
                err_fn,
                None,
            )
            .context("Failed to build input stream")?;

        Ok(stream)
    }

    /// Build audio stream for i16 samples
    fn build_stream_i16(&self) -> Result<Stream> {
        let buffer = Arc::clone(&self.buffer);
        let config = self.config.clone();
        let config_clone = config.clone();
        let capture_config = self.capture_config.clone();

        let err_fn = |err| log::error!("Audio stream error: {}", err);

        let stream = self
            .device
            .build_input_stream(
                &config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    Self::process_i16(data, &buffer, &config_clone, &capture_config);
                },
                err_fn,
                None,
            )
            .context("Failed to build input stream")?;

        Ok(stream)
    }

    /// Build audio stream for u16 samples
    fn build_stream_u16(&self) -> Result<Stream> {
        let buffer = Arc::clone(&self.buffer);
        let config = self.config.clone();
        let config_clone = config.clone();
        let capture_config = self.capture_config.clone();

        let err_fn = |err| log::error!("Audio stream error: {}", err);

        let stream = self
            .device
            .build_input_stream(
                &config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    Self::process_u16(data, &buffer, &config_clone, &capture_config);
                },
                err_fn,
                None,
            )
            .context("Failed to build input stream")?;

        Ok(stream)
    }

    /// Process f32 samples
    fn process_f32(
        data: &[f32],
        buffer: &AudioBuffer,
        config: &StreamConfig,
        capture_config: &CaptureConfig,
    ) {
        let mut samples = data.to_vec();
        Self::process_common(&mut samples, buffer, config, capture_config);
    }

    /// Process i16 samples
    fn process_i16(
        data: &[i16],
        buffer: &AudioBuffer,
        config: &StreamConfig,
        capture_config: &CaptureConfig,
    ) {
        let mut samples: Vec<f32> = data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
        Self::process_common(&mut samples, buffer, config, capture_config);
    }

    /// Process u16 samples
    fn process_u16(
        data: &[u16],
        buffer: &AudioBuffer,
        config: &StreamConfig,
        capture_config: &CaptureConfig,
    ) {
        let mut samples: Vec<f32> = data
            .iter()
            .map(|&s| (s as i32 - 32768) as f32 / 32768.0)
            .collect();
        Self::process_common(&mut samples, buffer, config, capture_config);
    }

    /// Common processing for all sample types
    fn process_common(
        samples: &mut Vec<f32>,
        buffer: &AudioBuffer,
        config: &StreamConfig,
        capture_config: &CaptureConfig,
    ) {
        // Convert stereo to mono if needed
        if config.channels > 1 {
            *samples = Self::stereo_to_mono(samples, config.channels);
        }

        // Resample if needed
        if config.sample_rate.0 != capture_config.sample_rate {
            *samples = Self::simple_resample(samples, config.sample_rate.0, capture_config.sample_rate);
        }

        // Write to buffer
        let written = buffer.write(samples);
        if written < samples.len() {
            log::warn!(
                "Buffer overflow: {} / {} samples written",
                written,
                samples.len()
            );
        }
    }

    /// Convert stereo (or multi-channel) to mono by averaging channels
    fn stereo_to_mono(samples: &[f32], channels: u16) -> Vec<f32> {
        let channels = channels as usize;
        let frames = samples.len() / channels;
        let mut mono = Vec::with_capacity(frames);

        for frame in 0..frames {
            let mut sum = 0.0;
            for ch in 0..channels {
                sum += samples[frame * channels + ch];
            }
            mono.push(sum / channels as f32);
        }

        mono
    }

    /// Simple linear resampling
    /// For production, consider using `rubato` crate for better quality
    fn simple_resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
        if from_rate == to_rate {
            return samples.to_vec();
        }

        let ratio = to_rate as f64 / from_rate as f64;
        let output_len = (samples.len() as f64 * ratio) as usize;
        let mut output = Vec::with_capacity(output_len);

        for i in 0..output_len {
            let src_idx = i as f64 / ratio;
            let idx = src_idx as usize;

            if idx + 1 < samples.len() {
                // Linear interpolation
                let frac = src_idx - idx as f64;
                let sample = samples[idx] * (1.0 - frac) as f32 + samples[idx + 1] * frac as f32;
                output.push(sample);
            } else if idx < samples.len() {
                output.push(samples[idx]);
            }
        }

        output
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_creation() {
        match AudioCapture::new() {
            Ok(capture) => {
                assert!(!capture.is_running());
                println!("Audio capture created successfully");
            }
            Err(e) => {
                println!("Warning: Could not create audio capture: {}", e);
            }
        }
    }

    #[test]
    fn test_stereo_to_mono() {
        let stereo = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; // 3 frames, 2 channels
        let mono = AudioCapture::stereo_to_mono(&stereo, 2);

        assert_eq!(mono.len(), 3);
        assert_eq!(mono[0], 1.5); // (1.0 + 2.0) / 2
        assert_eq!(mono[1], 3.5); // (3.0 + 4.0) / 2
        assert_eq!(mono[2], 5.5); // (5.0 + 6.0) / 2
    }

    #[test]
    fn test_simple_resample() {
        let samples = vec![1.0, 2.0, 3.0, 4.0];

        // Upsample 4 -> 8 samples
        let upsampled = AudioCapture::simple_resample(&samples, 4, 8);
        assert_eq!(upsampled.len(), 8);

        // Downsample 4 -> 2 samples
        let downsampled = AudioCapture::simple_resample(&samples, 4, 2);
        assert_eq!(downsampled.len(), 2);

        // No resampling
        let same = AudioCapture::simple_resample(&samples, 4, 4);
        assert_eq!(same, samples);
    }

    #[test]
    fn test_start_stop() {
        match AudioCapture::new() {
            Ok(mut capture) => {
                assert!(!capture.is_running());

                // Start capture
                if let Ok(()) = capture.start() {
                    assert!(capture.is_running());

                    // Give it a moment to capture some audio
                    std::thread::sleep(std::time::Duration::from_millis(100));

                    // Stop capture
                    capture.stop().unwrap();
                    assert!(!capture.is_running());

                    println!("Start/stop test passed");
                } else {
                    println!("Warning: Could not start audio capture (no device?)");
                }
            }
            Err(e) => {
                println!("Warning: Could not create audio capture: {}", e);
            }
        }
    }
}
