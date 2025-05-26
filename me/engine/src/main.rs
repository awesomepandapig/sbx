mod orderbook;
use orderbook::OrderBook;

use std::ffi::CString;
use std::slice;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;
use aeron_rs::context::Context;
use aeron_rs::example_config::{DEFAULT_CHANNEL, DEFAULT_STREAM_ID};
use aeron_rs::utils::errors::AeronError;
use aeron_rs::utils::types::Index;

use sbe::message_header_codec::MessageHeaderDecoder;
use sbe::*;

fn error_handler(error: AeronError) {
    println!("Error: {:?}", error);
}

fn read_order(
    order_book: &mut OrderBook,
    buffer: &AtomicBuffer,
    offset: Index,
    length: Index,
    _header: &Header,
) {
    let slice_msg = unsafe {
        slice::from_raw_parts_mut(buffer.buffer().offset(offset as isize), length as usize)
    };
    let read_buf = ReadBuf::new(slice_msg);
    let header_decoder: MessageHeaderDecoder<ReadBuf<'_>> =
        MessageHeaderDecoder::default().wrap(read_buf, 0);
    match header_decoder.template_id() {
        1 => {
            order_book.create_order(header_decoder);
        }
        2 => {
            // TODO: order_book.cancel_order(header_decoder);
        }
        _ => {
            panic!("Unknown templateId");
        }
    }
}

fn str_to_c(val: &str) -> CString {
    CString::new(val).expect("Error converting str to CString")
}

fn main() {
    // let channel = std::env::var("CHANNEL").expect("CHANNEL environment variable not set");
    let channel: String = String::from(DEFAULT_CHANNEL);
    // let stream_id = std::env::var("STREAM_ID").expect("STREAM_ID environment variable not set");
    let stream_id: i32 = DEFAULT_STREAM_ID.parse().unwrap();
    let aeron_dir: String =
        std::env::var("AERON_DIR").expect("AERON_DIR environment variable not set");

    let mut context: Context = Context::new();
    context.set_aeron_dir(aeron_dir);
    context.set_new_subscription_handler(Box::new(
        |channel: CString, stream_id: i32, correlation_id: i64| {
            println!(
                "Subscription: {} {} {}",
                channel.to_str().unwrap(),
                stream_id,
                correlation_id
            )
        },
    ));
    context.set_error_handler(Box::new(error_handler));
    context.set_pre_touch_mapped_memory(true);

    let mut aeron: Aeron = Aeron::new(context).expect("Error creating Aeron instance");

    let subscription_id: i64 = aeron
        .add_subscription(str_to_c(&channel), stream_id)
        .expect("Error adding subscription");

    let subscription = loop {
        if let Ok(subscription) = aeron.find_subscription(subscription_id) {
            break subscription;
        }
        std::thread::yield_now();
    };

    let channel_status = subscription.lock().unwrap().channel_status();

    println!("Subscription: {}", channel_status_to_str(channel_status));

    let mut orderbook = OrderBook::new();

    loop {
        subscription.lock().unwrap().poll(
            &mut |buffer, offset, length, header| {
                read_order(&mut orderbook, buffer, offset, length, header);
            },
            10,
        );
    }
}
