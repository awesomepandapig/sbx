mod aeron_handler;
use aeron_handler::{build_context, create_subscription, get_aeron_dir};
use sbe::ord_status_enum::OrdStatusEnum;

use std::time::Instant;

use aeron_rs::aeron::Aeron;
use aeron_rs::concurrent::atomic_buffer::{AtomicBuffer};
use aeron_rs::concurrent::status::status_indicator_reader::channel_status_to_str;
use sbe::exec_type_enum::ExecTypeEnum;
use sbe::side_enum::SideEnum;

use log::{error, info};

use std::slice;

use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::utils::types::Index;

use sbe::ReadBuf;
use sbe::execution_report_codec::ExecutionReportDecoder;
use sbe::message_header_codec::MessageHeaderDecoder;

const DECIMAL_NULL_VAL: i64 = i64::MIN;

use uuid::Uuid;

use questdb::{
    ingress::{Buffer, Sender, TimestampNanos},
};

pub fn read_message(
    buffer: &AtomicBuffer,
    offset: Index,
    length: Index,
    _header: &Header,
    qdb_buffer: &mut Buffer,
    qdb_sender: &mut Sender,
) {
    let slice_msg = unsafe {
        slice::from_raw_parts_mut(buffer.buffer().offset(offset as isize), length as usize)
    };
    let read_buf = ReadBuf::new(slice_msg);
    let header_decoder: MessageHeaderDecoder<ReadBuf<'_>> =
        MessageHeaderDecoder::default().wrap(read_buf, 0);

    match header_decoder.template_id() {
        3 => {
            let _result = write_to_db(header_decoder, qdb_buffer, qdb_sender);
            // println!("RESULT: {:?}", result);
        }
        4 => {}
        _ => {}
    }
}

fn write_to_db(
    header_decoder: MessageHeaderDecoder<ReadBuf<'_>>,
    qdb_buffer: &mut Buffer,
    qdb_sender: &mut Sender,
) -> questdb::Result<()> {
    let report: ExecutionReportDecoder<'_> =
        ExecutionReportDecoder::default().header(header_decoder, 0);

    let exec_type = report.exec_type();
    if !(exec_type == ExecTypeEnum::New || exec_type == ExecTypeEnum::Trade) {
        return Ok(());
    }

    let account = &report.account()[..];
    let account = std::str::from_utf8(account).unwrap_or("");

    let cl_ord_id = &report.account()[..];
    let cl_ord_id = std::str::from_utf8(cl_ord_id).unwrap_or("");

    // let account_bytes = report.account();
    // let account = Uuid::from_bytes(account_bytes).to_string();
    
    // let cl_ord_id_bytes = report.cl_ord_id();
    // let cl_ord_id = Uuid::from_bytes(cl_ord_id_bytes).to_string();

    let price_m = report.price_decoder().mantissa();
    let avg_px_m = report.avg_px_decoder().mantissa();

    let price = if price_m != DECIMAL_NULL_VAL {
        Some(price_m as f64 / 1e8)
    } else {
        None
    };

    let avg_px = if avg_px_m != DECIMAL_NULL_VAL {
        Some(avg_px_m as f64 / 1e8)
    } else {
        None
    };

    let leaves_qty = report.leaves_qty_decoder().mantissa() as f64 / 1e8;
    let cum_qty = report.cum_qty_decoder().mantissa() as f64 / 1e8;

    let ord_status = match report.ord_status() {
        OrdStatusEnum::New => "new",
        OrdStatusEnum::PartiallyFilled => "partially_filled",
        OrdStatusEnum::Canceled => "canceled",
        _ => return Ok(()),
    };
    
    let side_bool = match report.side() {
        SideEnum::Buy => true,
        SideEnum::Sell => false,
        _ => return Ok(()),
    };

    let symbol_raw = report.symbol();
    let symbol = match str::from_utf8(&symbol_raw) {
        Ok(s) => s.trim_end_matches('\0'), // strip padding nulls
        Err(_) => return Ok(()),           // ignore invalid UTF-8
    };

    let timestamp = TimestampNanos::new(report.transact_time_decoder().time() as i64);

    let mut builder = qdb_buffer
        .table("orders")?
        .symbol("symbol", symbol)?
        .symbol("ord_status", ord_status)?
        .column_str("account", account)?
        .column_str("cl_ord_id", cl_ord_id)?
        .column_f64("leaves_qty", leaves_qty)?
        .column_f64("cum_qty", cum_qty)?
        .column_bool("side", side_bool)?;

    if let Some(p) = price {
        builder = builder.column_f64("price", p)?;
    }
    if let Some(a) = avg_px {
        builder = builder.column_f64("avg_px", a)?;
    }

    builder.at(timestamp)?;

    qdb_sender.flush( qdb_buffer)?;
    Ok(())
}

fn main() -> questdb::Result<()> {
    env_logger::init();

    let start_time = Instant::now();

    let aeron_dir = get_aeron_dir();
    info!("Aeron: Using directory: {:?}", aeron_dir);

    let context = build_context(&aeron_dir);
    let mut aeron = match Aeron::new(context) {
        Ok(a) => {
            info!("Aeron: Instance created");
            a
        }
        Err(e) => {
            error!("Aeron: Failed to create instance: {:?}", e);
            return Ok(());
        }
    };

    let subscription = create_subscription(&mut aeron, "aeron:udp?endpoint=224.1.1.1:40456|interface=localhost", 1002);
    let sub_status = subscription.lock().unwrap().channel_status();
    info!("Aeron: Subscription {}", channel_status_to_str(sub_status));

    let startup_duration = start_time.elapsed();
    info!(
        "Startup complete in {:.2?} — ready to accept requests",
        startup_duration
    );

    let mut qdb_sender = Sender::from_conf("http::addr=localhost:9000;")?;
    let mut qdb_buffer = Buffer::new();

    let mut subscription = subscription.lock().unwrap();

    let mut msg_count = 0;
    let flush_every = 1000;

    loop {
        subscription.poll(
            &mut |buffer, offset, length, header| {
                read_message(
                    buffer,
                    offset,
                    length,
                    header,
                    &mut qdb_buffer,
                    &mut qdb_sender,
                );
                msg_count += 1;
                if msg_count >= flush_every {
                    let _ = qdb_sender.flush(&mut qdb_buffer);
                    msg_count = 0;
                }
            },
            1024,
        );
        
    }
}
