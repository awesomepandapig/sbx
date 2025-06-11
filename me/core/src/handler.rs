use crate::config::order_count_max;
use crate::orderbook::OrderBook;
use crate::publisher::Publisher;
use crate::side::{Buy, Sell, SideSpecificContext};
use crate::types::{CancelRequest, Order};

use std::cmp::min;

use sbe::ReadBuf;
use sbe::cxl_rej_reason_enum::CxlRejReasonEnum;
use sbe::cxl_rej_response_to_enum::CxlRejResponseToEnum;
use sbe::message_header_codec::MessageHeaderDecoder;
use sbe::new_order_single_codec::NewOrderSingleDecoder;
use sbe::ord_rej_reason_enum::OrdRejReasonEnum;
use sbe::ord_type_enum::OrdTypeEnum;
use sbe::order_cancel_request_codec::OrderCancelRequestDecoder;
use sbe::side_enum::SideEnum;

use tracing::error;

macro_rules! execute_trade {
    ($self:expr, $aggressor_order:expr, $resting_order:expr) => {{
        let trade_quantity = min(
            $aggressor_order.leaves_quantity,
            $resting_order.leaves_quantity,
        );
        let trade_px = $resting_order.price;

        $aggressor_order.fill(trade_quantity, trade_px);
        $resting_order.fill(trade_quantity, trade_px);

        $self.counter_match_id += 1;
        $self.counter_exec_id += 1;
        $self.publisher.publish_trade(
            $aggressor_order,
            $self.counter_exec_id,
            $self.counter_match_id,
            trade_quantity,
            trade_px,
        );

        $self.counter_exec_id += 1;
        $self.publisher.publish_trade(
            $resting_order,
            $self.counter_exec_id,
            $self.counter_match_id,
            trade_quantity,
            trade_px,
        );
    }};
}

pub struct Handler {
    pub book: OrderBook,
    pub counter_order_id: u64,
    pub counter_exec_id: u64,
    pub counter_match_id: u64,
    pub publisher: Publisher,
}

impl Handler {
    pub fn new(publisher: Publisher) -> Self {
        let order_count_max = match usize::try_from(order_count_max()) {
            Ok(usize) => usize,
            Err(error) => panic!("Problem opening the file: {error:?}"), // TODO: NO PANIC
        };

        Self {
            book: OrderBook::new(order_count_max),
            counter_order_id: 0,
            counter_exec_id: 0,
            counter_match_id: 0,
            publisher,
        }
    }

    #[inline(always)]
    pub fn process_new_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) {
        let mut order = self.process_new_order_decode(header_decoder);

        if self.book.is_full() {
            self.publish_reject(&order, OrdRejReasonEnum::Other);
            error!(
                target: "matching_engine_capacity",
                reason = "Book capacity limit reached",
                "OPERATIONAL WARNING: Order book capacity limit reached. New orders are being rejected. Consider investigating load or if orders_count_max needs adjustment.",
            ); // TODO: ERROR HANDLER
            return;
        }

        if self
            .book
            .order_key_map
            .contains_key(&(order.account, order.client_order_id))
        {
            self.publish_reject(&order, OrdRejReasonEnum::DuplicateOrder);
            return;
        }

        self.publish_new_order(&order);
        self.route_by_type(&mut order);
    }

    #[inline(always)]
    pub fn process_cancel_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) {
        let request = self.process_cancel_order_decode(header_decoder);
        let order_key = (request.account, request.client_order_id);

        if !self.book.order_key_map.contains_key(&order_key) {
            self.publish_cancel_reject(
                &request,
                CxlRejReasonEnum::UnknownOrder,
                CxlRejResponseToEnum::OrderCancelRequest,
            );
            return;
        }

        let order = self.book.remove(order_key);
        self.publish_cancel(&order);
    }

    #[inline(always)]
    fn process_new_order_decode(
        &mut self,
        header_decoder: MessageHeaderDecoder<ReadBuf<'_>>,
    ) -> Order {
        let decoder: NewOrderSingleDecoder<'_> =
            NewOrderSingleDecoder::default().header(header_decoder, 0);

        self.counter_order_id += 1;
        let quantity = decoder.order_qty_decoder().mantissa();

        Order {
            prev_order_idx: None,
            next_order_idx: None,

            client_order_id: {
                let id = decoder.cl_ord_id();
                (u128::from(id[0]) << 64) | u128::from(id[1])
            },
            account: {
                let id = decoder.account();
                (u128::from(id[0]) << 64) | u128::from(id[1])
            },

            symbol: decoder.symbol(),
            side: decoder.side(),
            // transact_time: decoder.transact_time_decoder().time(),
            quantity,
            r#type: decoder.ord_type(),
            price: decoder.price_decoder().mantissa(),
            sequence_number: self.counter_order_id,
            leaves_quantity: quantity,
            cumulative_quantity: 0,
            total_notional: 0,
        }
    }

    #[inline(always)]
    fn process_cancel_order_decode(
        &mut self,
        header_decoder: MessageHeaderDecoder<ReadBuf<'_>>,
    ) -> CancelRequest {
        let decoder: OrderCancelRequestDecoder<'_> =
            OrderCancelRequestDecoder::default().header(header_decoder, 0);

        self.counter_order_id += 1;

        CancelRequest {
            original_client_order_id: {
                let id = decoder.orig_cl_ord_id();
                (u128::from(id[0]) << 64) | u128::from(id[1])
            },
            client_order_id: {
                let id = decoder.cl_ord_id();
                (u128::from(id[0]) << 64) | u128::from(id[1])
            },
            account: {
                let id = decoder.account();
                (u128::from(id[0]) << 64) | u128::from(id[1])
            },
            // transact_time: decoder.transact_time_decoder().time(),
            // symbol: decoder.symbol(),
            // side: decoder.side(),
        }
    }

    #[inline(always)]
    fn route_by_type(&mut self, order: &mut Order) {
        match (order.r#type, order.side) {
            (OrdTypeEnum::Limit, SideEnum::Buy) => self.handle_limit_order::<Buy>(order),
            (OrdTypeEnum::Limit, SideEnum::Sell) => self.handle_limit_order::<Sell>(order),
            (OrdTypeEnum::Market, SideEnum::Buy) => self.handle_market_order::<Buy>(order),
            (OrdTypeEnum::Market, SideEnum::Sell) => self.handle_market_order::<Sell>(order),

            (OrdTypeEnum::Limit | OrdTypeEnum::Market, SideEnum::NullVal) => {
                self.reject_invalid_field_order(order, "side");
            }

            (OrdTypeEnum::NullVal, _) => {
                self.reject_invalid_field_order(order, "ord_type");
            }
        }
    }

    #[inline(always)]
    fn reject_invalid_field_order(&mut self, order: &Order, field: &'static str) {
        self.publish_reject(order, OrdRejReasonEnum::Other);
        // TODO: ERROR HANDLER
        error!(
            target: "matching_engine_critical",
            order_id = ?order.client_order_id,
            order_details = ?order,
            "CRITICAL ERROR: Order received with NullVal for {field}. Order rejected. This may indicate message corruption, a gateway bug, or SBE schema mismatch.",
        );
    }

    #[inline(always)]
    fn handle_limit_order<S: SideSpecificContext>(&mut self, aggressor_order: &mut Order) {
        while aggressor_order.leaves_quantity > 0 {
            // Get the best price level
            let Some(mut resting_order) = S::get_best_opposite(&mut self.book) else {
                break; // No orders on opposite side
            };

            // Check if prices can cross at this level
            if !S::can_cross(aggressor_order.price, resting_order.price) {
                break;
            }

            // Process all orders at this price level
            loop {
                // Check for self-trading
                if aggressor_order.account == resting_order.account {
                    self.publish_cancel(aggressor_order); // TODO: publish cancel with STP as reason
                    return;
                }

                execute_trade!(self, aggressor_order, resting_order);

                // Check if aggressor is fully filled
                if aggressor_order.leaves_quantity == 0 {
                    // If resting order is also fully filled, remove it
                    if resting_order.leaves_quantity == 0 {
                        let resting_key = (resting_order.account, resting_order.client_order_id);
                        self.book.remove(resting_key);
                    }
                    return; // Aggressor is done
                }

                if resting_order.leaves_quantity == 0 {
                    let resting_key = (resting_order.account, resting_order.client_order_id);
                    let next_order_idx = resting_order.next_order_idx;
                    self.book.remove(resting_key);

                    if let Some(next_idx) = next_order_idx {
                        resting_order = self.book.pool.get_mut(next_idx).expect(
                            "Data consistency error: next_order_idx points to invalid order",
                        ); // TODO: Handle error
                    } else {
                        // No more orders at this price level, break to get next price level
                        break;
                    }
                }
            }
        }

        // Any remaining portion is added to the book
        if aggressor_order.leaves_quantity > 0 {
            S::add_to_book(&mut self.book, *aggressor_order);
        }
    }

    #[inline(always)]
    fn handle_market_order<S: SideSpecificContext>(&mut self, aggressor_order: &mut Order) {
        while aggressor_order.leaves_quantity > 0 {
            // Get the best price level
            let Some(mut resting_order) = S::get_best_opposite(&mut self.book) else {
                // No orders on opposite side
                self.publish_cancel(aggressor_order);
                return;
            };

            // Process all orders at this price level
            loop {
                // Check for self-trading
                if aggressor_order.account == resting_order.account {
                    self.publish_cancel(aggressor_order); // TODO: publish cancel with STP as reason
                    return;
                }

                execute_trade!(self, aggressor_order, resting_order);

                // Check if aggressor is fully filled
                if aggressor_order.leaves_quantity == 0 {
                    // If resting order is also fully filled, remove it
                    if resting_order.leaves_quantity == 0 {
                        let resting_key = (resting_order.account, resting_order.client_order_id);
                        self.book.remove(resting_key);
                    }
                    return; // Aggressor is done
                }

                if resting_order.leaves_quantity == 0 {
                    let resting_key = (resting_order.account, resting_order.client_order_id);
                    let next_order_idx = resting_order.next_order_idx;
                    self.book.remove(resting_key);

                    if let Some(next_idx) = next_order_idx {
                        resting_order = self.book.pool.get_mut(next_idx).expect(
                            "Data consistency error: next_order_idx points to invalid order",
                        ); // TODO: Handle error
                    } else {
                        // No more orders at this price level, break to get next price level
                        break;
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn publish_new_order(&mut self, order: &Order) {
        self.counter_exec_id += 1;
        self.publisher
            .publish_new_order(order, self.counter_exec_id);
    }

    #[inline(always)]
    fn publish_cancel(&mut self, order: &Order) {
        self.counter_exec_id += 1;
        self.publisher.publish_cancel(order, self.counter_exec_id);
    }

    #[inline(always)]
    fn publish_cancel_reject(
        &mut self,
        req: &CancelRequest,
        reason: CxlRejReasonEnum,
        response_to: CxlRejResponseToEnum,
    ) {
        self.counter_exec_id += 1;
        self.publisher
            .publish_cancel_reject(req, self.counter_exec_id, reason, response_to);
    }

    #[inline(always)]
    fn publish_reject(&mut self, order: &Order, reason: OrdRejReasonEnum) {
        self.counter_exec_id += 1;
        self.publisher
            .publish_reject(order, self.counter_exec_id, reason);
    }
}
