use crate::orderbook::OrderBook;
use crate::types::{AskPriority, BidPriority, CapacityExceededError};

use std::fmt::Debug;

pub struct Buy;
pub struct Sell;

pub trait SideSpecificContext {
    type Priority: Ord + Copy + Debug;
    type OppositePriority: Ord + Copy + Debug;

    fn push_to_queue(
        book: &mut OrderBook,
        order_idx: usize,
        priority: Self::Priority,
    ) -> Result<(), CapacityExceededError>;
    fn peek_best_opposite(book: &OrderBook) -> Option<(usize, Self::OppositePriority)>;
    fn pop_best_opposite(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)>;
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority;
    fn can_cross(aggressor_price: i64, resting_price: i64) -> bool;
}

impl SideSpecificContext for Buy {
    type Priority = BidPriority;
    type OppositePriority = AskPriority;

    #[inline(always)]
    fn push_to_queue(
        book: &mut OrderBook,
        order_idx: usize,
        priority: Self::Priority,
    ) -> Result<(), CapacityExceededError> {
        book.queue_bid.try_insert(order_idx, priority)
    }

    #[inline(always)]
    fn peek_best_opposite(book: &OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.queue_ask
            .peek()
            .map(|(&idx, &priority_val)| (idx, priority_val))
    }

    #[inline(always)]
    fn pop_best_opposite(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.queue_ask.pop()
    }

    #[inline(always)]
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority {
        BidPriority { price, seq_num }
    }

    #[inline(always)]
    fn can_cross(aggressor_price: i64, resting_price: i64) -> bool {
        aggressor_price >= resting_price
    }
}

impl SideSpecificContext for Sell {
    type Priority = AskPriority;
    type OppositePriority = BidPriority;

    #[inline(always)]
    fn push_to_queue(
        book: &mut OrderBook,
        order_idx: usize,
        priority: Self::Priority,
    ) -> Result<(), CapacityExceededError> {
        book.queue_ask.try_insert(order_idx, priority)
    }

    #[inline(always)]
    fn peek_best_opposite(book: &OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.queue_bid
            .peek()
            .map(|(&idx, &priority_val)| (idx, priority_val))
    }

    #[inline(always)]
    fn pop_best_opposite(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.queue_bid.pop()
    }

    #[inline(always)]
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority {
        AskPriority { price, seq_num }
    }

    #[inline(always)]
    fn can_cross(aggressor_price: i64, resting_price: i64) -> bool {
        aggressor_price <= resting_price
    }
}
