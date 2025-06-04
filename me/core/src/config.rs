use std::env;
use std::ffi::CString;
use std::process;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;

use aeron_rs::exclusive_publication::ExclusivePublication;
use aeron_rs::subscription::Subscription;
use aeron_rs::utils::errors::AeronError;

use tracing::{error, info, warn};

pub fn error_handler(error: AeronError) {
    match &error {
        AeronError::SubscriptionNotReady(id) => {
            warn!(subscription_id = *id, "Subscription not ready");
        }
        AeronError::PublicationNotReady(id) => {
            warn!(publication_id = *id, "Publication not ready");
        }
        AeronError::DriverTimeout(_)
        | AeronError::ClientTimeoutException
        | AeronError::RegistrationException(_, _)
        | AeronError::ChannelEndpointException(_, _) => {
            error!(?error, "Critical Aeron communication failure. Exiting.");
            process::exit(1);
        }
        _ => {
            error!(?error, "Unhandled Aeron error. Exiting.");
            process::exit(1);
        }
    }
}

pub fn on_new_subscription_handler(channel: CString, stream_id: i32, correlation_id: i64) {
    let channel_str = channel.to_string_lossy();
    info!(
        target: "aeron_callbacks",
        correlation_id,
        channel = %channel_str,
        stream_id,
        "New subscription successfully established."
    );
}

pub fn on_new_exclusive_publication_handler(
    channel: CString,
    stream_id: i32,
    session_id: i32,
    correlation_id: i64,
) {
    let channel_str = channel.to_string_lossy();
    info!(
        target: "aeron_callbacks",
        correlation_id,
        channel = %channel_str,
        stream_id,
        session_id,
        "New publication successfully established.",
    );
}

fn str_to_c(val: &str) -> CString {
    CString::new(val).unwrap_or_else(|e| {
        error!(input_string = %val, error = ?e, "Failed to convert string to CString. Input may contain null bytes.");
        panic!("Critical error: Failed to convert string '{}' to CString: {:?}", val, e);
    })
}

pub fn get_aeron_dir() -> String {
    let aeron_dir = match env::var("AERON_DIR") {
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
                let os_type = env::consts::OS;
                error!(
                    target: "configuration",
                    os = %os_type,
                    "AERON_DIR environment variable not set and no default path configured for this OS. Please set AERON_DIR. Supported OS for defaults: macOS, Linux. Exiting."
                );
                process::exit(1);
            }
        }
    };

    info!(target: "configuration", aeron_dir = %aeron_dir, "Using Aeron directory");
    aeron_dir
}

fn wait_for<T, F>(mut finder: F, label: &str, timeout_duration: Duration) -> T
where
    F: FnMut() -> Option<T>,
{
    let start = Instant::now();

    loop {
        if let Some(found) = finder() {
            return found;
        }

        if start.elapsed() > timeout_duration {
            error!(
                target: "aeron_setup",
                timeout_seconds = timeout_duration.as_secs_f64(),
                resource_label = %label,
                "Timed out waiting for Aeron resource to become ready. Ensure the Aeron media driver is running and configured correctly. Exiting."
            );
            process::exit(1);
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}

pub fn create_subscription(aeron: &mut Aeron) -> Arc<Mutex<Subscription>> {
    info!(target: "aeron_setup", "Creating Aeron subscription.");

    let channel = env::var("SUB_CHANNEL").unwrap_or_else(|e| {
        error!(target: "configuration", variable = "SUB_CHANNEL", error = ?e, "Required environment variable for subscription channel not set. Exiting.");
        process::exit(1);
    });
    let stream_id_str = env::var("SUB_STREAM_ID").unwrap_or_else(|e| {
        error!(target: "configuration", variable = "SUB_STREAM_ID", error = ?e, "Required environment variable for subscription stream ID not set. Exiting.");
        process::exit(1);
    });
    let stream_id = stream_id_str.parse::<i32>().unwrap_or_else(|e| {
        error!(target: "configuration", variable = "SUB_STREAM_ID", value = %stream_id_str, error = ?e, "Failed to parse SUB_STREAM_ID as i32. Exiting.");
        process::exit(1);
    });

    info!(target: "aeron_setup", channel = %channel, stream_id = stream_id, "Attempting to add subscription.");
    let c_channel = str_to_c(&channel);
    let subscription_id: i64 = match aeron.add_subscription(c_channel, stream_id) {
        Ok(id) => id,
        Err(err) => {
            error!(target: "aeron_setup", kind="add_subscription_failed", error=?err, channel=%channel, stream_id=stream_id, "Failed to add Aeron subscription. See Aeron error details above/below. Exiting.");
            error_handler(err);
            unreachable!("error_handler should have exited");
        }
    };
    info!(target: "aeron_setup", subscription_id, "Subscription added, waiting for it to become available.");

    let subscription = wait_for(
        || aeron.find_subscription(subscription_id).ok(),
        &format!(
            "subscription (id: {}) on channel '{}'",
            subscription_id, channel
        ),
        Duration::from_secs(15),
    );

    let sub_guard = subscription.lock().unwrap_or_else(|poisoned_err| {
        error!(target: "concurrency_error", resource = "subscription_mutex", id = subscription_id, error = %poisoned_err, "Mutex for subscription is poisoned. Exiting.");
        process::exit(1);
    });
    let channel_status = sub_guard.channel_status();
    info!(target: "aeron_status", kind = "subscription_channel_status", status = %channel_status_to_str(channel_status), subscription_id = sub_guard.registration_id(), "Subscription channel status.");
    drop(sub_guard);

    info!(target: "aeron_setup", "Aeron subscription created.");
    subscription
}

pub fn create_exclusive_publication(aeron: &mut Aeron) -> Arc<Mutex<ExclusivePublication>> {
    info!(target: "aeron_setup", "Creating Aeron publication.");

    let channel = env::var("PUB_CHANNEL").unwrap_or_else(|e| {
        error!(target: "configuration", variable = "PUB_CHANNEL", error = ?e, "Required environment variable for publication channel not set. Exiting.");
        process::exit(1);
    });
    let stream_id_str = env::var("PUB_STREAM_ID").unwrap_or_else(|e| {
        error!(target: "configuration", variable = "PUB_STREAM_ID", error = ?e, "Required environment variable for publication stream ID not set. Exiting.");
        process::exit(1);
    });
    let stream_id = stream_id_str.parse::<i32>().unwrap_or_else(|e| {
        error!(target: "configuration", variable = "PUB_STREAM_ID", value = %stream_id_str, error = ?e, "Failed to parse PUB_STREAM_ID as i32. Exiting.");
        process::exit(1);
    });

    info!(target: "aeron_setup", channel = %channel, stream_id = stream_id, "Attempting to add exclusive publication.");
    let c_channel = str_to_c(&channel); // str_to_c will exit on failure
    let publication_id: i64 = match aeron.add_exclusive_publication(c_channel, stream_id) {
        Ok(id) => id,
        Err(err) => {
            error!(target: "aeron_setup", kind="add_publication_failed", error=?err, channel=%channel, stream_id=stream_id, "Failed to add Aeron exclusive publication. See Aeron error details above/below. Exiting.");
            error_handler(err); // error_handler will now also exit
            unreachable!("error_handler should have exited"); // Should not be reached if error_handler exits
        }
    };
    info!(target: "aeron_setup", publication_id, "Exclusive publication added, waiting for it to become available.");

    let publication = wait_for(
        // wait_for will exit on timeout
        || aeron.find_exclusive_publication(publication_id).ok(),
        &format!(
            "publication (id: {}) on channel '{}'",
            publication_id, channel
        ),
        Duration::from_secs(15),
    );

    let pub_guard = publication.lock().unwrap_or_else(|poisoned_err| {
        error!(target: "concurrency_error", resource = "publication_mutex", id = publication_id, error = %poisoned_err, "Mutex for publication is poisoned. Exiting.");
        process::exit(1);
    });
    let channel_status = pub_guard.channel_status();
    info!(target: "aeron_status", kind = "publication_channel_status", status = %channel_status_to_str(channel_status), publication_id = pub_guard.registration_id(), "Publication channel status.");
    drop(pub_guard);

    info!(target: "aeron_setup", "Aeron publication created.");

    publication
}
