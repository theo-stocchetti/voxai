//! Voice Activity Detection (VAD)
//!
//! This module provides voice activity detection using the WebRTC VAD algorithm.
//! VAD helps reduce unnecessary transcriptions by detecting when speech is present.

use anyhow::Result;
use webrtc_vad::{SampleRate, Vad, VadMode};

/// Sample rates supported by WebRTC VAD
const SUPPORTED_SAMPLE_RATES: &[u32] = &[8000, 16000, 32000, 48000];

/// Voice Activity Detector
pub struct VoiceActivityDetector {
    vad: Vad,
    sample_rate: u32,
    // Hysteresis counters to avoid flickering
    speech_frames: usize,
    silence_frames: usize,
    // Thresholds for hysteresis
    speech_threshold: usize,
    silence_threshold: usize,
}

impl VoiceActivityDetector {
    /// Create a new VAD with default settings (mode 2, 16kHz)
    pub fn new() -> Result<Self> {
        Self::with_mode_and_rate(VadMode::Quality, 16000)
    }

    /// Create a new VAD with specific mode and sample rate
    pub fn with_mode_and_rate(mode: VadMode, sample_rate: u32) -> Result<Self> {
        // Validate sample rate
        if !SUPPORTED_SAMPLE_RATES.contains(&sample_rate) {
            anyhow::bail!(
                "Unsupported sample rate: {}. Supported rates: {:?}",
                sample_rate,
                SUPPORTED_SAMPLE_RATES
            );
        }

        let vad = Vad::new_with_rate_and_mode(
            Self::sample_rate_to_enum(sample_rate)?,
            mode,
        );

        Ok(Self {
            vad,
            sample_rate,
            speech_frames: 0,
            silence_frames: 0,
            // Require 3 consecutive speech frames to trigger
            speech_threshold: 3,
            // Require 10 consecutive silence frames to stop
            silence_threshold: 10,
        })
    }

    /// Check if the given audio frame contains speech
    ///
    /// Frame size must be:
    /// - 10ms, 20ms, or 30ms of audio
    /// - At the configured sample rate
    ///
    /// Returns true if speech is detected (with hysteresis)
    pub fn is_speech(&mut self, audio_frame: &[i16]) -> Result<bool> {
        // Check frame size validity
        self.validate_frame_size(audio_frame.len())?;

        // Run VAD on frame
        let has_speech = self.vad.is_voice_segment(audio_frame)
            .map_err(|_| anyhow::anyhow!("VAD failed to process frame"))?;

        // Apply hysteresis
        if has_speech {
            self.speech_frames += 1;
            self.silence_frames = 0;
        } else {
            self.silence_frames += 1;
            self.speech_frames = 0;
        }

        // Return true if we have enough consecutive speech frames
        // and haven't had enough silence frames to stop
        Ok(self.speech_frames >= self.speech_threshold
            && self.silence_frames < self.silence_threshold)
    }

    /// Check if frame contains speech (for f32 audio)
    ///
    /// Converts f32 samples to i16 and runs VAD
    pub fn is_speech_f32(&mut self, audio_frame: &[f32]) -> Result<bool> {
        let i16_samples: Vec<i16> = audio_frame
            .iter()
            .map(|&s| (s * i16::MAX as f32) as i16)
            .collect();

        self.is_speech(&i16_samples)
    }

    /// Set the aggressiveness mode
    pub fn set_mode(&mut self, mode: VadMode) {
        self.vad.set_mode(mode);
    }

    /// Set speech threshold (consecutive frames needed to trigger)
    pub fn set_speech_threshold(&mut self, threshold: usize) {
        self.speech_threshold = threshold;
    }

    /// Set silence threshold (consecutive frames needed to stop)
    pub fn set_silence_threshold(&mut self, threshold: usize) {
        self.silence_threshold = threshold;
    }

    /// Reset hysteresis counters
    pub fn reset(&mut self) {
        self.speech_frames = 0;
        self.silence_frames = 0;
    }

    /// Get the configured sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Validate that frame size is correct
    fn validate_frame_size(&self, frame_len: usize) -> Result<()> {
        let valid_sizes = self.get_valid_frame_sizes();

        if !valid_sizes.contains(&frame_len) {
            anyhow::bail!(
                "Invalid frame size: {}. Valid sizes for {}Hz: {:?}",
                frame_len,
                self.sample_rate,
                valid_sizes
            );
        }

        Ok(())
    }

    /// Get valid frame sizes for current sample rate (10ms, 20ms, 30ms)
    fn get_valid_frame_sizes(&self) -> Vec<usize> {
        vec![
            (self.sample_rate * 10 / 1000) as usize, // 10ms
            (self.sample_rate * 20 / 1000) as usize, // 20ms
            (self.sample_rate * 30 / 1000) as usize, // 30ms
        ]
    }

    /// Convert u32 sample rate to enum
    fn sample_rate_to_enum(rate: u32) -> Result<SampleRate> {
        match rate {
            8000 => Ok(SampleRate::Rate8kHz),
            16000 => Ok(SampleRate::Rate16kHz),
            32000 => Ok(SampleRate::Rate32kHz),
            48000 => Ok(SampleRate::Rate48kHz),
            _ => anyhow::bail!("Unsupported sample rate: {}", rate),
        }
    }
}

impl Default for VoiceActivityDetector {
    fn default() -> Self {
        Self::new().expect("Failed to create default VAD")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vad_creation() {
        let vad = VoiceActivityDetector::new();
        assert!(vad.is_ok());

        let vad = vad.unwrap();
        assert_eq!(vad.sample_rate(), 16000);
    }

    #[test]
    fn test_vad_modes() {
        let vad = VoiceActivityDetector::with_mode_and_rate(VadMode::Quality, 16000);
        assert!(vad.is_ok());

        let mut vad = vad.unwrap();

        // Change mode - just test that it doesn't panic
        vad.set_mode(VadMode::VeryAggressive);
    }

    #[test]
    fn test_invalid_sample_rate() {
        let vad = VoiceActivityDetector::with_mode_and_rate(VadMode::Quality, 44100);
        assert!(vad.is_err());
    }

    #[test]
    fn test_silence_detection() {
        let mut vad = VoiceActivityDetector::new().unwrap();

        // 10ms of silence at 16kHz = 160 samples
        let silence = vec![0i16; 160];

        // Initially should not detect speech (need consecutive frames)
        let result = vad.is_speech(&silence);
        assert!(result.is_ok());
        // First frame of silence should not immediately trigger
    }

    #[test]
    fn test_frame_size_validation() {
        let mut vad = VoiceActivityDetector::new().unwrap();

        // Valid frame sizes for 16kHz: 160 (10ms), 320 (20ms), 480 (30ms)
        let valid_frame = vec![0i16; 160];
        assert!(vad.is_speech(&valid_frame).is_ok());

        // Invalid frame size
        let invalid_frame = vec![0i16; 100];
        assert!(vad.is_speech(&invalid_frame).is_err());
    }

    #[test]
    fn test_f32_conversion() {
        let mut vad = VoiceActivityDetector::new().unwrap();

        // 10ms of silence at 16kHz
        let silence_f32 = vec![0.0f32; 160];

        let result = vad.is_speech_f32(&silence_f32);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_frame_sizes() {
        let vad = VoiceActivityDetector::new().unwrap();
        let sizes = vad.get_valid_frame_sizes();

        // For 16kHz: 160, 320, 480
        assert_eq!(sizes, vec![160, 320, 480]);
    }

    #[test]
    fn test_hysteresis_thresholds() {
        let mut vad = VoiceActivityDetector::new().unwrap();

        // Set custom thresholds
        vad.set_speech_threshold(5);
        vad.set_silence_threshold(15);

        // Reset counters
        vad.reset();
        assert_eq!(vad.speech_frames, 0);
        assert_eq!(vad.silence_frames, 0);
    }

    #[test]
    fn test_all_sample_rates() {
        for &rate in SUPPORTED_SAMPLE_RATES {
            let vad = VoiceActivityDetector::with_mode_and_rate(VadMode::Quality, rate);
            assert!(vad.is_ok(), "Should support {}Hz", rate);

            let vad = vad.unwrap();
            assert_eq!(vad.sample_rate(), rate);
        }
    }

    #[test]
    fn test_speech_pattern() {
        let mut vad = VoiceActivityDetector::new().unwrap();
        vad.set_speech_threshold(2); // Lower threshold for testing

        // Create a simple audio pattern (sine wave)
        let mut audio = Vec::new();
        for i in 0..160 {
            // 10ms @ 16kHz
            let sample = ((i as f32 / 160.0) * std::f32::consts::PI * 2.0).sin();
            audio.push((sample * 1000.0) as i16);
        }

        // Process the audio
        let result = vad.is_speech(&audio);
        assert!(result.is_ok());
        // Result depends on VAD's analysis
    }

    #[test]
    fn test_reset() {
        let mut vad = VoiceActivityDetector::new().unwrap();

        // Process some frames
        let frame = vec![100i16; 160];
        let _ = vad.is_speech(&frame);
        let _ = vad.is_speech(&frame);

        // Reset should clear counters
        vad.reset();

        // Verify counters are reset (they should be accessible as pub in tests)
        assert_eq!(vad.speech_frames, 0);
        assert_eq!(vad.silence_frames, 0);
    }
}
