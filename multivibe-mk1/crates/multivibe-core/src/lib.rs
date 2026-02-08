pub mod software;
pub mod network {
    pub mod multicast;
    pub mod websocket;
}

use std::sync::Arc;
use tokio::sync::broadcast;

/// Common audio configuration for Multivibe.
/// We target 48kHz Stereo for high-fidelity TV audio.
pub const SAMPLE_RATE: u32 = 48_000;
pub const CHANNELS: u16 = 2;

/// A thread-safe chunk of audio data.
/// Using Arc<Vec<f32>> allows us to broadcast the same buffer 
/// to multiple neighbors with zero-copy overhead.
pub type AudioChunk = Arc<Vec<f32>>;

/// The contract for any audio input device.
pub trait AudioSource: Send + Sync {
    /// Returns a human-readable name for the device (e.g., "PCM1808" or "BlackHole").
    fn name(&self) -> &str;

    /// Starts the capture loop.
    /// Samples are pushed into the provided broadcast channel.
    fn start(&mut self, tx: broadcast::Sender<AudioChunk>) -> Result<(), String>;

    /// Stops the capture loop and releases the hardware.
    fn stop(&mut self) -> Result<(), String>;
}

/// A simple metadata struct for the UI to show what's happening.
#[derive(Debug, Clone)]
pub struct StreamStatus {
    pub active: bool,
    pub source_name: String,
    pub sample_rate: u32,
    pub bit_depth: u8,
}

/// The contract for broadcasting audio data over the network.
#[async_trait::async_trait]
pub trait Broadcaster: Send + Sync {
    /// Starts the broadcast loop, listening for chunks on the receiver.
    async fn run(&self, mut rx: broadcast::Receiver<AudioChunk>) -> Result<(), String>;
}
