use crate::{AudioChunk, Broadcaster};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::UdpSocket;
use tokio::sync::broadcast;

/// A broadcaster that sends audio data over UDP multicast.
pub struct MulticastBroadcaster {
    addr: SocketAddrV4,
}

impl MulticastBroadcaster {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        Self {
            addr: SocketAddrV4::new(ip, port),
        }
    }
}

#[async_trait::async_trait]
impl Broadcaster for MulticastBroadcaster {
    async fn run(&self, mut rx: broadcast::Receiver<AudioChunk>) -> Result<(), String> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.map_err(|e| e.to_string())?;
        println!("Broadcasting to {}...", self.addr);

        while let Ok(chunk) = rx.recv().await {
            // Check for any signal
            // let has_signal = chunk.iter().any(|&s| s.abs() > 0.0001);
            // if !has_signal {
            //     // Only print this once every few seconds so it doesn't spam
            //     // println!("DEBUG: Receiving silence..."); 
            // } else {
            //     println!("DEBUG: Signal detected! Sending packet...");
            // }

            // 1. Convert to i16 (PCM16) bytes
            let pcm_bytes: Vec<u8> = chunk
                .iter()
                .flat_map(|&s| ((s * i16::MAX as f32) as i16).to_le_bytes())
                .collect();

            // 2. CHUNK: Break the PCM data into ~1024 byte slices
            // This ensures we stay under the 1500 byte MTU limit
            for slice in pcm_bytes.chunks(1024) {
                if let Err(e) = socket.send_to(slice, self.addr).await {
                    eprintln!("UDP Send Error: {}", e);
                }
            }
        }
        Ok(())
    }
}