mod messages;
mod orderbook;
mod processors;
mod transport;

use messages::decode_execution_report;
use processors::execution::process_execution_report;

use orderbook::OrderBook;
use transport::aeron::{build_context, create_subscription, get_aeron_dir};

use std::option::Option::None;
use std::slice;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;
use aeron_rs::concurrent::strategies::BusySpinIdleStrategy;
use aeron_rs::concurrent::strategies::Strategy;
use aeron_rs::fragment_assembler::FragmentAssembler;
use aeron_rs::utils::types::Index;

use sbe::ReadBuf;
use sbe::message_header_codec::MessageHeaderDecoder;

use tracing::Level;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // Initialize tracing
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    // Initialize Aeron
    let aeron_dir = get_aeron_dir();
    let context = build_context(&aeron_dir);
    let mut aeron = match Aeron::new(context) {
        Ok(instance) => instance,
        Err(e) => {
            error!("Aeron: Failed to create instance: {:?}", e);
            return;
        }
    };
    let subscription = create_subscription(
        &mut aeron,
        // "aeron:udp?endpoint=224.1.1.1:40456|interface=localhost",
        "aeron:ipc",
        1002,
    );
    let sub_status = subscription.lock().unwrap().channel_status();
    info!("Aeron: Subscription {}", channel_status_to_str(sub_status));

    // Initialize structs
    let mut book = OrderBook::new();
    let poll_idle_strategy = BusySpinIdleStrategy::default();

    // TODO: Move out into handler function
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

                    process_execution_report(&mut book, &report);
                }
                template_id => {
                    panic!("incorrect template_id: {}", template_id)
                }
            }
        };

    // Listen for new Aeron messages
    let mut fragment_assembler = FragmentAssembler::new(&mut order_message_handler, None);
    let mut fragment_handler = fragment_assembler.handler();
    loop {
        let fragments_read = subscription.lock().unwrap().poll(&mut fragment_handler, 10);
        poll_idle_strategy.idle_opt(fragments_read);
    }
}
