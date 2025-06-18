use futures_util::{SinkExt, StreamExt};

use std::net::SocketAddr;
use std::process;
use std::time::SystemTime;

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result, Utf8Bytes},
};
use tokio::time::{interval, Duration, Instant};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use serde_json::{Value, json};

use tracing::Level;
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;

use tracing::{error, info};

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8(_) => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

pub fn format_timestamp_ns(timestamp_ns: u64) -> String {
    let secs = (timestamp_ns / 1_000_000_000) as i64;
    let nanos = (timestamp_ns % 1_000_000_000) as u32;

    let naive_dt = NaiveDateTime::from_timestamp_opt(secs, nanos)
        .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()); // Default to epoch if out of range

    let datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive_dt);
    datetime.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    info!("New WebSocket connection: {}", peer);

    // Connect to the NATS server
    // TODO: GET NATS_SERVER FROM ENV VARIABLE
    info!("Connecting to NATS server at localhost");
    let client = async_nats::connect("localhost")
        .await
        .expect("nats connection failed"); // TODO: Replace expect with explicit handle and trace

    // TODO: GET NATS_CHANNEL FROM ENV VARIABLE
    let mut subscriber = client
        .subscribe("foo")
        .await
        .expect("Failed to connect to NATS");

    let mut sequence_num = 0;
    let mut buffer: Vec<Value> = Vec::new();
    let mut ticker = interval(Duration::from_millis(250));

    while let Some(message) = subscriber.next().await {
        let inner_json: Value =
            serde_json::from_slice(&message.payload).expect("Invalid JSON in NATS message payload"); // TODO: HANDLE ERROR

        let timestamp_ns = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap() // TODO: REPLACE UNWRAP WITH ERROR HANDLE
            .as_nanos() as u64;

        let update_json = json!({
            "channel": "l2_data",
            "client_id": "", // TODO:
            "timestamp": format_timestamp_ns(timestamp_ns),
            "sequence_num": sequence_num,
            "events": [{
                "type": "update",
                "product_id": "JSP",
                "updates": [inner_json]
            }]
        });

        sequence_num += 1;

        let update_str = update_json.to_string();
        let ws_msg = Message::Text(Utf8Bytes::from(update_str));
        ws_stream
            .send(ws_msg)
            .await
            .expect("Failed to send message");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    set_global_default(subscriber).unwrap_or_else(|err| {
        error!(target: "setup", kind="tracing_init_failed", error=?err, "Failed to create Tracing subscriber");
        process::exit(1);
    });

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }
}
