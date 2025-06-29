use std::env;
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread::yield_now;
use std::process;

use aeron_rs::aeron::Aeron;
use aeron_rs::context::Context;
use aeron_rs::publication::Publication;
use aeron_rs::utils::errors::AeronError;

use log::{info, error};

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


pub fn unwrap_publication(
    wrapped_pub: Arc<Mutex<Publication>>,
) -> Publication {
    // Attempt to get the Mutex out of the Arc.
    // This will only succeed if the strong count is 1.
    let mutex = Arc::try_unwrap(wrapped_pub).unwrap_or_else(|_| {
        error!("Could not unwrap Arc for Publication, still has multiple owners. This should not happen in the matching engine. Exiting.");
        process::exit(1);
    });

    // Get the Publication out of the Mutex.
    // This consumes the mutex, returning the inner data.
    mutex.into_inner().unwrap_or_else(|poison_err| {
        error!(
            "Mutex for Publication is poisoned. Exiting. Error: {}",
            poison_err
        );
        process::exit(1);
    })
}