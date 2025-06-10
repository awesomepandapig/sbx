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

use std::process;
use std::slice;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::concurrent::strategies::{BusySpinIdleStrategy, Strategy};
use aeron_rs::context::Context;
use aeron_rs::fragment_assembler::FragmentAssembler;
use aeron_rs::utils::types::Index;

use sbe::ReadBuf;
use sbe::message_header_codec::MessageHeaderDecoder;

use tracing::Level;
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;

use tracing::{error, info};

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

    let publication = create_exclusive_publication(&mut aeron);
    let subscription = create_subscription(&mut aeron);

    let publisher = Publisher::new(publication);
    let mut handler = Handler::new(publisher);

    let poll_idle_strategy = BusySpinIdleStrategy::default();

    let mut subscription_guard = subscription.lock().unwrap(); // TODO: EXPLICTLY HANDLE THIS ERROR

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
                1 => {
                    handler.process_new_order(header_decoder);
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
        let fragments_read = subscription_guard.poll(&mut fragment_handler, 10);
        poll_idle_strategy.idle_opt(fragments_read);
    }
}
