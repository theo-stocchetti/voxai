//! Ring buffer for audio streaming
//!
//! This module provides a thread-safe ring buffer for audio samples,
//! allowing efficient producer-consumer communication between audio
//! capture and processing threads.

use ringbuf::HeapRb;
use std::sync::{Arc, Mutex};

/// Sample rate required by Whisper (16 kHz)
pub const WHISPER_SAMPLE_RATE: u32 = 16000;

/// Default buffer capacity (5 seconds of audio at 16kHz)
pub const DEFAULT_BUFFER_CAPACITY: usize = WHISPER_SAMPLE_RATE as usize * 5;

type Producer = ringbuf::HeapProducer<f32>;
type Consumer = ringbuf::HeapConsumer<f32>;

/// Ring buffer for audio samples (f32)
pub struct AudioBuffer {
    producer: Arc<Mutex<Producer>>,
    consumer: Arc<Mutex<Consumer>>,
    capacity: usize,
}

impl AudioBuffer {
    /// Create a new audio buffer with given capacity (in samples)
    pub fn new(capacity: usize) -> Self {
        let rb = HeapRb::<f32>::new(capacity);
        let (producer, consumer) = rb.split();

        Self {
            producer: Arc::new(Mutex::new(producer)),
            consumer: Arc::new(Mutex::new(consumer)),
            capacity,
        }
    }

    /// Create a new audio buffer with default capacity (5 seconds)
    pub fn with_default_capacity() -> Self {
        Self::new(DEFAULT_BUFFER_CAPACITY)
    }

    /// Write audio samples to the buffer
    ///
    /// Returns the number of samples actually written.
    /// If the buffer is full, some samples may be dropped.
    pub fn write(&self, samples: &[f32]) -> usize {
        let mut producer = self.producer.lock().unwrap();
        producer.push_slice(samples)
    }

    /// Read audio samples from the buffer
    ///
    /// Returns the number of samples actually read.
    /// Reads up to `buffer.len()` samples.
    pub fn read(&self, buffer: &mut [f32]) -> usize {
        let mut consumer = self.consumer.lock().unwrap();
        consumer.pop_slice(buffer)
    }

    /// Try to read exactly `count` samples
    ///
    /// Returns `Some(Vec<f32>)` if exactly `count` samples are available,
    /// otherwise returns `None`.
    pub fn try_read_exact(&self, count: usize) -> Option<Vec<f32>> {
        let consumer = self.consumer.lock().unwrap();
        if consumer.len() >= count {
            drop(consumer); // Release lock before calling read
            let mut buffer = vec![0.0f32; count];
            let read = self.read(&mut buffer);
            if read == count {
                return Some(buffer);
            }
        }
        None
    }

    /// Get number of samples currently in the buffer
    pub fn len(&self) -> usize {
        let consumer = self.consumer.lock().unwrap();
        consumer.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get number of free slots in buffer
    pub fn available(&self) -> usize {
        let producer = self.producer.lock().unwrap();
        producer.free_len()
    }

    /// Clear all samples from the buffer
    pub fn clear(&self) {
        let mut consumer = self.consumer.lock().unwrap();
        consumer.clear();
    }

    /// Get a clone of the producer for use in other threads
    pub fn get_producer(&self) -> Arc<Mutex<Producer>> {
        Arc::clone(&self.producer)
    }

    /// Get a clone of the consumer for use in other threads
    pub fn get_consumer(&self) -> Arc<Mutex<Consumer>> {
        Arc::clone(&self.consumer)
    }
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self::with_default_capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = AudioBuffer::new(1000);
        assert_eq!(buffer.capacity(), 1000);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_write_read() {
        let buffer = AudioBuffer::new(1000);

        // Write samples
        let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let written = buffer.write(&samples);
        assert_eq!(written, 5);
        assert_eq!(buffer.len(), 5);

        // Read samples
        let mut read_buffer = vec![0.0; 5];
        let read = buffer.read(&mut read_buffer);
        assert_eq!(read, 5);
        assert_eq!(read_buffer, samples);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_buffer_overflow() {
        let buffer = AudioBuffer::new(10);

        // Try to write more than capacity
        let samples = vec![1.0; 20];
        let written = buffer.write(&samples);

        // Should only write capacity amount
        assert!(written <= 10);
    }

    #[test]
    fn test_try_read_exact() {
        let buffer = AudioBuffer::new(1000);

        // Write 10 samples
        buffer.write(&vec![1.0; 10]);

        // Try to read exactly 5 samples - should succeed
        let result = buffer.try_read_exact(5);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 5);

        // Try to read exactly 10 samples - should fail (only 5 left)
        let result = buffer.try_read_exact(10);
        assert!(result.is_none());
    }

    #[test]
    fn test_clear() {
        let buffer = AudioBuffer::new(1000);

        buffer.write(&vec![1.0; 100]);
        assert_eq!(buffer.len(), 100);

        buffer.clear();
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let buffer = Arc::new(AudioBuffer::new(10000));

        // Writer thread
        let buffer_writer = Arc::clone(&buffer);
        let writer = thread::spawn(move || {
            for _ in 0..10 {
                let samples = vec![1.0; 100];
                buffer_writer.write(&samples);
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        // Reader thread
        let buffer_reader = Arc::clone(&buffer);
        let reader = thread::spawn(move || {
            let mut total_read = 0;
            for _ in 0..20 {
                let mut buf = vec![0.0; 50];
                total_read += buffer_reader.read(&mut buf);
                thread::sleep(std::time::Duration::from_millis(5));
            }
            total_read
        });

        writer.join().unwrap();
        let total_read = reader.join().unwrap();

        // Should have read most of the data
        assert!(total_read > 0);
    }
}
