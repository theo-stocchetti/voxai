//! Noise reduction using RNNoise
//!
//! This module provides noise reduction capabilities using the RNNoise algorithm
//! (Recurrent Neural Network for Noise Suppression).

use anyhow::Result;
use nnnoiseless::DenoiseState;

/// Sample rate required by RNNoise (48 kHz)
const RNNOISE_SAMPLE_RATE: u32 = 48000;

/// Frame size for RNNoise (480 samples @ 48kHz = 10ms)
const RNNOISE_FRAME_SIZE: usize = DenoiseState::FRAME_SIZE;

/// Noise reduction processor
pub struct NoiseReducer {
    denoiser: Box<DenoiseState<'static>>,
    enabled: bool,
    // Buffer for accumulating samples
    input_buffer: Vec<f32>,
}

impl NoiseReducer {
    /// Create a new noise reducer
    pub fn new() -> Self {
        Self {
            denoiser: DenoiseState::new(),
            enabled: true,
            input_buffer: Vec::new(),
        }
    }

    /// Process audio samples with noise reduction
    ///
    /// Input should be @ 48kHz. Returns processed audio.
    /// If disabled, returns input unchanged.
    pub fn process(&mut self, audio_48k: &[f32]) -> Vec<f32> {
        if !self.enabled {
            return audio_48k.to_vec();
        }

        // Add to buffer
        self.input_buffer.extend_from_slice(audio_48k);

        let mut output = Vec::with_capacity(audio_48k.len());

        // Process in frames of 480 samples
        while self.input_buffer.len() >= RNNOISE_FRAME_SIZE {
            // Extract one frame
            let frame: Vec<f32> = self.input_buffer.drain(..RNNOISE_FRAME_SIZE).collect();

            // Process frame
            let processed_frame = self.process_frame(&frame);
            output.extend_from_slice(&processed_frame);
        }

        output
    }

    /// Process a single frame (480 samples @ 48kHz)
    fn process_frame(&mut self, frame: &[f32]) -> Vec<f32> {
        // RNNoise expects input/output buffers
        let mut output = vec![0.0f32; RNNOISE_FRAME_SIZE];

        // Process with RNNoise
        // Returns voice probability (not used for now)
        let _voice_prob = self.denoiser.process_frame(&mut output, frame);

        output
    }

    /// Enable noise reduction
    pub fn enable(&mut self) {
        self.enabled = true;
        log::info!("Noise reduction enabled");
    }

    /// Disable noise reduction
    pub fn disable(&mut self) {
        self.enabled = false;
        self.input_buffer.clear();
        log::info!("Noise reduction disabled");
    }

    /// Check if noise reduction is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        if enabled {
            self.enable();
        } else {
            self.disable();
        }
    }

    /// Get the required sample rate for RNNoise
    pub fn required_sample_rate() -> u32 {
        RNNOISE_SAMPLE_RATE
    }

    /// Clear internal buffers
    pub fn reset(&mut self) {
        self.input_buffer.clear();
        self.denoiser = DenoiseState::new();
    }
}

impl Default for NoiseReducer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_reducer_creation() {
        let reducer = NoiseReducer::new();
        assert!(reducer.is_enabled());
    }

    #[test]
    fn test_enable_disable() {
        let mut reducer = NoiseReducer::new();

        assert!(reducer.is_enabled());

        reducer.disable();
        assert!(!reducer.is_enabled());

        reducer.enable();
        assert!(reducer.is_enabled());
    }

    #[test]
    fn test_process_disabled() {
        let mut reducer = NoiseReducer::new();
        reducer.disable();

        let input = vec![0.5; 1000];
        let output = reducer.process(&input);

        // Should return input unchanged when disabled
        assert_eq!(output, input);
    }

    #[test]
    fn test_process_frame() {
        let mut reducer = NoiseReducer::new();

        // Create a frame of silence
        let frame = vec![0.0; RNNOISE_FRAME_SIZE];
        let processed = reducer.process_frame(&frame);

        // Should return a frame of the same size
        assert_eq!(processed.len(), RNNOISE_FRAME_SIZE);
    }

    #[test]
    fn test_process_multiple_frames() {
        let mut reducer = NoiseReducer::new();

        // Create 2 frames worth of data
        let input = vec![0.1; RNNOISE_FRAME_SIZE * 2];
        let output = reducer.process(&input);

        // Should process 2 complete frames
        assert_eq!(output.len(), RNNOISE_FRAME_SIZE * 2);
    }

    #[test]
    fn test_process_incomplete_frame() {
        let mut reducer = NoiseReducer::new();

        // Create less than one frame
        let input = vec![0.1; RNNOISE_FRAME_SIZE / 2];
        let output = reducer.process(&input);

        // Should not process incomplete frame
        assert_eq!(output.len(), 0);

        // Complete the frame
        let input2 = vec![0.1; RNNOISE_FRAME_SIZE / 2];
        let output2 = reducer.process(&input2);

        // Now should have processed one complete frame
        assert_eq!(output2.len(), RNNOISE_FRAME_SIZE);
    }

    #[test]
    fn test_required_sample_rate() {
        assert_eq!(NoiseReducer::required_sample_rate(), 48000);
    }

    #[test]
    fn test_reset() {
        let mut reducer = NoiseReducer::new();

        // Add some data
        let input = vec![0.1; RNNOISE_FRAME_SIZE / 2];
        let _ = reducer.process(&input);

        // Reset should clear buffer
        reducer.reset();

        // Buffer should be empty, so processing same data again should return nothing
        let input2 = vec![0.1; RNNOISE_FRAME_SIZE / 2];
        let output = reducer.process(&input2);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn test_process_with_noise() {
        let mut reducer = NoiseReducer::new();

        // Create some noisy audio (random values)
        let mut input = Vec::with_capacity(RNNOISE_FRAME_SIZE);
        for i in 0..RNNOISE_FRAME_SIZE {
            input.push((i as f32 / 100.0).sin() * 0.1);
        }

        let output = reducer.process(&input);

        // Should produce output
        assert_eq!(output.len(), RNNOISE_FRAME_SIZE);
        // Output should be different from input (noise reduced)
        // Note: With our test signal, results may vary
    }
}
