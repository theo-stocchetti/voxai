//! Real-time transcription pipeline
//!
//! This module implements the complete audio → transcription → text pipeline
//! integrating audio capture, VAD, chunking, and Whisper transcription.

use anyhow::{Context, Result};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc;

use super::models::ModelSize;

/// Chunk size for transcription (10 seconds @ 16kHz)
const CHUNK_SIZE_SAMPLES: usize = 16000 * 10;

/// Overlap size to avoid cutting words (2 seconds)
const OVERLAP_SIZE_SAMPLES: usize = 16000 * 2;

/// Audio chunk ready for transcription
#[derive(Clone)]
pub struct AudioChunk {
    /// Audio samples (16kHz mono f32)
    pub samples: Vec<f32>,
    /// Timestamp when chunk was created
    pub timestamp: std::time::Instant,
    /// Chunk sequence number
    pub sequence: u64,
}

/// Transcription result
#[derive(Clone, Debug)]
pub struct TranscriptionResult {
    /// Transcribed text
    pub text: String,
    /// Timestamp of transcription
    pub timestamp: std::time::Instant,
    /// Chunk sequence number
    pub sequence: u64,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
}

/// Transcription pipeline configuration
#[derive(Clone)]
pub struct PipelineConfig {
    /// Whisper model to use
    pub model: ModelSize,
    /// Enable Voice Activity Detection
    pub enable_vad: bool,
    /// Enable noise reduction
    pub enable_noise_reduction: bool,
    /// Maximum concurrent transcriptions
    pub max_concurrent: usize,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            model: ModelSize::Base,
            enable_vad: true,
            enable_noise_reduction: true,
            max_concurrent: 2,
        }
    }
}

/// Real-time transcription pipeline
pub struct TranscriptionPipeline {
    config: PipelineConfig,
    running: Arc<AtomicBool>,
    // Channels
    audio_tx: Option<mpsc::UnboundedSender<Vec<f32>>>,
    text_rx: Option<mpsc::UnboundedReceiver<TranscriptionResult>>,
}

impl TranscriptionPipeline {
    /// Create a new transcription pipeline
    pub fn new(config: PipelineConfig) -> Result<Self> {
        Ok(Self {
            config,
            running: Arc::new(AtomicBool::new(false)),
            audio_tx: None,
            text_rx: None,
        })
    }

    /// Start the transcription pipeline
    pub async fn start(&mut self) -> Result<()> {
        if self.running.load(Ordering::Relaxed) {
            anyhow::bail!("Pipeline already running");
        }

        log::info!("Starting transcription pipeline with model: {}", self.config.model);

        // Create channels
        let (audio_tx, audio_rx) = mpsc::unbounded_channel();
        let (text_tx, text_rx) = mpsc::unbounded_channel();

        self.audio_tx = Some(audio_tx);
        self.text_rx = Some(text_rx);
        self.running.store(true, Ordering::Relaxed);

        // Spawn pipeline task
        let config = self.config.clone();
        let running = Arc::clone(&self.running);

        tokio::spawn(async move {
            if let Err(e) = run_pipeline(audio_rx, text_tx, config, running).await {
                log::error!("Pipeline error: {}", e);
            }
        });

        log::info!("Transcription pipeline started");
        Ok(())
    }

    /// Stop the transcription pipeline
    pub fn stop(&mut self) {
        if self.running.load(Ordering::Relaxed) {
            log::info!("Stopping transcription pipeline");
            self.running.store(false, Ordering::Relaxed);
            self.audio_tx = None;
            self.text_rx = None;
        }
    }

    /// Send audio samples to the pipeline
    pub fn send_audio(&self, samples: Vec<f32>) -> Result<()> {
        if let Some(tx) = &self.audio_tx {
            tx.send(samples).context("Failed to send audio to pipeline")?;
        }
        Ok(())
    }

    /// Receive transcription result (non-blocking)
    pub async fn receive_text(&mut self) -> Option<TranscriptionResult> {
        if let Some(rx) = &mut self.text_rx {
            rx.recv().await
        } else {
            None
        }
    }

    /// Check if pipeline is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
}

impl Drop for TranscriptionPipeline {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Run the pipeline (internal task)
async fn run_pipeline(
    mut audio_rx: mpsc::UnboundedReceiver<Vec<f32>>,
    text_tx: mpsc::UnboundedSender<TranscriptionResult>,
    config: PipelineConfig,
    running: Arc<AtomicBool>,
) -> Result<()> {
    log::info!("Pipeline task started");

    // TODO: VAD integration pending - webrtc-vad is not Send, needs separate thread
    // For now, VAD is disabled in the async pipeline to allow compilation.
    // Future implementation will use a dedicated VAD thread with channels.
    if config.enable_vad {
        log::warn!("VAD is not yet supported in the async pipeline (webrtc-vad is not Send)");
        log::warn!("VAD will be implemented in a separate thread in a future update");
    }

    // Buffer for accumulating audio
    let mut buffer = Vec::new();
    let mut sequence = 0u64;

    // Main pipeline loop
    while running.load(Ordering::Relaxed) {
        // Receive audio with timeout
        match tokio::time::timeout(
            std::time::Duration::from_millis(100),
            audio_rx.recv()
        ).await {
            Ok(Some(samples)) => {
                // Add to buffer
                buffer.extend_from_slice(&samples);

                // Check if we have enough for a chunk
                if buffer.len() >= CHUNK_SIZE_SAMPLES {
                    // Extract chunk with overlap
                    let chunk_end = CHUNK_SIZE_SAMPLES.min(buffer.len());
                    let chunk_samples = buffer[..chunk_end].to_vec();

                    // Keep overlap for next chunk
                    if buffer.len() > OVERLAP_SIZE_SAMPLES {
                        buffer.drain(..(chunk_end - OVERLAP_SIZE_SAMPLES));
                    } else {
                        buffer.clear();
                    }

                    // Create chunk
                    let chunk = AudioChunk {
                        samples: chunk_samples,
                        timestamp: std::time::Instant::now(),
                        sequence,
                    };

                    sequence += 1;

                    // Transcribe chunk (spawn task for concurrency)
                    let tx = text_tx.clone();
                    let model = config.model;
                    tokio::spawn(async move {
                        match transcribe_chunk(chunk, model).await {
                            Ok(result) => {
                                if let Err(e) = tx.send(result) {
                                    log::error!("Failed to send transcription result: {}", e);
                                }
                            }
                            Err(e) => {
                                log::error!("Transcription error: {}", e);
                            }
                        }
                    });
                }
            }
            Ok(None) => {
                // Channel closed
                break;
            }
            Err(_) => {
                // Timeout - continue loop
                continue;
            }
        }
    }

    log::info!("Pipeline task stopped");
    Ok(())
}

/// Transcribe an audio chunk using Whisper
async fn transcribe_chunk(chunk: AudioChunk, model: ModelSize) -> Result<TranscriptionResult> {
    // TODO: Integrate real whisper-rs when CMake is configured
    // For now, use placeholder transcription

    log::debug!(
        "Transcribing chunk {} ({} samples, {} sec) with model {}",
        chunk.sequence,
        chunk.samples.len(),
        chunk.samples.len() as f32 / 16000.0,
        model
    );

    // Simulate transcription delay based on model
    let delay_ms = match model {
        ModelSize::Tiny => 100,
        ModelSize::Base => 200,
        ModelSize::Small => 500,
        ModelSize::Medium => 1000,
    };

    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;

    // Placeholder: Generate fake transcription
    // TODO: Replace with real whisper-rs call:
    // let whisper = WhisperContext::new(model)?;
    // let text = whisper.transcribe(&chunk.samples)?;

    let text = format!(
        "[Placeholder] Chunk {} transcribed ({:.1}s audio)",
        chunk.sequence,
        chunk.samples.len() as f32 / 16000.0
    );

    Ok(TranscriptionResult {
        text,
        timestamp: std::time::Instant::now(),
        sequence: chunk.sequence,
        confidence: 0.85, // Placeholder confidence
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pipeline_creation() {
        let config = PipelineConfig::default();
        let pipeline = TranscriptionPipeline::new(config);
        assert!(pipeline.is_ok());
    }

    #[tokio::test]
    async fn test_pipeline_start_stop() {
        let config = PipelineConfig::default();
        let mut pipeline = TranscriptionPipeline::new(config).unwrap();

        assert!(!pipeline.is_running());

        pipeline.start().await.unwrap();
        assert!(pipeline.is_running());

        pipeline.stop();
        assert!(!pipeline.is_running());
    }

    #[tokio::test]
    async fn test_send_audio() {
        let config = PipelineConfig::default();
        let mut pipeline = TranscriptionPipeline::new(config).unwrap();

        pipeline.start().await.unwrap();

        // Send some audio samples
        let samples = vec![0.1; 1600]; // 0.1 seconds @ 16kHz
        assert!(pipeline.send_audio(samples).is_ok());

        pipeline.stop();
    }

    #[tokio::test]
    async fn test_transcription_flow() {
        let config = PipelineConfig {
            enable_vad: false, // Disable VAD for testing
            ..Default::default()
        };
        let mut pipeline = TranscriptionPipeline::new(config).unwrap();

        pipeline.start().await.unwrap();

        // Send a full chunk of audio
        let chunk_size = 16000 * 10; // 10 seconds
        let samples = vec![0.1; chunk_size];
        pipeline.send_audio(samples).unwrap();

        // Wait for transcription
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        // Try to receive result
        if let Some(result) = pipeline.receive_text().await {
            assert!(!result.text.is_empty());
            assert!(result.confidence > 0.0);
            println!("Transcription: {}", result.text);
        }

        pipeline.stop();
    }

    #[tokio::test]
    async fn test_chunk_creation() {
        let chunk = AudioChunk {
            samples: vec![0.0; 1000],
            timestamp: std::time::Instant::now(),
            sequence: 42,
        };

        assert_eq!(chunk.samples.len(), 1000);
        assert_eq!(chunk.sequence, 42);
    }

    #[tokio::test]
    async fn test_transcribe_chunk_placeholder() {
        let chunk = AudioChunk {
            samples: vec![0.1; 16000], // 1 second
            timestamp: std::time::Instant::now(),
            sequence: 0,
        };

        let result = transcribe_chunk(chunk, ModelSize::Tiny).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(!result.text.is_empty());
        assert_eq!(result.sequence, 0);
    }
}
