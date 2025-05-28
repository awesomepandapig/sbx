use std::slice;

use aeron_rs::concurrent::atomic_buffer::AtomicBuffer;
use aeron_rs::concurrent::logbuffer::header::Header;
use aeron_rs::utils::types::Index;

use sbe::ReadBuf;
use sbe::exec_type_enum::ExecTypeEnum;
use sbe::execution_report_codec::ExecutionReportDecoder;
use sbe::message_header_codec::MessageHeaderDecoder;
use sbe::ord_rej_reason_enum::OrdRejReasonEnum;
use sbe::ord_status_enum::OrdStatusEnum;

use sbe::side_enum::SideEnum;

use uuid::Uuid;

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

            println!(
                "{:<10} {:<36} | {:<10} {:<36} | {:<10} {:<10} | {:<5} {:<4} | {:<6} {:>10} | {:<6} {:>20}",
                "ClOrdID:",
                Uuid::from_bytes(execution_msg.cl_ord_id).to_string(),
                "PartyID:",
                Uuid::from_bytes(execution_msg.party_id).to_string(),
                "ExecType:",
                execution_msg.exec_type,
                "Side:",
                execution_msg.side,
                "Price:",
                execution_msg.last_px,
                "Time:",
                execution_msg.transact_time,
            );

            // execution_msg.print();
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
