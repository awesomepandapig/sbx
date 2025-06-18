use sbe::{
    ReadBuf, exec_type_enum::ExecTypeEnum, execution_report_codec::ExecutionReportDecoder,
    message_header_codec::MessageHeaderDecoder, ord_rej_reason_enum::OrdRejReasonEnum,
    ord_status_enum::OrdStatusEnum, side_enum::SideEnum,
};

pub type UuidType = [u64; 2];
pub type SymbolType = [u8; 6];

#[derive(Debug, Clone)]
pub struct ExecutionReportMessage {
    pub account: UuidType,
    pub cl_ord_id: UuidType,
    pub trd_match_id: Option<u64>,
    pub order_id: u64,
    pub exec_id: u64,
    pub transact_time: u64,
    pub price: i64,
    pub ord_qty: i64,
    pub last_qty: i64,
    pub last_px: i64,
    pub leaves_qty: i64,
    pub cum_qty: i64,
    pub avg_px: i64,
    pub symbol: SymbolType,
    pub exec_type: ExecTypeEnum,
    pub ord_status: OrdStatusEnum,
    pub ord_rej_reason: OrdRejReasonEnum,
    pub side: SideEnum,
}

pub fn decode_execution_report(
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
        ord_qty: execution_report_decoder.order_qty_decoder().mantissa(),
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
