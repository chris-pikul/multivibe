use axum::Router;
use axum::extract::ws::WebSocket;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use multivibe_core::software::SoftwareSource;
use multivibe_core::network::multicast::MulticastBroadcaster;
use multivibe_core::network::websocket::WsBroadcaster;
use multivibe_core::{AudioChunk, AudioSource, Broadcaster};
use tokio::sync::broadcast;
use std::net::Ipv4Addr;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<AudioChunk>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, _) = broadcast::channel(64);
    let state = AppState { tx: tx.clone() };

    // 1. Start Audio Capture (Software Mode)
    #[cfg(feature = "software")]
    {
        let mut source = SoftwareSource::new("BlackHole")?;
        source.start(tx.clone())?;
    }

    // 2. Build the Web Server
    let app = Router::new()
        .route("/listen", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server active at http://localhost:3000/listen");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();
    while let Ok(chunk) = rx.recv().await {
        let msg = WsBroadcaster::chunk_to_message(chunk);
        if socket.send(msg).await.is_err() {
            break; // Neighbor disconnected
        }
    }
}
