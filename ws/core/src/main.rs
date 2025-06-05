use std::{net::SocketAddr, sync::Arc, time::Instant};

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;

use futures_util::{SinkExt};
use log::{error, info};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast},
    time::{interval, Duration},
};
use tokio_tungstenite::{accept_async, tungstenite::Error};

mod sbe_messages;
use sbe_messages::read_message;

mod aeron_handler;
use aeron_handler::{build_context, create_subscription, get_aeron_dir};

type SharedSender = broadcast::Sender<String>;

async fn accept_connection(peer: SocketAddr, stream: TcpStream, tx: SharedSender) {
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
                    // Combine messages into a single payload (newline-delimited for example)
                    let batched = buffer.join("\n");

                    if ws_stream.send(
                        tokio_tungstenite::tungstenite::Message::Text(batched.into())
                    ).await.is_err() {
                        break; // client disconnected
                    }

                    buffer.clear();
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let start_time = Instant::now();

    let aeron_dir = get_aeron_dir();
    info!("Aeron: Using directory: {:?}", aeron_dir);

    let context = build_context(&aeron_dir);
    let mut aeron = match Aeron::new(context) {
        Ok(a) => {
            info!("Aeron: Instance created");
            a
        }
        Err(e) => {
            error!("Aeron: Failed to create instance: {:?}", e);
            return;
        }
    };

    let subscription = create_subscription(&mut aeron, "aeron:udp?endpoint=224.1.1.1:40456|interface=localhost", 1002);
    let sub_status = subscription.lock().unwrap().channel_status();
    info!("Aeron: Subscription {}", channel_status_to_str(sub_status));

    let (tx, _rx) = broadcast::channel::<String>(1024); // Shared buffer
    let sub_clone = Arc::clone(&subscription);
    let tx_clone = tx.clone();

    // Spawn background task for Aeron polling
    tokio::task::spawn_blocking(move || {
        let mut subscription = sub_clone.lock().unwrap();
        loop {
            subscription.poll(
                &mut |buffer, offset, length, header| {
                    // Here we parse and send to the channel
                    if let Some(message) = read_message(buffer, offset, length, header) {
                        // Send message to all subscribers
                        let _ = tx_clone.send(message);
                    }
                },
                256,
            );
            // std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    let startup_duration = start_time.elapsed();
    info!(
        "Startup complete in {:.2?} — ready to accept requests",
        startup_duration
    );

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        let tx = tx.clone();
        tokio::spawn(accept_connection(peer, stream, tx));
    }
}
