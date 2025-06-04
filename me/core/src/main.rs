mod config;
mod orderbook;
mod publisher;

use config::{create_exclusive_publication, create_subscription, error_handler, get_aeron_dir, on_new_exclusive_publication_handler, on_new_subscription_handler};
use orderbook::OrderBook;
use publisher::ExecutionReportPublisher;

use std::process;
use std::slice;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::concurrent::strategies::{BusySpinIdleStrategy, Strategy};
use aeron_rs::context::Context;
use aeron_rs::utils::types::Index;

use sbe::ReadBuf;
use sbe::message_header_codec::MessageHeaderDecoder;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use tracing::subscriber::set_global_default;

use tracing::{debug, error, info};

fn read_order(
    order_book: &mut OrderBook,
    buffer: &AtomicBuffer,
    offset: Index,
    length: Index,
    _header: &Header,
) {
    debug!(target: "aeron_debug", message="Attempting to read a new aeron message");

    // TODO: Safety comment (idk if this is actually safe may just be necessary for zero copy)
    // SAFETY: lorem ipsum dolor
    let slice_msg = unsafe {
        slice::from_raw_parts_mut(buffer.buffer().offset(offset.try_into().expect("")), length.try_into().expect(""))
    };

    let read_buf = ReadBuf::new(slice_msg);
    let header_decoder: MessageHeaderDecoder<ReadBuf<'_>> =
        MessageHeaderDecoder::default().wrap(read_buf, 0);
    match header_decoder.template_id() {
        1 => {
            order_book.process_new_order(header_decoder);
        }
        2 => {
            // TODO: order_book.cancel_order(header_decoder);
        }
        unknown_id => {
            error!(
                target: "matching_engine",
                template_id = unknown_id,
                "Unknown message template ID received, rejecting message"
            );
        }
    }
}

fn main() -> ! {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    set_global_default(subscriber).unwrap_or_else(|err| {
        error!(target: "setup", kind="tracing_init_failed", error=?err, "Failed to create Tracing subscriber"); // TODO: Should this be a span?
        process::exit(1);
    }); // TODO: 

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

    let publisher = ExecutionReportPublisher::new(publication);
    let mut orderbook = OrderBook::new(publisher);

    let poll_idle_strategy = BusySpinIdleStrategy::default();

    let mut subscription_guard = subscription.lock().unwrap(); // TODO: EXPLICTLY HANDLE THIS ERROR
    loop {
        let fragments_read = subscription_guard.poll(
            &mut |buffer, offset, length, header| {
                read_order(&mut orderbook, buffer, offset, length, header);
            },
            256,
        );
        poll_idle_strategy.idle_opt(fragments_read);
    }
}
