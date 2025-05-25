use std::ffi::CString;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::{AlignedBuffer, AtomicBuffer};
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;
use aeron_rs::context::Context;
use aeron_rs::example_config::{DEFAULT_CHANNEL, DEFAULT_STREAM_ID};
use aeron_rs::utils::errors::AeronError;

use sbe::new_order_single_codec::NewOrderSingleEncoder;
use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;
use sbe::*;

use uuid::Uuid;
use rand::Rng;

#[derive(Clone)]
struct Settings {
    dir_prefix: String,
    channel: String,
    stream_id: i32,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            dir_prefix: std::env::var("AERON_DIR").unwrap_or_else(|_| String::new()),
            channel: String::from(DEFAULT_CHANNEL),
            stream_id: DEFAULT_STREAM_ID.parse().unwrap(),
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

fn str_to_c(val: &str) -> CString {
    CString::new(val).expect("Error converting str to CString")
}

pub fn create_order(
    cl_ord_id: &[u8; 16],
    party_id: &[u8; 16],
    symbol: &[u8; 6],
    side: SideEnum,
    ord_type: OrdTypeEnum,
    timestamp_ns: u64,
    qty_mantissa: i64,
    price_mantissa: i64,
) -> [u8; 72] {
    let mut buffer = [0u8; 72]; // 8 byte aeron header + 64 byte body
    let write_buf = WriteBuf::new(&mut buffer[..]);

    let sbe_header_offset = 0;
    let message_body_offset = sbe_header_offset + message_header_codec::ENCODED_LENGTH;

    let mut order_encoder = NewOrderSingleEncoder::default().wrap(write_buf, message_body_offset);

    let mut header_composite_encoder = order_encoder.header(sbe_header_offset);
    order_encoder = header_composite_encoder
        .parent()
        .expect("Failed to retrieve parent encoder after SBE header encoding");

    order_encoder.cl_ord_id(cl_ord_id);
    order_encoder.party_id(party_id);
    order_encoder.symbol(symbol);
    order_encoder.side(side);

    let mut transact_time_encoder = order_encoder.transact_time_encoder();
    transact_time_encoder.time(timestamp_ns);
    order_encoder = transact_time_encoder
        .parent()
        .expect("Failed to retrieve parent encoder after transact_time encoding");

    order_encoder.ord_type(ord_type);

    let mut order_qty_encoder = order_encoder.order_qty_encoder();
    order_qty_encoder.mantissa(qty_mantissa);
    order_encoder = order_qty_encoder
        .parent()
        .expect("Failed to retrieve parent encoder after order_qty encoding");

    let mut price_encoder_composite = order_encoder.price_encoder();
    price_encoder_composite.mantissa(price_mantissa);
    order_encoder = price_encoder_composite
        .parent()
        .expect("Failed to retrieve parent encoder after price encoding");

    buffer
}

fn main() {
    let settings = Settings::new();

    println!(
        "Publishing to channel {} on Stream ID {}",
        settings.channel, settings.stream_id
    );

    let mut context = Context::new();

    if !settings.dir_prefix.is_empty() {
        context.set_aeron_dir(settings.dir_prefix.clone());
    }

    println!("Using CnC file: {}", context.cnc_file_name());

    context.set_new_publication_handler(Box::new(on_new_publication_handler));
    context.set_error_handler(Box::new(error_handler));
    context.set_pre_touch_mapped_memory(true);

    let aeron = Aeron::new(context);

    if aeron.is_err() {
        println!("Error creating Aeron instance: {:?}", aeron.err());
        return;
    }

    let mut aeron = aeron.unwrap();

    // add the publication to start the process
    let publication_id = aeron
        .add_publication(str_to_c(&settings.channel), settings.stream_id)
        .expect("Error adding publication");

    let publication = loop {
        if let Ok(publication) = aeron.find_publication(publication_id) {
            break publication;
        }
        std::thread::yield_now();
    };

    let channel_status = publication.lock().unwrap().channel_status();

    println!(
        "Publication channel status {}: {} ",
        channel_status,
        channel_status_to_str(channel_status)
    );

    let mut rng = rand::rng();
    
    loop {
        let side = if rng.random_bool(0.5) {
            SideEnum::Buy
        } else {
            SideEnum::Sell
        };

        let price: i64 = rng.random_range(10_000..=60_000);
        let symbol: [u8; 6] = {
            let mut buf = [0u8; 6];
            let sym = b"AAPL";
            buf[..sym.len()].copy_from_slice(sym);
            buf
        };

        let order_bytes: [u8; 72] = create_order(
            Uuid::new_v4().as_bytes(),
            Uuid::new_v4().as_bytes(),
            &symbol,
            side,
            OrdTypeEnum::Limit,
            1_717_000_000_000_000,
            100_000,
            price,
        );

        let buffer = AlignedBuffer::with_capacity(72);
        let src_buffer = AtomicBuffer::from_aligned(&buffer);
        src_buffer.put_bytes(0, &order_bytes);

        let result = publication
            .lock()
            .unwrap()
            .offer_part(src_buffer, 0, 72 as i32);

        match result {
            Ok(code) => {
                if code < 0 {
                    eprintln!("Back pressure or admin action: {}", code);
                }
            },
            Err(err) => {
                eprintln!("Offer failed: {}", err);
            }
        }

    }

    // let order_bytes = create_order(
    //     b"ORDER1234567890A",
    //     b"TRADERXYZ1234567",
    //     b"BTCUSD",
    //     SideEnum::Buy,
    //     OrdTypeEnum::Limit,
    //     1_717_000_000_000_000,
    //     100_000,
    //     500_000,
    // );

    // let buffer = AlignedBuffer::with_capacity(76);
    // let src_buffer = AtomicBuffer::from_aligned(&buffer);
    // src_buffer.put_bytes(0, &order_bytes);

    // let result = publication
    //     .lock()
    //     .unwrap()
    //     .offer_part(src_buffer, 0, 76 as i32);
    // match result {
    //     Ok(code) => println!("Sent with code {}", code),
    //     Err(err) => println!("Offer with error: {}", err),
    // }

    // if !publication.lock().unwrap().is_connected() {
    //     println!("No active subscribers detected");
    // }

    println!("Done sending.");
}
