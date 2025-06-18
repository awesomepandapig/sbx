mod config;
mod handler;
mod orderbook;
mod publisher;
mod side;
mod types;

use config::{
    create_exclusive_publication, create_subscription, error_handler, get_aeron_dir,
    on_new_exclusive_publication_handler, on_new_subscription_handler,
};

use handler::Handler;
use publisher::Publisher;

use std::collections::VecDeque;
use std::process;
use std::slice;
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::concurrent::strategies::{BusySpinIdleStrategy, Strategy};
use aeron_rs::context::Context;
use aeron_rs::exclusive_publication::ExclusivePublication;
use aeron_rs::subscription::Subscription;
use aeron_rs::fragment_assembler::FragmentAssembler;
use aeron_rs::utils::types::Index;

use sbe::ReadBuf;
use sbe::message_header_codec::MessageHeaderDecoder;

use tracing::Level;
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;

use tracing::{error, info};

struct LatencyMetrics {
    total_messages: u64,
    total_latency_sum_ns: u128, // Use u128 for sum to prevent overflow with many messages
    recent_latencies_ns: VecDeque<u64>,
    window_size: usize,
}

impl LatencyMetrics {
    fn new(window_size: usize) -> Self {
        LatencyMetrics {
            total_messages: 0,
            total_latency_sum_ns: 0,
            recent_latencies_ns: VecDeque::with_capacity(window_size),
            window_size,
        }
    }

    fn add_latency(&mut self, latency_ns: u64) {
        self.total_messages += 1;
        self.total_latency_sum_ns += u128::from(latency_ns);

        self.recent_latencies_ns.push_back(latency_ns);
        if self.recent_latencies_ns.len() > self.window_size {
            self.recent_latencies_ns.pop_front();
        }
    }

    fn get_total_average(&self) -> f64 {
        if self.total_messages == 0 {
            0.0
        } else {
            self.total_latency_sum_ns as f64 / self.total_messages as f64
        }
    }

    fn get_recent_average(&self) -> f64 {
        if self.recent_latencies_ns.is_empty() {
            0.0
        } else {
            let sum: u64 = self.recent_latencies_ns.iter().sum();
            sum as f64 / self.recent_latencies_ns.len() as f64
        }
    }
}

/// Unwraps an ExclusivePublication from its Arc<Mutex<>> container.
///
/// This is safe to call only when we are sure that we hold the only
/// strong reference to the Arc, which is true in our single-threaded
/// matching engine context right after creation.
fn unwrap_exclusive_publication(
    wrapped_pub: Arc<Mutex<ExclusivePublication>>,
) -> ExclusivePublication {
    // Attempt to get the Mutex out of the Arc.
    // This will only succeed if the strong count is 1.
    let mutex = Arc::try_unwrap(wrapped_pub).unwrap_or_else(|_| {
        error!("Could not unwrap Arc for ExclusivePublication, still has multiple owners. This should not happen in the matching engine. Exiting.");
        process::exit(1);
    });

    // Get the ExclusivePublication out of the Mutex.
    // This consumes the mutex, returning the inner data.
    mutex.into_inner().unwrap_or_else(|poison_err| {
        error!(
            "Mutex for ExclusivePublication is poisoned. Exiting. Error: {}",
            poison_err
        );
        process::exit(1);
    })
}

/// Unwraps a Subscription from its Arc<Mutex<>> container.
fn unwrap_subscription(wrapped_sub: Arc<Mutex<Subscription>>) -> Subscription {
    let mutex = Arc::try_unwrap(wrapped_sub).unwrap_or_else(|_| {
        error!("Could not unwrap Arc for Subscription, still has multiple owners. This should not happen in the matching engine. Exiting.");
        process::exit(1);
    });
    mutex.into_inner().unwrap_or_else(|poison_err| {
        error!(
            "Mutex for Subscription is poisoned. Exiting. Error: {}",
            poison_err
        );
        process::exit(1);
    })
}

fn main() -> ! {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    set_global_default(subscriber).unwrap_or_else(|err| {
        error!(target: "setup", kind="tracing_init_failed", error=?err, "Failed to create Tracing subscriber"); // TODO: Should this be a span?
        process::exit(1);
    });

    info!(target: "aeron_setup", "Initializing Aeron context.");
    let mut context: Context = Context::new();
    context.set_aeron_dir(get_aeron_dir());
    context.set_new_exclusive_publication_handler(Box::new(on_new_exclusive_publication_handler));
    context.set_new_subscription_handler(Box::new(on_new_subscription_handler));
    context.set_error_handler(Box::new(error_handler));
    context.set_pre_touch_mapped_memory(true);

    let mut aeron = match Aeron::new(context) {
        Ok(client) => client,
        Err(err) => {
            error!(
                target: "aeron_setup",
                kind = "aeron_init_failed",
                error = ?err,
                "Failed to create Aeron client instance."
            );
            error_handler(err);
            process::exit(1);
        }
    };

    let wrapped_publication = create_exclusive_publication(&mut aeron);
    let publication = unwrap_exclusive_publication(wrapped_publication);

    let wrapped_subscription = create_subscription(&mut aeron);
    let mut subscription = unwrap_subscription(wrapped_subscription);

    let publisher = Publisher::new(publication);
    let mut handler = Handler::new(publisher);

    let poll_idle_strategy = BusySpinIdleStrategy {};

    let metrics = Arc::new(Mutex::new(LatencyMetrics::new(100_000)));
    let metrics_clone = Arc::clone(&metrics); // Clone for the closure

    let mut order_message_handler =
        move |buffer: &AtomicBuffer, offset: Index, length: Index, _header: &Header| {
            let base_instant = Instant::now();

            // SAFETY: This creates a slice from the Aeron buffer for zero-copy message processing.
            // The buffer is guaranteed to be valid for the specified offset and length by Aeron.
            // The slice lifetime is bounded by this function scope, ensuring memory safety.
            let slice_msg = unsafe {
                slice::from_raw_parts_mut(
                    buffer.buffer().offset(offset.try_into().expect("")), // TODO: NO EXPECT
                    length.try_into().expect(""),                         // TODO: NO EXPECT
                )
            };

            let read_buf = ReadBuf::new(slice_msg);
            let header_decoder: MessageHeaderDecoder<ReadBuf<'_>> =
                MessageHeaderDecoder::default().wrap(read_buf, 0);
            match header_decoder.template_id() {
                1 => {
                    handler.process_new_order(header_decoder);
                    let now_instant = Instant::now();
                    let delta_ns = now_instant.duration_since(base_instant).as_nanos() as u64;
                    
                    // Acquire lock, update metrics, and print averages
                    let mut metrics_guard = metrics_clone.lock().unwrap_or_else(|poisoned| {
                        error!("LatencyMetrics mutex poisoned: {:?}", poisoned);
                        process::exit(1); // Exit if the mutex is poisoned, as state is corrupted
                    });
                    metrics_guard.add_latency(delta_ns);

                    // Print statistics periodically, e.g., every 1000 messages or so
                    if metrics_guard.total_messages % 1000 == 0 {
                        info!(
                            target: "matching_engine",
                            total_messages = metrics_guard.total_messages,
                            last_delta_ns = delta_ns,
                            recent_avg_ns = metrics_guard.get_recent_average(),
                            total_avg_ns = metrics_guard.get_total_average(),
                            "Processing latency metrics"
                        );
                    }
                }
                2 => {
                    handler.process_cancel_order(header_decoder);
                }
                unknown_id => {
                    error!(
                        target: "matching_engine",
                        template_id = unknown_id,
                        "Unknown message template ID received, rejecting message"
                    );
                }
            }
        };

    let mut fragment_assembler = FragmentAssembler::new(&mut order_message_handler, None);
    let mut fragment_handler = fragment_assembler.handler();

    loop {
        let fragments_read = subscription.poll(&mut fragment_handler, 10);
        poll_idle_strategy.idle_opt(fragments_read);
    }
}
