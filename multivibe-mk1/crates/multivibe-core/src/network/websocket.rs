use crate::{AudioChunk, Broadcaster};
use tokio::sync::broadcast;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;

pub struct WsBroadcaster {
    // We use a simple list of active peer senders
    // For Mk.I, we'll let the Server handle the Axum routing, 
    // but the Core defines how we transform the chunks.
}

impl WsBroadcaster {
    /// Converts our f32 AudioChunk into a binary WebSocket message
    pub fn chunk_to_message(chunk: AudioChunk) -> Message {
        let pcm_bytes: Vec<u8> = chunk
            .iter()
            .flat_map(|&s| ((s * i16::MAX as f32) as i16).to_le_bytes())
            .collect();
        Message::Binary(pcm_bytes.into())
    }
}
