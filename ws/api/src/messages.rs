use std::slice;

use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::utils::types::Index;

use sbe::ReadBuf;
use sbe::execution_report_codec::ExecutionReportDecoder;
use sbe::message_header_codec::MessageHeaderDecoder;
use sbe::{
    exec_type_enum::ExecTypeEnum, ord_rej_reason_enum::OrdRejReasonEnum,
    ord_status_enum::OrdStatusEnum, side_enum::SideEnum,
};

use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub type UuidType = [u8; 16];
pub type SymbolType = [u8; 6];

#[derive(Serialize)]
#[serde(remote = "ExecTypeEnum")]
pub enum ExecTypeEnumDef {
    New,
    Canceled,
    Rejected,
    Trade,
    NullVal,
}

#[derive(Serialize)]
#[serde(remote = "OrdStatusEnum")]
pub enum OrdStatusEnumDef {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    NullVal,
}

#[derive(Serialize)]
#[serde(remote = "OrdRejReasonEnum")]
pub enum OrdRejReasonEnumDef {
    UnknownOrder,
    DuplicateOrder,
    StaleOrder,
    Other,
    NullVal,
}

#[derive(Serialize)]
#[serde(remote = "SideEnum")]
pub enum SideEnumDef {
    Buy,
    Sell,
    NullVal,
}

#[derive(Debug)]
struct ExecutionReportMessage {
    pub account: UuidType,         // 16 bytes
    pub cl_ord_id: UuidType,       // 16 bytes - Client Order ID
    pub trd_match_id: Option<u64>, // 8 bytes
    pub order_id: u64,
    pub exec_id: u64,
    pub transact_time: u64, // 8 bytes - Time of transaction from client
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
    pub side: SideEnum,
}

fn is_null_rej_reason(reason: &OrdRejReasonEnum) -> bool {
    *reason == OrdRejReasonEnum::NullVal
}

fn is_null_exec_type(exec_type: &ExecTypeEnum) -> bool {
    *exec_type == ExecTypeEnum::NullVal
}

#[derive(Serialize)]
struct ExecutionReportMessageLog {
    account: String,
    cl_ord_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    trd_match_id: Option<u64>,

    order_id: u64,
    exec_id: u64,
    transact_time: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_qty: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_px: Option<String>,

    leaves_qty: String,
    cum_qty: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_px: Option<String>,
    
    symbol: String,

    #[serde(skip_serializing_if = "is_null_exec_type")]
    #[serde(with = "ExecTypeEnumDef")]
    exec_type: ExecTypeEnum,
    
    #[serde(with = "OrdStatusEnumDef")]
    ord_status: OrdStatusEnum,

    #[serde(skip_serializing_if = "is_null_rej_reason")]
    #[serde(with = "OrdRejReasonEnumDef")]
    ord_rej_reason: OrdRejReasonEnum,

    #[serde(with = "SideEnumDef")]
    side: SideEnum,
}

fn format_decimal_with_exponent_neg8(mantissa: i64) -> String {
    const SCALE: i64 = 100_000_000;

    let integral = mantissa / SCALE;
    let fractional_part = (mantissa % SCALE).abs();

    if fractional_part == 0 {
        return integral.to_string();
    }

    
    let mut fractional_str = format!("{:08}", fractional_part);
    while fractional_str.ends_with('0') {
        fractional_str.pop();
    }

    if fractional_str.is_empty() {
        integral.to_string()
    } else {
        format!("{}.{}", integral, fractional_str)
    }
}

fn nanos_to_iso8601(nanos_since_epoch: i64) -> String {
    DateTime::<Utc>::from_timestamp_nanos(nanos_since_epoch)
        .format("%Y-%m-%dT%H:%M:%S.%9fZ")
        .to_string()
}

impl ExecutionReportMessageLog {
    fn from(msg: ExecutionReportMessage) -> Self {
        const DECIMAL_NULL_VAL: i64 = i64::MIN; 

        let price = if msg.price != DECIMAL_NULL_VAL {
            Some(format_decimal_with_exponent_neg8(msg.price))
        } else {
            None
        };

        let last_qty = if msg.last_qty != DECIMAL_NULL_VAL {
            Some(format_decimal_with_exponent_neg8(msg.last_qty))
        } else {
            None
        };

        let last_px = if msg.last_px != DECIMAL_NULL_VAL {
            Some(format_decimal_with_exponent_neg8(msg.last_px))
        } else {
            None
        };

        let avg_px = if msg.avg_px != DECIMAL_NULL_VAL {
            Some(format_decimal_with_exponent_neg8(msg.avg_px))
        } else {
            None
        };

        Self {
            cl_ord_id: Uuid::from_bytes(msg.cl_ord_id).to_string(),
            account: Uuid::from_bytes(msg.account).to_string(),
            trd_match_id: msg.trd_match_id,
            order_id: msg.order_id,
            exec_id: msg.exec_id,
            transact_time: nanos_to_iso8601(msg.transact_time as i64),
            price,
            last_qty,
            last_px,
            leaves_qty: format_decimal_with_exponent_neg8(msg.leaves_qty),
            cum_qty: format_decimal_with_exponent_neg8(msg.cum_qty),
            avg_px,
            symbol: String::from_utf8_lossy(&msg.symbol).trim_end_matches('\0').to_string(),
            exec_type: msg.exec_type, 
            ord_status: msg.ord_status,
            ord_rej_reason: msg.ord_rej_reason,
            side: msg.side,
        }
    }
}

pub fn read_message(buffer: &AtomicBuffer, offset: Index, length: Index, _header: &Header) {
    let slice_msg = unsafe {
        slice::from_raw_parts_mut(buffer.buffer().offset(offset as isize), length as usize)
    };
    let read_buf = ReadBuf::new(slice_msg);
    let header_decoder: MessageHeaderDecoder<ReadBuf<'_>> =
        MessageHeaderDecoder::default().wrap(read_buf, 0);

    match header_decoder.template_id() {
        3 => {
            let execution_msg = decode_execution_report(header_decoder);

            let log_view = ExecutionReportMessageLog::from(execution_msg);
            match serde_json::to_string(&log_view) {
                Ok(json) => println!("{}", json),
                Err(e) => eprintln!("Failed to serialize ExecutionReportMessage: {}", e),
            }

        }
        4 => {}
        _ => {}
    }
}

fn decode_execution_report(
    header_decoder: MessageHeaderDecoder<ReadBuf<'_>>,
) -> ExecutionReportMessage {
    let execution_report_decoder: ExecutionReportDecoder<'_> =
        ExecutionReportDecoder::default().header(header_decoder, 0);

    ExecutionReportMessage {
        account: execution_report_decoder.account(),
        cl_ord_id: execution_report_decoder.cl_ord_id(),
        trd_match_id: execution_report_decoder.trd_match_id(),
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
        side: execution_report_decoder.side()
    }
}
