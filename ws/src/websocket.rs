use std::net::SocketAddr;

use futures_util::SinkExt;
use tokio::{
    net::TcpStream,
    sync::broadcast,
    time::{interval, Duration},
};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tracing::{error, info};

pub type SharedSender = broadcast::Sender<String>;

pub async fn accept_connection(peer: SocketAddr, stream: TcpStream, tx: SharedSender) {
    if let Err(e) = handle_connection(peer, stream, tx).await {
        if !matches!(
            e,
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8
        ) {
            error!("Error processing connection: {}", e);
        }
    }
}

async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    tx: SharedSender,
) -> Result<(), Error> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);

    let mut rx = tx.subscribe();
    let mut ticker = interval(Duration::from_millis(1));
    let mut buffer = Vec::with_capacity(16);

    loop {
        tokio::select! {
            Ok(msg) = rx.recv() => {
                buffer.push(msg);
            }

            _ = ticker.tick() => {
                if !buffer.is_empty() {
                    let batched = buffer.join("\n");

                    if ws_stream.send(
                        tokio_tungstenite::tungstenite::Message::Text(batched.into())
                    ).await.is_err() {
                        break;
                    }

                    buffer.clear();
                }
            }
        }
    }

    Ok(())
}