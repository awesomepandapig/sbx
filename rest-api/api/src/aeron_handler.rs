use std::env;
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread::yield_now;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;
use aeron_rs::context::Context;
use aeron_rs::publication::Publication;
use aeron_rs::subscription::Subscription;
use aeron_rs::utils::errors::AeronError;

use log::{error, info};

pub fn get_aeron_dir() -> String {
    env::var("AERON_DIR").unwrap_or_else(|_| {
        #[cfg(target_os = "macos")]
        return "/Volumes/DevShm/aeron".to_string();

        #[cfg(target_os = "linux")]
        return "/dev/shm/aeron".to_string();

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        panic!(
            "AERON_DIR not set and no default available for this OS ({})",
            env::consts::OS
        );
    })
}

pub fn build_context(aeron_dir: &str) -> Context {
    let mut context = Context::new();

    context.set_aeron_dir(aeron_dir.to_string());
    context.set_new_publication_handler(Box::new(on_new_publication_handler));
    context.set_new_subscription_handler(Box::new(on_new_subscription_handler));
    context.set_error_handler(Box::new(error_handler));
    context.set_pre_touch_mapped_memory(true);

    context
}

pub fn create_publication(
    aeron: &mut Aeron,
    channel: &str,
    stream_id: i32,
) -> Arc<Mutex<Publication>> {
    let publication_id = aeron
        .add_publication(str_to_c(channel), stream_id)
        .expect("Error adding publication");

    loop {
        if let Ok(publication) = aeron.find_publication(publication_id) {
            return publication;
        }
        yield_now();
    }
}

pub fn create_subscription(
    aeron: &mut Aeron,
    channel: &str,
    stream_id: i32,
) -> Arc<Mutex<Subscription>> {
    let subscription_id = aeron
        .add_subscription(str_to_c(channel), stream_id)
        .expect("Error adding subscription");

    loop {
        if let Ok(subscription) = aeron.find_subscription(subscription_id) {
            return subscription;
        }
        yield_now();
    }
}

fn str_to_c(val: &str) -> CString {
    CString::new(val).expect("Error converting str to CString")
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
    info!(
        "Aeron: Created Subscription — Channel: {}, StreamID: {}, SessionID: {}, CorrelationID: {}",
        channel.to_str().unwrap(),
        stream_id,
        session_id,
        correlation_id
    );
}

fn on_new_subscription_handler(channel: CString, stream_id: i32, correlation_id: i64) {
    info!(
        "Aeron: Created Subscription — Channel: {}, StreamID: {}, CorrelationID: {}",
        channel.to_str().unwrap(),
        stream_id,
        correlation_id
    );
}
