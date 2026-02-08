use crate::{AudioChunk, AudioSource, CHANNELS, SAMPLE_RATE};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Arc;
use tokio::sync::broadcast;

// A wrapper to make cpal::Stream Send + Sync.
// This is safe because we only use the stream to manage the lifecycle (play/stop).
#[allow(dead_code)]
struct SendSyncStream(cpal::Stream);
unsafe impl Send for SendSyncStream {}
unsafe impl Sync for SendSyncStream {}

pub struct SoftwareSource {
    device_name: String,
    stream: Option<SendSyncStream>,
}

impl SoftwareSource {
    /// Creates a new source. 'target_name' should be "BlackHole" for your dev setup.
    pub fn new(target_name: &str) -> Result<Self, String> {
        let host = cpal::default_host();
        let devices = host.input_devices().map_err(|e| e.to_string())?;
    
        for (i, dev) in devices.enumerate() {
            if let Ok(name) = dev.name() {
                println!("DEBUG: Found device [{}]: {}", i, name);
            }
        }
    
        // Now find the one we want
        let device = host.input_devices()
            .map_err(|e| e.to_string())?
            .find(|x| x.name().map(|n| n.contains(target_name)).unwrap_or(false))
            .ok_or_else(|| format!("Audio device '{}' not found", target_name))?;
    
        Ok(Self {
            device_name: device.name().unwrap_or_else(|_| target_name.to_string()),
            stream: None,
        })
    }
}

impl AudioSource for SoftwareSource {
    fn name(&self) -> &str {
        &self.device_name
    }

    fn start(&mut self, tx: broadcast::Sender<AudioChunk>) -> Result<(), String> {
        let host = cpal::default_host();
        let device = host
            .input_devices()
            .map_err(|e| e.to_string())?
            .find(|x| x.name().map(|n| n.contains(&self.device_name)).unwrap_or(false))
            .ok_or("Device disconnected unexpectedly")?;

        let config = cpal::StreamConfig {
            channels: CHANNELS,
            sample_rate: cpal::SampleRate(SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };

        // build_input_stream expects a callback that runs on the audio thread.
        let stream = device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &_| {
                    // 1. Wrap the samples in an Arc to avoid copying the vector for every listener.
                    let chunk = Arc::new(data.to_vec());
                    // 2. Shotgun the data to the rest of the app.
                    let _ = tx.send(chunk);
                },
                |err| eprintln!("Software capture error: {}", err),
                None, // No timeout
            )
            .map_err(|e| e.to_string())?;

        stream.play().map_err(|e| e.to_string())?;
        self.stream = Some(SendSyncStream(stream));

        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        // Dropping the cpal::Stream automatically stops the audio thread.
        self.stream = None;
        Ok(())
    }
}
