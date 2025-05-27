use std::env;
use std::ffi::CString;
use std::slice;
use std::time::SystemTime;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::{AlignedBuffer, AtomicBuffer};
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;
use aeron_rs::context::Context;
use aeron_rs::example_config::{DEFAULT_CHANNEL, DEFAULT_STREAM_ID};
use aeron_rs::utils::errors::AeronError;
use aeron_rs::utils::types::Index;

use sbe::exec_type_enum::ExecTypeEnum;
use sbe::execution_report_codec::ExecutionReportDecoder;
use sbe::message_header_codec::MessageHeaderDecoder;
use sbe::new_order_single_codec::NewOrderSingleEncoder;
use sbe::ord_rej_reason_enum::OrdRejReasonEnum;
use sbe::ord_status_enum::OrdStatusEnum;
use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;
use sbe::*;

use serde::Serialize;

use rand::Rng;
use uuid::Uuid;

#[derive(Clone)]
struct Settings {
    dir_prefix: String,
    channel: String,
    stream_id: i32,
}

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
    let order_info = OrderPrintHelper {
        cl_ord_id: hex::encode(cl_ord_id),
        party_id: hex::encode(party_id),
        symbol: String::from_utf8_lossy(symbol).to_string(),
        side: format!("{:?}", side),
        ord_type: format!("{:?}", ord_type),
        timestamp_ns,
        qty_mantissa,
        price_mantissa,
    };
    // println!("{}", serde_json::to_string_pretty(&order_info).unwrap());

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

#[derive(Debug)]
struct DecodedMessageInfo {
    cl_ord_id: ClOrdIdType,
    original_send_timestamp_ns: u64,
    // other fields from ExecutionReport can be added if needed for more complex logic
}

fn read_message(buffer: &AtomicBuffer, offset: Index, length: Index, _header: &Header) -> Option<DecodedMessageInfo> {
    let slice_msg = unsafe {
        slice::from_raw_parts_mut(buffer.buffer().offset(offset as isize), length as usize)
    };
    let read_buf = ReadBuf::new(slice_msg);
    let header_decoder: MessageHeaderDecoder<ReadBuf<'_>> =
        MessageHeaderDecoder::default().wrap(read_buf, 0);

    match header_decoder.template_id() {
        3 => {
            let execution_msg = decode_execution_report(header_decoder);

            // execution_msg.print();
            
            println!(
    "{:<10} {:<36} | {:<10} {:<36} | {:<10} {:<10} | {:<5} {:<4} | {:<6} {:>10} | {:<6} {:>20}",
    "ClOrdID:", Uuid::from_bytes(execution_msg.cl_ord_id).to_string(),
    "PartyID:", Uuid::from_bytes(execution_msg.party_id).to_string(),
    "ExecType:", execution_msg.exec_type,
    "Side:", execution_msg.side,
    "Price:", execution_msg.price,
    "Time:", execution_msg.transact_time,
);

            // let execution_report_decoder: ExecutionReportDecoder<'_> =
            //     ExecutionReportDecoder::default().header(header_decoder, 0);

            // Some(DecodedMessageInfo {
            //      cl_ord_id: execution_report_decoder.cl_ord_id(), // Useful for correlation
            //      original_send_timestamp_ns: execution_report_decoder.transact_time_decoder().time(),
            // })
            None

            // println!("{:?}", execution_msg.transact_time);
            // match order.ord_type {
            //     OrdTypeEnum::Limit => self.process_limit_order(order),
            //     OrdTypeEnum::Market => {
            //         // TODO: Implement market order processing
            //     }
            //     _ => {
            //         self.publish_reject(&order, OrdRejReasonEnum::Other)
            //         // TODO: Log and continue
            //     }
            // }
        }
        4 => {
            // TODO:cancel order reject
            None
        }
        _ => {
            // panic!("Unknown templateId");
            println!("{:?}", header_decoder.template_id());
            None
        }
    }
}

pub type ClOrdIdType = [u8; 16];
pub type PartyIdType = [u8; 16];
pub type SymbolType = [u8; 6];

#[derive(Debug)]
struct ExecutionReportMessage {
    pub cl_ord_id: ClOrdIdType, // 16 bytes - Client Order ID
    pub party_id: PartyIdType,  // 16 bytes - Trading Party ID
    pub order_id: u64,
    pub exec_id: u64,
    transact_time: u64, // 8 bytes - Time of transaction from client
    pub price: i64,     // 8 bytes - Price for Limit orders
    pub last_qty: i64,
    pub last_px: i64,
    pub leaves_qty: i64, // 8 bytes - Remaining quantity to be filled
    pub cum_qty: i64,    // 8 bytes - Cumulative quantity filled
    pub avg_px: i64,
    pub symbol: SymbolType, // 6 bytes - Instrument symbol
    pub exec_type: ExecTypeEnum,
    pub ord_status: OrdStatusEnum,
    pub ord_rej_reason: OrdRejReasonEnum,
    pub side: SideEnum, // 4 bytes - Buy or Sell
}

impl ExecutionReportMessage {
    pub fn print(&self) {
        println!("{{");
        println!("  cl_ord_id: {:?}", Uuid::from_bytes(self.cl_ord_id));
        println!("  party_id: {:?}", Uuid::from_bytes(self.party_id));
        println!("  order_id: {}", self.order_id);
        println!("  exec_id: {}", self.exec_id);
        println!("  transact_time: {}", self.transact_time);
        println!("  price: {}", self.price);
        println!("  last_qty: {}", self.last_qty);
        println!("  last_px: {}", self.last_px);
        println!("  leaves_qty: {}", self.leaves_qty);
        println!("  cum_qty: {}", self.cum_qty);
        println!("  avg_px: {}", self.avg_px);
        println!("  symbol: {:?}", self.symbol);
        println!("  exec_type: {:?}", self.exec_type);
        println!("  ord_status: {:?}", self.ord_status);
        println!("  ord_rej_reason: {:?}", self.ord_rej_reason);
        println!("  side: {:?}", self.side);
        println!("}}");
    }
}

fn decode_execution_report(
    header_decoder: MessageHeaderDecoder<ReadBuf<'_>>,
) -> ExecutionReportMessage {
    let execution_report_decoder: ExecutionReportDecoder<'_> =
        ExecutionReportDecoder::default().header(header_decoder, 0);

    ExecutionReportMessage {
        cl_ord_id: execution_report_decoder.cl_ord_id(),
        party_id: execution_report_decoder.party_id(),
        order_id: execution_report_decoder.order_id(),
        exec_id: execution_report_decoder.exec_id(),
        transact_time: execution_report_decoder.transact_time_decoder().time(),
        price: execution_report_decoder.price_decoder().mantissa(),
        last_qty: execution_report_decoder.last_qty_decoder().mantissa(),
        last_px: execution_report_decoder.last_px_decoder().mantissa(),
        leaves_qty: execution_report_decoder.leaves_qty_decoder().mantissa(),
        cum_qty: execution_report_decoder.cum_qty_decoder().mantissa(),
        avg_px: execution_report_decoder.avg_px_decoder().mantissa(),
        symbol: execution_report_decoder.symbol(),
        exec_type: execution_report_decoder.exec_type(),
        ord_status: execution_report_decoder.ord_status(),
        ord_rej_reason: execution_report_decoder.ord_rej_reason(),
        side: execution_report_decoder.side(),
    }
}

#[derive(Serialize)]
struct OrderPrintHelper {
    cl_ord_id: String,
    party_id: String,
    symbol: String,
    side: String,
    ord_type: String,
    timestamp_ns: u64,
    qty_mantissa: i64,
    price_mantissa: i64,
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

    let aeron = Aeron::new(context);

    if aeron.is_err() {
        println!("Error creating Aeron instance: {:?}", aeron.err());
        return;
    }

    let mut aeron = aeron.unwrap();

    // ----- initialize publication --------------------------------------------
    // TODO:let channel = std::env::var("CHANNEL").expect("CHANNEL environment variable not set");
    let publication_channel: String = String::from("aeron:ipc?endpoint=localhost:40123");
    // TODO: let stream_id = std::env::var("STREAM_ID").expect("STREAM_ID environment variable not set");
    let publication_stream_id: i32 = "1001".parse().unwrap();

    let publication_id: i64 = aeron
        .add_publication(str_to_c(&publication_channel), publication_stream_id)
        .expect("Error adding publication");

    let publication = loop {
        if let Ok(publication) = aeron.find_publication(publication_id) {
            break publication;
        }
        std::thread::yield_now();
    };

    let channel_status = publication.lock().unwrap().channel_status();

    println!("Publication: {}", channel_status_to_str(channel_status));
    // -------------------------------------------------------------------------

    // ----- initialize subscription -------------------------------------------
    // TODO:let channel = std::env::var("CHANNEL").expect("CHANNEL environment variable not set");
    let subscription_channel: String = String::from("aeron:ipc?endpoint=localhost:40124");
    // TODO: let stream_id = std::env::var("STREAM_ID").expect("STREAM_ID environment variable not set");
    let subscription_stream_id: i32 = "1002".parse().unwrap();

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

    let mut rng = rand::rng();

    // println!("Starting RTT measurement loop...");

    // for i in 0..100 {
    //     let side = if rng.random_bool(0.5) {
    //         SideEnum::Buy
    //     } else {
    //         SideEnum::Sell
    //     };

    //     let price: i64 = rng.random_range(10_000..=60_000);
    //     let symbol: [u8; 6] = {
    //         let mut buf = [0u8; 6];
    //         let sym = b"AAPL";
    //         buf[..sym.len()].copy_from_slice(sym);
    //         buf
    //     };

    //     let start_time = SystemTime::now()
    //         .duration_since(SystemTime::UNIX_EPOCH)
    //         .expect("System time is before Unix epoch")
    //         .as_nanos() as u64;

    //     let order_bytes: [u8; 72] = create_order(
    //         Uuid::new_v4().as_bytes(),
    //         Uuid::new_v4().as_bytes(),
    //         &symbol,
    //         side,
    //         OrdTypeEnum::Limit,
    //         start_time,
    //         100_000,
    //         price,
    //     );

    //     let buffer = AlignedBuffer::with_capacity(72);
    //     let src_buffer = AtomicBuffer::from_aligned(&buffer);
    //     src_buffer.put_bytes(0, &order_bytes);

    //     let result = publication
    //         .lock()
    //         .unwrap()
    //         .offer_part(src_buffer, 0, 72 as i32);

    //     match result {
    //         Ok(code) => {
    //             if code < 0 {
    //                 eprintln!("Back pressure or admin action: {}", code);
    //             }
    //         },
    //         Err(err) => {
    //             eprintln!("Offer failed: {}", err);
    //         }
    //     }

    //     let mut message_processed = false;
    //     for _ in 0..10 { // Try polling a few times
    //         let fragments_read = subscription.lock().unwrap().poll(
    //             &mut |buffer, offset, length, header| {
    //                 let arrival_time_ns = SystemTime::now()
    //                     .duration_since(SystemTime::UNIX_EPOCH)
    //                     .expect("SystemTime before UNIX_EPOCH!")
    //                     .as_nanos() as u64;

    //                 if let Some(decoded_info) = read_message(buffer, offset, length, header) {
    //                     // If you were using the sent_timestamps map:
    //                     // if let Some(original_send_time) = sent_timestamps.lock().unwrap().remove(&decoded_info.cl_ord_id) {
    //                     //    let rtt = arrival_time_ns - original_send_time;
    //                     //    println!("RTT for cl_ord_id {:x?}: {} ns", decoded_info.cl_ord_id[0], rtt);
    //                     // } else {
    //                     //    println!("Received message for unknown cl_ord_id: {:x?}", decoded_info.cl_ord_id[0]);
    //                     // }

    //                     // Direct RTT calculation using the timestamp from the message itself:
    //                     let rtt = arrival_time_ns - decoded_info.original_send_timestamp_ns;
    //                     // println!(
    //                     //     "Loop {}, ClOrdId {:x?}...: RTT = {} ns (Sent: {}, Received: {})",
    //                     //     i, decoded_info.cl_ord_id[0], rtt, decoded_info.original_send_timestamp_ns, arrival_time_ns
    //                     // );
    //                     message_processed = true;
    //                 }
    //             },
    //             1, // Process up to 1 fragment, can be increased
    //         );


    //         if fragments_read > 0 && message_processed {
    //             break;
    //         }
    //         std::thread::sleep(std::time::Duration::from_micros(10)); // Small delay before retrying poll
    //     }
    //     if !message_processed {
    //         // eprintln!("Warning: No response received for message with cl_ord_id prefix {:x?} after polling.", cl_ord_id[0]);
    //     }
    //      // Optional: Add a small delay between sends if desired for testing

    // }

    let order_bytes = create_order(
        b"ORDERAAAAAAAAAAA",
        b"TRADERXYZ1234567",
        b"BTCUSD",
        SideEnum::Buy,
        OrdTypeEnum::Limit,
        1_717_000_000_000_000,
        1,
        500_000,
    );

    let buffer = AlignedBuffer::with_capacity(76);
    let src_buffer = AtomicBuffer::from_aligned(&buffer);
    src_buffer.put_bytes(0, &order_bytes);

    let result = publication
        .lock()
        .unwrap()
        .offer_part(src_buffer, 0, 76 as i32);
    match result {
        Ok(code) => println!("Sent with code {}", code),
        Err(err) => println!("Offer with error: {}", err),
    }

    let order_bytes = create_order(
        b"ORDERBBBBBBBBBBB",
        b"TRADERXYZ1234567",
        b"BTCUSD",
        SideEnum::Buy,
        OrdTypeEnum::Limit,
        1_717_000_000_000_000,
        2,
        500_000,
    );

    let buffer = AlignedBuffer::with_capacity(76);
    let src_buffer = AtomicBuffer::from_aligned(&buffer);
    src_buffer.put_bytes(0, &order_bytes);

    let result = publication
        .lock()
        .unwrap()
        .offer_part(src_buffer, 0, 76 as i32);
    match result {
        Ok(code) => println!("Sent with code {}", code),
        Err(err) => println!("Offer with error: {}", err),
    }

    let order_bytes = create_order(
        b"ORDERCCCCCCCCCCC",
        b"TRADERXYZ1234567",
        b"BTCUSD",
        SideEnum::Buy,
        OrdTypeEnum::Limit,
        1_717_000_000_000_000,
        3,
        500_000,
    );

    let buffer = AlignedBuffer::with_capacity(76);
    let src_buffer = AtomicBuffer::from_aligned(&buffer);
    src_buffer.put_bytes(0, &order_bytes);

    let result = publication
        .lock()
        .unwrap()
        .offer_part(src_buffer, 0, 76 as i32);
    match result {
        Ok(code) => println!("Sent with code {}", code),
        Err(err) => println!("Offer with error: {}", err),
    }

    subscription.lock().unwrap().poll(
        &mut |buffer, offset, length, header| {
            read_message(buffer, offset, length, header);
        },
        10,
    );

    let order_bytes = create_order(
        b"ORDERDDDDDDDDDDD",
        b"TRADERXYZ1234567",
        b"BTCUSD",
        SideEnum::Sell,
        OrdTypeEnum::Limit,
        1_717_000_000_000_000,
        6,
        500_000,
    );

    let buffer = AlignedBuffer::with_capacity(76);
    let src_buffer = AtomicBuffer::from_aligned(&buffer);
    src_buffer.put_bytes(0, &order_bytes);

    let result = publication
        .lock()
        .unwrap()
        .offer_part(src_buffer, 0, 76 as i32);
    match result {
        Ok(code) => println!("Sent with code {}", code),
        Err(err) => println!("Offer with error: {}", err),
    }

    if !publication.lock().unwrap().is_connected() {
        println!("No active subscribers detected");
    }

    subscription.lock().unwrap().poll(
                &mut |buffer, offset, length, header| {

                    if let Some(decoded_info) = read_message(buffer, offset, length, header) {

                    }
                },
                4, // Process up to 1 fragment, can be increased
            );

    // println!("Done sending.");
}
