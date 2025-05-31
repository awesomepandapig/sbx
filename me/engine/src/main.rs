mod orderbook;
mod publisher;

use orderbook::OrderBook;
use publisher::ExecutionReportPublisher;

use std::env;
use std::ffi::CString;
use std::slice;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;
use aeron_rs::concurrent::strategies::{BusySpinIdleStrategy, Strategy};
use aeron_rs::context::Context;
use aeron_rs::utils::errors::AeronError;
use aeron_rs::utils::types::Index;

use sbe::ReadBuf;
use sbe::message_header_codec::MessageHeaderDecoder;

// TODO: Move to a config file
fn get_aeron_dir() -> String {
    match env::var("AERON_DIR") {
        Ok(path_str) => path_str,
        Err(_) => {
            #[cfg(target_os = "macos")]
            {
                "/Volumes/DevShm/aeron".to_string()
            }
            #[cfg(target_os = "linux")]
            {
                "/dev/shm/aeron"
            }
            #[cfg(not(any(target_os = "macos", target_os = "linux")))]
            {
                // Fallback for other OSes:
                // Option 1: Panic, similar to your original .expect()
                panic!(
                    "AERON_DIR environment variable not set and no default path configured for this OS ({}). \
                    Please set AERON_DIR. Supported OS for defaults: macOS, Linux.",
                    env::consts::OS
                );

                // Option 2: Provide a generic default or an empty path, and handle it later
                // e.g., return an Option<PathBuf> or Result<PathBuf, String> from this function
                // For now, we'll stick to the panic approach to match your original .expect() style.
            }
        }
    }
}

fn error_handler(error: AeronError) {
    println!("Error: {:?}", error);
}

fn on_new_publication_handler(
    channel: CString,
    stream_id: i32,
    session_id: i32,
    correlation_id: i64,
) {
    println!(
        "Publication: {} {} {} {}",
        channel.to_str().unwrap(),
        stream_id,
        session_id,
        correlation_id
    );
}

fn on_new_subscription_handler(channel: CString, stream_id: i32, correlation_id: i64) {
    println!(
        "Subscription: {} {} {}",
        channel.to_str().unwrap(),
        stream_id,
        correlation_id
    );
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
            order_book.process_new_order(header_decoder);
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
    let aeron_dir: String = get_aeron_dir();
    println!("Using Aeron directory: {:?}", aeron_dir);

    let mut context: Context = Context::new();

    context.set_aeron_dir(aeron_dir);
    context.set_new_publication_handler(Box::new(on_new_publication_handler));
    context.set_new_subscription_handler(Box::new(on_new_subscription_handler));
    context.set_error_handler(Box::new(error_handler));
    context.set_pre_touch_mapped_memory(true);

    let mut aeron: Aeron = Aeron::new(context).expect("Error creating Aeron instance");

    // ----- initialize subscription -------------------------------------------
    // TODO:let channel = std::env::var("CHANNEL").expect("CHANNEL environment variable not set");
    let subscription_channel: String = String::from("aeron:ipc?endpoint=localhost:40123");
    // TODO: let stream_id = std::env::var("STREAM_ID").expect("STREAM_ID environment variable not set");
    let subscription_stream_id: i32 = "1001".parse().unwrap();

    let subscription_id: i64 = aeron
        .add_subscription(str_to_c(&subscription_channel), subscription_stream_id)
        .expect("Error adding subscription");

    let subscription = loop {
        if let Ok(subscription) = aeron.find_subscription(subscription_id) {
            break subscription;
        }
        std::thread::yield_now();
    };

    let channel_status = subscription.lock().unwrap().channel_status();

    println!("Subscription: {}", channel_status_to_str(channel_status));
    // -------------------------------------------------------------------------

    // ----- initialize publication --------------------------------------------
    // TODO:let channel = std::env::var("CHANNEL").expect("CHANNEL environment variable not set");
    let publication_channel: String = String::from("aeron:ipc?endpoint=localhost:40124");
    // TODO: let stream_id = std::env::var("STREAM_ID").expect("STREAM_ID environment variable not set");
    let publication_stream_id: i32 = "1002".parse().unwrap();

    let publication_id: i64 = aeron
        .add_exclusive_publication(str_to_c(&publication_channel), publication_stream_id)
        .expect("Error adding publication");

    let publication = loop {
        if let Ok(publication) = aeron.find_exclusive_publication(publication_id) {
            break publication;
        }
        std::thread::yield_now();
    };
    let channel_status = publication.lock().unwrap().channel_status();

    println!("Publication: {}", channel_status_to_str(channel_status));
    // -------------------------------------------------------------------------

    let publisher = ExecutionReportPublisher::new(publication);
    let mut orderbook = OrderBook::new(publisher);

    let poll_idle_strategy = BusySpinIdleStrategy::default();

    let mut subscription = subscription.lock().unwrap();
    loop {
        let fragments_read = subscription.poll(
            &mut |buffer, offset, length, header| {
                read_order(&mut orderbook, buffer, offset, length, header);
            },
            256,
        );
        poll_idle_strategy.idle_opt(fragments_read);
    }
}
