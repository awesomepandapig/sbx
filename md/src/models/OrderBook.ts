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

  public addOrder(order: Order) {
    const priceMap = order.side === 'buy' ? this.bids : this.asks;
    if (!priceMap.has(order.price)) {
      priceMap.set(order.price, new Map());
    }
    const bucket = priceMap.get(order.price);
    if (!bucket) return;

    bucket.set(order.id, order);
    this.lastUpdateTime = order.created_at;
    this.updateSnapshot();
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
    this.updateSnapshot();
  }

  public updateSnapshot() {
    const highestBid = this.getHighestBid();
    const lowestAsk = this.getLowestAsk();

    const spread = lowestAsk - highestBid;
    const depth = 20;
    const rawTickSize = spread / depth;
    const minTickSize = 1;
    const tickSize = Math.max(rawTickSize, minTickSize);

    const bidBuckets = new Map<number, number>();
    const askBuckets = new Map<number, number>();

    // Aggregate bids
    for (const [price, orders] of this.bids) {
      const bucket = Math.floor(price / tickSize) * tickSize;

      // Aggregate the total size of orders in this bucket
      let totalSize = 0;
      for (const order of orders.values()) {
        totalSize += order.size;
      }

      bidBuckets.set(bucket, (bidBuckets.get(bucket) ?? 0) + totalSize);
    }

    // Aggregate asks
    for (const [price, orders] of this.asks) {
      const bucket = Math.ceil(price / tickSize) * tickSize;
      let totalSize = 0;
      for (const order of orders.values()) {
        totalSize += order.size;
      }
      askBuckets.set(bucket, (askBuckets.get(bucket) ?? 0) + totalSize);
    }

    const updates: Update[] = [];

    // Sort the orders
    const sortedBids = Array.from(bidBuckets.entries()).sort(
      (a, b) => b[0] - a[0],
    );

    // Sort asks (lowest first)
    const sortedAsks = Array.from(askBuckets.entries()).sort(
      (a, b) => a[0] - b[0],
    );

    for (const [price, size] of sortedBids) {
      updates.push({
        side: 'bid',
        price_level: price,
        new_quantity: size,
        event_time: new Date(this.lastUpdateTime).toISOString(),
      });
    }

    for (const [price, size] of sortedAsks) {
      updates.push({
        side: 'ask',
        price_level: price,
        new_quantity: size,
        event_time: new Date(this.lastUpdateTime).toISOString(),
      });
    }

    this.snapshot = updates;
  }

  public getHighestBid(): number {
    if (this.bids.size === 0) return 0;
    return Math.max(...Array.from(this.bids.keys()));
  }

  public getLowestAsk(): number {
    if (this.asks.size === 0) return 0;
    return Math.min(...Array.from(this.asks.keys()));
  }

  public getSnapshot(): Update[] {
    return this.snapshot;
  }

  // TODO: READ
  public getDiffs(current: Update[], previous: Update[]): Update[] {
    // Create maps for faster lookups
    const previousMap = new Map<string, Update>();
    for (const update of previous) {
      previousMap.set(`${update.side}-${update.price_level}`, update);
    }

    const currentMap = new Map<string, Update>();
    for (const update of current) {
      currentMap.set(`${update.side}-${update.price_level}`, update);
    }

    const changes: Update[] = [];

    // Check for new or modified price levels
    for (const [key, update] of currentMap.entries()) {
      const previousUpdate = previousMap.get(key);
      if (
        !previousUpdate ||
        previousUpdate.new_quantity !== update.new_quantity
      ) {
        changes.push(update);
      }
    }

    // Check for removed price levels
    for (const [key, update] of previousMap.entries()) {
      if (!currentMap.has(key)) {
        changes.push({
          ...update,
          new_quantity: 0, // Price level removed
          event_time: new Date(this.lastUpdateTime).toISOString(),
        });
      }
    }

    return changes;
  }
}
