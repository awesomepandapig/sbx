use super::orderbook::Order;

use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use aeron_rs::concurrent::logbuffer::buffer_claim::BufferClaim;
use aeron_rs::concurrent::strategies::BusySpinIdleStrategy;
use aeron_rs::concurrent::strategies::Strategy;
use aeron_rs::exclusive_publication::ExclusivePublication;
use aeron_rs::utils::errors::AeronError;

use sbe::WriteBuf;
use sbe::exec_type_enum::ExecTypeEnum;
use sbe::execution_report_codec::{ExecutionReportEncoder, SBE_BLOCK_LENGTH};
use sbe::message_header_codec::ENCODED_LENGTH;
use sbe::ord_rej_reason_enum::OrdRejReasonEnum;
use sbe::ord_status_enum::OrdStatusEnum;
use sbe::ord_type_enum::OrdTypeEnum;

const MAX_MESSAGE_SIZE: usize = SBE_BLOCK_LENGTH as usize + ENCODED_LENGTH;

#[derive(Clone, Copy)]
pub struct Trade {
    pub match_id: u64,
    pub qty: i64,
    pub px: i64,
}

#[derive(Clone, Copy)]
pub struct Reject {
    pub reason: OrdRejReasonEnum,
}

#[derive(Clone, Copy)]
pub enum ExecutionReport {
    New,
    Trade(Trade),
    Cancel,
    Reject(Reject),
}

impl ExecutionReport {
    #[inline(always)]
    pub const fn exec_type(&self) -> ExecTypeEnum {
        match self {
            ExecutionReport::New => ExecTypeEnum::New,
            ExecutionReport::Trade(_) => ExecTypeEnum::Trade,
            ExecutionReport::Cancel => ExecTypeEnum::Canceled,
            ExecutionReport::Reject(_) => ExecTypeEnum::Rejected,
        }
    }

    #[inline(always)]
    pub fn ord_status(&self, order: &Order) -> OrdStatusEnum {
        match self {
            ExecutionReport::New => OrdStatusEnum::New,
            ExecutionReport::Trade(_) => {
                if order.is_fully_filled() {
                    OrdStatusEnum::Filled
                } else {
                    OrdStatusEnum::PartiallyFilled
                }
            }
            ExecutionReport::Cancel => OrdStatusEnum::Canceled,
            ExecutionReport::Reject(_) => OrdStatusEnum::Rejected,
        }
    }

    #[inline(always)]
    pub fn ord_rej_reason(&self) -> OrdRejReasonEnum {
        match self {
            ExecutionReport::Reject(r) => r.reason,
            _ => OrdRejReasonEnum::NullVal,
        }
    }

    #[inline(always)]
    fn set_optional_fields<'a>(
        self,
        mut encoder: ExecutionReportEncoder<'a>,
        order: &Order,
    ) -> ExecutionReportEncoder<'a> {
        match self {
            ExecutionReport::Trade(trade) => {
                encoder.trd_match_id(trade.match_id);
                let encoder = Self::set_last_px(encoder, trade.px);
                let encoder = Self::set_last_qty(encoder, trade.qty);
                Self::set_avg_px(encoder, order.avg_px())
            }
            _ => {
                encoder.trd_match_id(u64::MAX);
                let encoder = Self::set_last_px(encoder, i64::MIN);
                let encoder = Self::set_last_qty(encoder, i64::MIN);
                Self::set_avg_px(encoder, i64::MIN)
            }
        }
    }

    #[inline(always)]
    fn set_last_qty(encoder: ExecutionReportEncoder<'_>, qty: i64) -> ExecutionReportEncoder<'_> {
        let mut last_qty_encoder = encoder.last_qty_encoder();
        last_qty_encoder.mantissa(qty);
        last_qty_encoder
            .parent()
            .expect("Failed to get parent after last_qty")
    }

    #[inline(always)]
    fn set_last_px(encoder: ExecutionReportEncoder<'_>, px: i64) -> ExecutionReportEncoder<'_> {
        let mut last_px_encoder = encoder.last_px_encoder();
        last_px_encoder.mantissa(px);
        last_px_encoder
            .parent()
            .expect("Failed to get parent after last_px")
    }

    #[inline(always)]
    fn set_avg_px(encoder: ExecutionReportEncoder<'_>, avg_px: i64) -> ExecutionReportEncoder<'_> {
        let mut avg_px_encoder = encoder.avg_px_encoder();
        avg_px_encoder.mantissa(avg_px);
        avg_px_encoder
            .parent()
            .expect("Failed to get parent after avg_px")
    }
}

pub struct ExecutionReportPublisher {
    publication: Arc<Mutex<ExclusivePublication>>,
    buffer_claim: BufferClaim,
    offer_idle_strategy: BusySpinIdleStrategy,
}

impl ExecutionReportPublisher {
    pub fn new(publication: Arc<Mutex<ExclusivePublication>>) -> Self {
        Self {
            publication,
            buffer_claim: BufferClaim::default(),
            offer_idle_strategy: BusySpinIdleStrategy::default(),
        }
    }

    #[inline(always)]
    pub fn publish_new_order(&mut self, order: &Order, exec_id: u64) {
        self.publish_execution_report(&ExecutionReport::New, order, exec_id);
    }

    #[inline(always)]
    pub fn publish_trade(&mut self, order: &Order, exec_id: u64, match_id: u64, qty: i64, px: i64) {
        let trade_report = ExecutionReport::Trade(Trade { match_id, qty, px });
        self.publish_execution_report(&trade_report, order, exec_id);
    }

    #[inline(always)]
    pub fn publish_cancel(&mut self, order: &Order, exec_id: u64) {
        self.publish_execution_report(&ExecutionReport::Cancel, order, exec_id);
    }

    #[inline(always)]
    pub fn publish_reject(&mut self, order: &Order, exec_id: u64, reason: OrdRejReasonEnum) {
        let reject_report = ExecutionReport::Reject(Reject { reason });
        self.publish_execution_report(&reject_report, order, exec_id);
    }

    #[inline(always)]
    fn publish_execution_report(&mut self, report: &ExecutionReport, order: &Order, exec_id: u64) {
        self.offer_idle_strategy.reset();

        loop {
            let result = self
                .publication
                .lock()
                .unwrap()
                .try_claim(MAX_MESSAGE_SIZE as i32, &mut self.buffer_claim);
            
            match result {
                Ok(_) => break,
                Err(AeronError::BackPressured) => {
                    self.offer_idle_strategy.idle();
                }
                Err(err) => {
                    Self::handle_publication_error(err, exec_id);
                    return;
                }
            }
        }

        let offset = self.buffer_claim.offset() as usize;
        let mut buffer = self.buffer_claim.buffer();
        let claimed_slice = &mut buffer.as_mutable_slice()[offset..offset + MAX_MESSAGE_SIZE];

        let write_buf = WriteBuf::new(claimed_slice);
        let mut encoder = Self::begin_encoding(write_buf);

        Self::set_common_fields(&mut encoder, order, exec_id);
        let mut encoder = Self::set_composite_fields(encoder, order);
        encoder.exec_type(report.exec_type());
        encoder.ord_status(report.ord_status(order));
        encoder.ord_rej_reason(report.ord_rej_reason());
        report.set_optional_fields(encoder, order);

        self.buffer_claim.commit();
    }

    fn handle_publication_error(_err: AeronError, _exec_id: u64) {
        return;
        // TODO: Add error traces
        // match err {
        //     Err(AeronError::AdminAction) => {
        //         println!("TryClaim: AdminAction encountered, retrying...");
        //         self.offer_idle_strategy.idle();
        //         break;
        //     }
        //     Err(AeronError::NotConnected) => {
                
        //     }
        //     Err(AeronError::PublicationClosed) => {
        //         return;
        //     }
        //     Err(AeronError::MaxPositionExceeded) => {
                
        //     }
        //     Err(other_err) => {
                
        //     }
        // }
    }

    #[inline(always)]
    fn begin_encoding<'a>(write_buf: WriteBuf<'a>) -> ExecutionReportEncoder<'a> {
        let encoder = ExecutionReportEncoder::default().wrap(write_buf, ENCODED_LENGTH);
        encoder
            .header(0)
            .parent()
            .expect("Failed to create encoder header")
    }

    #[inline(always)]
    fn set_common_fields<'a>(
        encoder: &mut ExecutionReportEncoder<'a>,
        order: &Order,
        exec_id: u64,
    ) {
        encoder.cl_ord_id(&order.cl_ord_id);
        encoder.account(&order.account);
        encoder.order_id(order.seq_num);
        encoder.exec_id(exec_id);
        encoder.symbol(&order.symbol);
        encoder.side(order.side);
    }

    #[inline(always)]
    fn set_composite_fields<'a>(
        encoder: ExecutionReportEncoder<'a>,
        order: &Order,
    ) -> ExecutionReportEncoder<'a> {
        let encoder = Self::set_transact_time(encoder);
        let encoder = Self::set_leaves_qty(encoder, order.leaves_qty);
        let encoder = Self::set_cum_qty(encoder, order.cum_qty);

        // Only set price for limit orders
        if order.ord_type == OrdTypeEnum::Limit {
            Self::set_price(encoder, order.price)
        } else {
            Self::set_price(encoder, i64::MIN)
        }
    }

    #[inline(always)]
    fn set_transact_time(encoder: ExecutionReportEncoder<'_>) -> ExecutionReportEncoder<'_> {
        let transact_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System time is before Unix epoch")
            .as_nanos() as u64;

        let mut time_encoder = encoder.transact_time_encoder();
        time_encoder.time(transact_time);
        time_encoder
            .parent()
            .expect("Failed to get parent after transact_time")
    }

    #[inline(always)]
    fn set_leaves_qty(encoder: ExecutionReportEncoder<'_>, qty: i64) -> ExecutionReportEncoder<'_> {
        let mut qty_encoder = encoder.leaves_qty_encoder();
        qty_encoder.mantissa(qty);
        qty_encoder
            .parent()
            .expect("Failed to get parent after leaves_qty")
    }

    #[inline(always)]
    fn set_cum_qty(encoder: ExecutionReportEncoder<'_>, qty: i64) -> ExecutionReportEncoder<'_> {
        let mut qty_encoder = encoder.cum_qty_encoder();
        qty_encoder.mantissa(qty);
        qty_encoder
            .parent()
            .expect("Failed to get parent after cum_qty")
    }

    #[inline(always)]
    fn set_price(encoder: ExecutionReportEncoder<'_>, price: i64) -> ExecutionReportEncoder<'_> {
        let mut price_encoder = encoder.price_encoder();
        price_encoder.mantissa(price);
        price_encoder
            .parent()
            .expect("Failed to get parent after price")
    }
}
