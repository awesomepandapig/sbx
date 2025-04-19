struct CandleBuilder {
    start: i64,
    open: i64,
    high: i64,
    low: i64,
    close: i64,
    volume: i64,
}

impl CandleBuilder {
    pub fn new() -> Self {
        let now = Utc::now();

        // TODO: Load cached_start from Redis
        let cached_start = now;

        return Self {
            start = cached_start;
            open = 0;
            high = 0;
            low = 0;
            close = 0;
            volume = 0;
        }
    }

    fn recover(&mut self, missed_orders: Vec<Order>) {
        for order in missed_orders {
            self.process_order(&order);
        }
    }

    fn process_order(&mut self, order: &Order) {
        self.close = order.price; // set the close on each order

        // If timestamp is 5 minutes ahead
        if(order.timestamp >= self.start + 300) {
            self.reset_values();
            self.start += 300;
            // TODO: Store latest start time in redis (for disaster recovery)
        }

        self.update_values(&order);
    }

    fn update_values(&mut self, order: &Order) {
        if(self.open == 0) {
            self.open = order.price;
        }
        if(self.high < order.price) {
            self.high = order.price;
        }
        if(self.low == 0 || self.low > order.price) {
            self.low = order.price;
        }
        self.volume += order.size;
    }

    fn reset_values(&mut self) {
        self.open = 0;
        self.high = 0;
        self.low = 0;
        self.close = 0;
        self.volume = 0;
    }

    fn emit(&mut self) {
        if self.open == 0 {
            return; // skip empty candle
        }

        let payload = {
            start: self.start,
            open: self.open,
            high: self.high,
            low: self.low,
            close: self.close,
            volume: self.volume,
            product_id: // TODO:
        };

        // TODO: send over redis channel the OHLCV data
    }
}