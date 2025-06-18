mod messages;
mod orderbook;
mod processors;
mod transport;

use messages::decode_execution_report;
use orderbook::OrderBook;
use processors::execution::process_execution_report;
use transport::aeron::{build_context, create_subscription, get_aeron_dir};

use std::process;
use std::slice;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;
use aeron_rs::concurrent::strategies::{BusySpinIdleStrategy, Strategy};
use aeron_rs::fragment_assembler::FragmentAssembler;
use aeron_rs::utils::types::Index;

use sbe::ReadBuf;
use sbe::message_header_codec::MessageHeaderDecoder;

use tokio::sync::mpsc;

use tracing::subscriber::set_global_default;
use tracing::{Level, error, info, warn};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    set_global_default(subscriber).unwrap_or_else(|err| {
        error!(target: "setup", kind="tracing_init_failed", error=?err, "Failed to create Tracing subscriber"); // TODO: Should this be a span?
        process::exit(1);
    });

    // --- Aeron and NATS/JetStream Setup ---
    let aeron_dir = get_aeron_dir();
    let context = build_context(&aeron_dir);
    let mut aeron = match Aeron::new(context) {
        Ok(instance) => instance,
        Err(e) => {
            error!("Aeron: Failed to create instance: {:?}", e);
            return;
        }
    };
    // TODO: GET AERON_SUB_CHANNEL FROM ENV VARIABLE
    let subscription = create_subscription(
        &mut aeron,
        //"aeron:udp?endpoint=224.1.1.1:40456|interface=localhost",
        "aeron:ipc",
        1002,
    );
    let sub_status = subscription.lock().unwrap().channel_status();
    info!("Aeron: Subscription {}", channel_status_to_str(sub_status));

    // Connect to the NATS server
    // TODO: GET NATS_SERVER FROM ENV VARIABLE
    info!("Connecting to NATS server at localhost");
    let client = async_nats::connect("localhost")
        .await
        .expect("nats connection failed"); // TODO: Replace expect with explicit handle and trace

    let (tx, mut rx) = mpsc::channel::<String>(1024);

    tokio::spawn(async move {
        info!("Async I/O publisher task started.");
        while let Some(message) = rx.recv().await {
            // TODO: GET SUBJECT FROM HARDCODED VALUE (DERIVED FROM ENV VAR AT STARTUP)
            client
                .publish("foo", message.into())
                .await
                .expect("JetStream publish failed"); // TODO: Add proper error handling/flow control
        }
    });

    let mut book = OrderBook::new();

    // Initialize message handler
    let mut order_message_handler =
        move |buffer: &AtomicBuffer, offset: Index, length: Index, _header: &Header| {
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
                3 => {
                    let report = decode_execution_report(header_decoder);
                    if let Some(message) = process_execution_report(&mut book, &report) {
                        if let Err(e) = tx.try_send(message) {
                            // TODO: NOTE: For true lossless, you would implement the "pending_report"
                            // backpressure logic here, but using the String instead.
                        }
                    }
                }
                template_id => {
                    panic!("incorrect template_id: {}", template_id)
                }
            }
        };

    let mut fragment_assembler = FragmentAssembler::new(&mut order_message_handler, None);
    let mut fragment_handler = fragment_assembler.handler();
    let poll_idle_strategy = BusySpinIdleStrategy::default();

    info!("Starting Aeron polling loop...");

    loop {
        let fragments_read = subscription.lock().unwrap().poll(&mut fragment_handler, 10);
        poll_idle_strategy.idle_opt(fragments_read);
    }
}
