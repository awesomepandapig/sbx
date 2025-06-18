use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;

#[derive(Debug, Clone, Copy)]
pub struct Order {
    // Hot fields (accessed frequently during matching) - first cache line (64 bytes)
    pub prev_order_idx: Option<usize>, // 8 bytes -
    pub next_order_idx: Option<usize>, // 8 bytes -

    pub leaves_quantity: i64, // 8 bytes - Remaining quantity to be filled
    pub price: i64,           // 8 bytes - Price for Limit orders
    pub cumulative_quantity: i64, // 8 bytes - Cumulative quantity filled
    pub total_notional: i128, // 16 bytes - Total value of fills
    pub sequence_number: u64, // 8 bytes - Monotonic identifier for order
    pub quantity: i64,        // 8 bytes - Original quantity of the order
    pub side: SideEnum,       // 1 bytes - Buy or Sell
    pub r#type: OrdTypeEnum,  // 1 bytes - Limit or Market
    // 64 bytes total for first cache line

    // Cold fields (rarely accessed during matching) - subsequent cache lines
    pub client_order_id: UuidType, // 16 bytes - Client Order ID
    pub account: UuidType,         // 16 bytes - Account ID
    pub transact_time: u64,        // 8 bytes - Time of transaction from client
    pub symbol: SymbolType, // 6 bytes - Instrument symbol
}

impl Order {
    pub const fn key(&self) -> OrderKey {
        (self.account, self.client_order_id)
    }

    pub fn fill(&mut self, qty: i64, price: i64) {
        self.cumulative_quantity += qty;
        self.leaves_quantity -= qty;
        self.total_notional += i128::from(qty) * i128::from(price);
    }

    pub fn avg_px(&self) -> i64 {
        if self.cumulative_quantity == 0 {
            return 0;
        }
        let avg = self.total_notional / i128::from(self.cumulative_quantity);
        i64::try_from(avg).expect("avg_px: VWAP out of i64 range — invariant broken") // TODO: NO EXPECTS
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CancelRequest {
    pub original_client_order_id: UuidType, // 16 bytes - Original Client Order ID
    pub client_order_id: UuidType,          // 16 bytes - Client Order ID
    pub account: UuidType,                  // 16 bytes - Account ID
                                            // pub transact_time: u64,                 // 8 bytes - Time of transaction from client
                                            // pub symbol: SymbolType,                 // 6 bytes - Instrument symbol
                                            // pub side: SideEnum, // 1 bytes - Buy or Sell
}

pub type UuidType = u128;
pub type SymbolType = [u8; 6];
pub type OrderKey = (UuidType, UuidType);
