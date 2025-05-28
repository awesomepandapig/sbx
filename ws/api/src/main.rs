mod aeron_handler;
use aeron_handler::{build_context, create_subscription, get_aeron_dir};

mod messages;
use aeron_rs::subscription::Subscription;
use messages::read_message;

use std::sync::{Arc, Mutex};
use std::time::Instant;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::{AlignedBuffer, AtomicBuffer};
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;

use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;

use log::{error, info};

fn main() {
    env_logger::init();

    let start_time = Instant::now();

    let aeron_dir = get_aeron_dir();
    info!("Aeron: Using directory: {:?}", aeron_dir);

    let mut context = build_context(&aeron_dir);
    let mut aeron = match Aeron::new(context) {
        Ok(a) => {
            info!("Aeron: Instance created");
            a
        }
        Err(e) => {
            error!("Aeron: Failed to create instance: {:?}", e);
            return;
        }
    };

    let subscription = create_subscription(&mut aeron, "aeron:ipc?endpoint=localhost:40124", 1002);
    let sub_status = subscription.lock().unwrap().channel_status();
    info!("Aeron: Subscription {}", channel_status_to_str(sub_status));

    let aligned_buffer = AlignedBuffer::with_capacity(72);

    let startup_duration = start_time.elapsed();
    info!(
        "Startup complete in {:.2?} — ready to accept requests",
        startup_duration
    );

    loop {
        subscription.lock().unwrap().poll(
            &mut |buffer, offset, length, header| {
                read_message(buffer, offset, length, header);
            },
            200,
        );
    }
}
