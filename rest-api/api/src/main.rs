mod aeron_handler;
use aeron_handler::{build_context, create_publication, get_aeron_dir};

use aeron_rs::publication::Publication;

mod order;

mod routes;
use routes::{get_order, post_order};

mod errors;

use std::sync::{Arc, Mutex};
use std::time::Instant;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::{AlignedBuffer, AtomicBuffer};
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;

use axum::{Router, routing::get};

use log::{error, info};

pub struct AppState {
    publication: Arc<Mutex<Publication>>,
    buffer: AtomicBuffer,
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

    let publication = create_publication(&mut aeron, "aeron:ipc?endpoint=localhost:40123", 1001);
    let pub_status = publication.lock().unwrap().channel_status();
    info!("Aeron: Publication {}", channel_status_to_str(pub_status));

    let aligned_buffer = AlignedBuffer::with_capacity(72);

    let shared_state = Arc::new(AppState {
        publication,
        buffer: AtomicBuffer::from_aligned(&aligned_buffer),
    });

    let app = Router::new().route(
        "/api/v1/orders",
        get(get_order).post(post_order).with_state(shared_state),
    );

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8000").await {
        Ok(listener) => {
            info!("Server: Bound to 0.0.0.0:8000");
            listener
        }
        Err(e) => {
            error!("Server: Failed to bind to port 8000: {}", e);
            return;
        }
    };

    let startup_duration = start_time.elapsed();
    info!(
        "Startup complete in {:.2?} — ready to accept requests",
        startup_duration
    );

    if let Err(e) = axum::serve(listener, app).await {
        error!("Server failed: {}", e);
    }
}
