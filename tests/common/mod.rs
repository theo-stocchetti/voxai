//! Common test utilities and fixtures

use std::path::PathBuf;

/// Get path to test fixtures
pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures")
}

/// Get path to test audio samples
pub fn audio_samples_dir() -> PathBuf {
    fixtures_dir().join("audio")
}

/// Mock audio sample data (16kHz, mono, f32)
pub fn mock_audio_sample(duration_secs: usize) -> Vec<f32> {
    let sample_rate = 16000;
    let num_samples = sample_rate * duration_secs;

    // Generate simple sine wave
    (0..num_samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            let freq = 440.0; // A4 note
            (2.0 * std::f32::consts::PI * freq * t).sin() * 0.5
        })
        .collect()
}

/// Create temporary config for testing
pub fn create_temp_config() -> voxai::config::Config {
    voxai::config::Config::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_audio_sample() {
        let sample = mock_audio_sample(1);
        assert_eq!(sample.len(), 16000); // 1 second at 16kHz

        // Verify amplitude is in range [-1, 1]
        for &s in &sample {
            assert!(s >= -1.0 && s <= 1.0);
        }
    }
}
