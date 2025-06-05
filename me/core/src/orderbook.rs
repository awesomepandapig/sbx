use crate::config::orders_count_max;
use crate::publisher::Publisher;
use crate::side::{Buy, Sell, SideSpecificContext};
use crate::types::{
    AskQueue, BidQueue, CancelRequest, CapacityExceededError, Order, OrderBookKey, OrderMap,
    OrderPool,
};

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

use tracing::{error, warn};

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

pub struct OrderBook {
    pub orders_count_max: u64,
    pub queue_bid: BidQueue,
    pub queue_ask: AskQueue,
    pub order_pool: OrderPool,
    pub order_map: OrderMap,
    pub counter_order_id: u64,
    pub counter_exec_id: u64,
    pub counter_match_id: u64,
    pub publisher: Publisher,
}

impl OrderBook {
    pub fn new(publisher: Publisher) -> Self {
        let orders_count_max = orders_count_max();

        Self {
            orders_count_max,
            queue_bid: BidQueue::with_capacity(orders_count_max / 2),
            queue_ask: AskQueue::with_capacity(orders_count_max / 2),
            order_pool: OrderPool::with_capacity(orders_count_max),
            order_map: OrderMap::with_capacity(orders_count_max),
            counter_order_id: 0,
            counter_exec_id: 0,
            counter_match_id: 0,
            publisher,
        }
    }

    pub fn process_new_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) {
        let mut order = self.process_new_order_decode(header_decoder);

        if self.order_pool.is_full() {
            self.publish_reject(&order, OrdRejReasonEnum::Other);
            // TODO: ERROR HANDLER
            warn!(
                target: "matching_engine_capacity",
                reason = "Book capacity limit reached",
                "OPERATIONAL WARNING: Order book capacity limit reached. New orders are being rejected. Consider investigating load or if orders_count_max (={}) needs adjustment.",
                self.orders_count_max
            );
            return;
        }

        if self
            .order_map
            .contains_key(&(order.account, order.client_order_id))
        {
            self.publish_reject(&order, OrdRejReasonEnum::DuplicateOrder);
            return;
        }

        self.publish_new_order(&order);
        self.route_by_type(&mut order);
    }

    pub fn process_cancel_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) {
        let request = self.process_cancel_order_decode(header_decoder);

        let order_key = (request.account, request.client_order_id);
        let order_idx = if let Some(idx) = self.order_map.get(&order_key) {
            *idx
        } else {
            self.publish_cancel_reject(
                &request,
                CxlRejReasonEnum::UnknownOrder,
                CxlRejResponseToEnum::OrderCancelRequest,
            );
            return;
        };

        let order = self.order_pool.remove(order_idx);
        self.order_map.remove(&order_key);

        if request.side == SideEnum::Buy {
            self.queue_bid.remove(&order_idx);
        } else if request.side == SideEnum::Sell {
            self.queue_ask.remove(&order_idx);
        } else {
            self.reject_invalid_field_cancel(&request, "side");
        }

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
            client_order_id: decoder.cl_ord_id(),
            account: decoder.account(),
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
            original_client_order_id: decoder.orig_cl_ord_id(),
            client_order_id: decoder.cl_ord_id(),
            account: decoder.account(),
            // transact_time: decoder.transact_time_decoder().time(),
            // symbol: decoder.symbol(),
            side: decoder.side(),
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
    fn reject_invalid_field_cancel(&mut self, req: &CancelRequest, field: &'static str) {
        self.publish_cancel_reject(
            req,
            CxlRejReasonEnum::UnknownOrder,
            CxlRejResponseToEnum::OrderCancelRequest,
        );
        // TODO: ERROR HANDLER
        error!(
            target: "matching_engine_critical",
            order_id = ?req.client_order_id,
            order_details = ?req,
            "CRITICAL ERROR: Cancel Request received with NullVal for {field}. Cancel Request rejected. This may indicate message corruption, a gateway bug, or SBE schema mismatch.",
        );
    }

    #[inline(always)]
    fn handle_limit_order<S: SideSpecificContext>(&mut self, aggressor_order: &mut Order) {
        while aggressor_order.leaves_quantity > 0 {
            let Some((resting_idx, _)) = S::peek_best_opposite(self) else {
                break; // No orders on opposite side
            };

            let Some(resting_order) = self.order_pool.get_mut(resting_idx) else {
                return; // TODO: CREATE AN ERROR ENUM AND HANDLER FUNCTION  Log fatal error and exit the program
            };

            // Check prices cross
            if !S::can_cross(aggressor_order.price, resting_order.price) {
                break;
            }

            // Check for self-trading
            if aggressor_order.account == resting_order.account {
                self.publish_cancel(aggressor_order); // TODO: publish cancel with STP as reason
                return;
            }

            execute_trade!(self, aggressor_order, resting_order);
            if resting_order.leaves_quantity == 0 {
                S::pop_best_opposite(self);
                self.remove_order(resting_idx);
            }
        }

        // Any remaining portion is added to the book
        if aggressor_order.leaves_quantity > 0 {
            self.add_order::<S>(aggressor_order);
        }
    }

    #[inline(always)]
    fn handle_market_order<S: SideSpecificContext>(&mut self, aggressor_order: &mut Order) {
        while aggressor_order.leaves_quantity > 0 {
            let Some((resting_idx, _)) = S::peek_best_opposite(self) else {
                // No orders on opposite side
                self.publish_cancel(aggressor_order);
                return;
            };

            let Some(resting_order) = self.order_pool.get_mut(resting_idx) else {
                return; // TODO: CREATE AN ERROR ENUM AND HANDLER FUNCTION  Log fatal error and exit the program
            };

            // Check for self-trading
            if aggressor_order.account == resting_order.account {
                self.publish_cancel(aggressor_order); // TODO: publish cancel with STP as reason
                return;
            }

            execute_trade!(self, aggressor_order, resting_order);
            if resting_order.leaves_quantity == 0 {
                S::pop_best_opposite(self);
                self.remove_order(resting_idx);
            }
        }
    }

    // #[inline(always)]
    // fn handle_fill_or_kill_order<S: SideSpecificContext>(&mut self, aggressor_order: &mut Order) {
    //     let mut remaining_quantity = aggressor_order.leaves_quantity;
    //     let mut fills_queue = Vec::new(); // TODO: instead of vec::new use an array bounded by leaves_quantity

    //     while aggressor_order.leaves_quantity > 0 {
    //         let (resting_idx, _) = match S::peek_best_opposite(self) {
    //             Some(idx) => idx,
    //             _ => {
    //                 // No orders on opposite side - kill
    //                 self.publish_cancel(&aggressor_order);
    //                 return;
    //             }
    //         };

    //         let resting_order = match self.order_pool.get_mut(resting_idx) {
    //             Some(order) => order,
    //             _ => return, // TODO: CREATE AN ERROR ENUM AND HANDLER FUNCTION  Log fatal error and exit the program
    //         };

    //         // Check prices cross
    //         if !S::can_cross(aggressor_order.price, resting_order.price) {
    //             break;
    //         }

    //         // Check for self-trading
    //         if aggressor_order.account == resting_order.account {
    //             self.publish_cancel(&aggressor_order); // TODO: publish cancel with STP as reason
    //             return;
    //         }

    //         remaining_quantity -= min(remaining_quantity, resting_order.leaves_quantity);
    //         fills_queue.push((resting_order, resting_idx));

    //         // Temporarily pop orders off the book
    //         S::pop_best_opposite(self);
    //     }

    //     if remaining_quantity == 0 {
    //         for (resting_order, resting_idx) in fills_queue {
    //             execute_trade!(self, aggressor_order, resting_order, resting_idx);
    //         }
    //     } else {
    //         // COLDER? than fills but idk how often FOK fills if we want to mark as cold... prob not
    //         // If FOK fails add removed orders back to the book
    //         for (resting_order, resting_idx) in fills_queue {
    //             let priority = S::create_priority(resting_order.price, resting_order.sequence_number);
    //             S::push_to_queue(self, resting_idx, priority);
    //         }

    //         // kill the order
    //         self.publish_cancel(&aggressor_order);
    //         return;
    //     }
    // }

    #[inline(always)]
    fn remove_order(&mut self, resting_idx: usize) {
        let resting_order = self.order_pool.remove(resting_idx);
        let order_key = (resting_order.account, resting_order.client_order_id);
        self.order_map.remove(&order_key);
    }

    #[inline(always)]
    fn add_order<S: SideSpecificContext>(&mut self, order: &Order) {
        let order_key: OrderBookKey = (order.account, order.client_order_id);
        let priority = S::create_priority(order.price, order.sequence_number);

        let Ok(order_idx) = self.order_pool.try_insert(*order) else {
            self.reject_due_to_capacity(order);
            return;
        };

        if let Err(CapacityExceededError::Map) = self.order_map.try_insert(order_key, order_idx) {
            self.reject_due_to_capacity(order);
        }

        if let Err(CapacityExceededError::Queue) = S::push_to_queue(self, order_idx, priority) {
            self.reject_due_to_capacity(order);
        }
    }

    fn reject_due_to_capacity(&mut self, order: &Order) {
        debug_assert!(
            self.order_pool.is_full(),
            "Invariant violated: Rejection due to capacity but order_pool below orders_count_max"
        );

        debug_assert!(
            self.queue_bid.is_full() && self.queue_ask.is_full(),
            "Invariant violated: OrderPool full but at least one priority queue is not full"
        );

        self.publish_reject(order, OrdRejReasonEnum::Other);

        warn!(
            target: "matching_engine_capacity",
            reason = "Book capacity limit reached",
            "OPERATIONAL WARNING: Order book capacity limit reached. New orders are being rejected. Consider investigating load or if orders_count_max (={}) needs adjustment.",
            self.orders_count_max
        );
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
