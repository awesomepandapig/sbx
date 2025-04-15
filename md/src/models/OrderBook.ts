import { Order } from './order';

export interface Update {
  side: 'ask' | 'bid';
  event_time: string;
  price_level: number;
  new_quantity: number;
}

export class OrderBook {
  private bids = new Map<number, Map<string, Order>>();
  private asks = new Map<number, Map<string, Order>>();
  private snapshot: Update[] = [];
  private lastUpdateTime: number = Date.now();
  
  private volumeStartTime: number;
  private volume24h: number = 0;
  private bestBid: number = 0;
  private bestAsk: number = 0;
  private high24h: number = 0;
  private low24h: number = 0;
  private openPrice24h: number = 0;
  private lastPrice: number = 0;

  constructor() {
    this.volumeStartTime = this.getDayStart(Date.now());
  }
  
  private getDayStart(timestamp: number): number {
    return new Date(timestamp).setHours(0, 0, 0, 0);
  }

  public addOrder(order: Order) {
    const priceMap = order.side === 'buy' ? this.bids : this.asks;
    if (!priceMap.has(order.price)) {
      priceMap.set(order.price, new Map());
    }
    const bucket = priceMap.get(order.price);
    if (!bucket) return;

    bucket.set(order.id, order);
    this.lastUpdateTime = order.created_at;
  }

  public removeOrder(order: Order) {
    const priceMap = order.side === 'buy' ? this.bids : this.asks;
    if (priceMap.has(order.price)) {
      const bucket = priceMap.get(order.price);
      if (!bucket) return;

      bucket.delete(order.id);
      if (bucket.size === 0) {
        priceMap.delete(order.price);
      }
    }
    this.lastUpdateTime = order.created_at;
  }

  public updateSnapshot() {
    let bestBid = this.getBestBid();
    let bestAsk = this.getBestAsk();
    const event_time = new Date(this.lastUpdateTime).toISOString();

    if (bestBid === 0 && bestAsk === 0) return;

    if (bestBid === 0) bestBid = bestAsk;
    if (bestAsk === 0) bestAsk = bestBid;

    // Calculate tick size
    const spread = Math.max(1, bestAsk - bestBid);
    const depth = 20;

    
    let tickSize = Math.max(1, Math.ceil(spread / depth));

    // Skip bucketing when order count is low
    if (this.bids.size + this.asks.size < 20) {
      tickSize = 1;
    }    

    // Pre-allocate arrays for better performance
    const updates: Update[] = [];

    // Process bids (use a more efficient algorithm)
    const bidPrices = Array.from(this.bids.keys()).sort((a, b) => b - a);
    const bidBuckets = new Map<number, number>();

    // Aggregate bids
    for (const price of bidPrices) {
      const bucket = Math.floor(price / tickSize) * tickSize;
      const orders = this.bids.get(price);
      if (!orders) continue;

      // Aggregate the total size of orders in this bucket
      let totalSize = 0;
      for (const order of orders.values()) {
        totalSize += order.size;
      }

      bidBuckets.set(bucket, (bidBuckets.get(bucket) ?? 0) + totalSize);
    }

    // Process bids (use a more efficient algorithm)
    const askPrices = Array.from(this.asks.keys()).sort((a, b) => b - a);
    const askBuckets = new Map<number, number>();

    // Aggregate asks
    for (const price of askPrices) {
      const bucket = Math.floor(price / tickSize) * tickSize;
      const orders = this.asks.get(price);
      if (!orders) continue;

      // Aggregate the total size of orders in this bucket
      let totalSize = 0;
      for (const order of orders.values()) {
        totalSize += order.size;
      }

      askBuckets.set(bucket, (askBuckets.get(bucket) ?? 0) + totalSize);
    }

    // Create updates from buckets
    for (const [price, size] of bidBuckets) {
      updates.push({
        side: 'bid',
        price_level: price,
        new_quantity: size,
        event_time: event_time,
      });
    }

    for (const [price, size] of askBuckets) {
      updates.push({
        side: 'ask',
        price_level: price,
        new_quantity: size,
        event_time: event_time,
      });
    }

    this.snapshot = updates;
  }

  public updateBestPrices(): void {
    this.bestBid = this.bids.size > 0 ? Math.max(...Array.from(this.bids.keys())) : 0;
    this.bestAsk = this.asks.size > 0 ? Math.min(...Array.from(this.asks.keys())) : 0;
  }

  public getBestBid(): number {
    return this.bestBid;
  }
  
  public getBestAsk(): number {
    return this.bestAsk;
  }

  public getQty(side: string, price_level: number): number {
    const priceMap = side === 'buy' ? this.bids : this.asks;
    const bucket = priceMap.get(price_level);
    return bucket ? bucket.size : 0;
  }

  public updatePriceData(order: Order): void {
    const currentTimestamp = order.created_at;
    const currentPrice = order.price;
    this.lastPrice = currentPrice;
    
    // Reset 24h data if we've crossed to a new day
    const currentDayStart = this.getDayStart(currentTimestamp);
    if (currentDayStart > this.volumeStartTime) {
      this.volumeStartTime = currentDayStart;
      this.volume24h = 0;
      this.high24h = currentPrice;
      this.low24h = currentPrice;
      this.openPrice24h = currentPrice;
    }

    // Update high/low
    if (currentPrice > this.high24h) this.high24h = currentPrice;
    if (this.low24h === 0 || currentPrice < this.low24h) this.low24h = currentPrice;

    this.volume24h += order.executed_value;
  }

  public getTickerData(): {
    price: number,
    volume_24h: number,
    low_24h: number,
    high_24h: number,
    price_percent_chg_24h: number,
    best_bid: number,
    best_ask: number
  } {
    let percentChange = 0;
    if (this.openPrice24h > 0) {
      const priceDifference = this.lastPrice - this.openPrice24h;
      const relativeDifference = priceDifference / this.openPrice24h;
      percentChange = relativeDifference * 100;
    }
    
    return {
      price: this.lastPrice,
      volume_24h: this.volume24h,
      low_24h: this.low24h,
      high_24h: this.high24h,
      price_percent_chg_24h: percentChange,
      best_bid: this.bestBid,
      best_ask: this.bestAsk
    };
  }


  public getSnapshot(): Update[] {
    return this.snapshot;
  }

  public getDiffs(current: Update[], previous: Update[]): Update[] {
    const diffs: Update[] = [];
    const event_time = new Date(this.lastUpdateTime).toISOString();

    const previousBids = new Map<number, number>();
    const previousAsks = new Map<number, number>();

    for (const update of previous) {
      if(update.side === 'bid') {
        previousBids.set(update.price_level, update.new_quantity);
      } else {
        previousAsks.set(update.price_level, update.new_quantity);
      }
    }

    const currentBids = new Map<number, number>();
    const currentAsks = new Map<number, number>();

    for (const update of current) {
      if (update.side === 'bid') {
        currentBids.set(update.price_level, update.new_quantity);
      } else {
        currentAsks.set(update.price_level, update.new_quantity);
      }
    }
    
    // // Counterparty collision logic

    // // If there is one bid or one ask
    // if (
    //   (previousBids.size === 1 ||
    //   previousAsks.size === 1) &&
    //   (currentBids.size === 1 ||
    //   currentAsks.size === 1)
    // ) {

    //   // Check if the price cancels the other
    //   const [currentBid] = currentBids.keys();
    //   const [previousBid] = previousBids.keys();
    //   const [currentAsk] = currentAsks.keys();
    //   const [previousAsk] = previousAsks.keys();

    //   // If a new bid cancels the previous ask
    //   if (
    //     typeof currentBid === 'number' &&
    //     typeof previousAsk === 'number' &&
    //     currentBid >= previousAsk
    //   ) {
    //     diffs.push({
    //       side: 'ask',
    //       price_level: previousAsk,
    //       new_quantity: 0,
    //       event_time,
    //     });
    //     previousAsks.delete(previousAsk);
    //   }

    //   // If a new ask cancels the previous bid
    //   if (
    //     typeof currentAsk === 'number' &&
    //     typeof previousBid === 'number' &&
    //     currentAsk <= previousBid
    //   ) {
    //     diffs.push({
    //       side: 'bid',
    //       price_level: previousBid,
    //       new_quantity: 0,
    //       event_time,
    //     });
    //     previousBids.delete(previousBid);
    //   }
    // }
  
    // Compare current snapshot with previous
    for (const update of current) {
      let previousQty = 0;
      if (update.side === 'bid') {
        previousQty = previousBids.get(update.price_level) || 0;
        previousBids.delete(update.price_level);
      } else {
        previousQty = previousAsks.get(update.price_level) || 0;
        previousAsks.delete(update.price_level);
      }
  
      if (previousQty !== update.new_quantity) {
        diffs.push({
          ...update,
          event_time,
        });
      }
    }
    
    for (const [price, _] of previousBids) {
      diffs.push({
        side: 'bid',
        price_level: price,
        new_quantity: 0,
        event_time,
      });
    }

    for (const [price, _] of previousAsks) {
      diffs.push({
        side: 'ask',
        price_level: price,
        new_quantity: 0,
        event_time,
      });
    }
  
    return diffs;
  }
}