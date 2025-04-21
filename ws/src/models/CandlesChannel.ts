import { redisClient } from '../config/index';
import { AuthenticatedWebSocket } from './index';
import { Channel } from './index';

interface CandleUpdate {
  start: string;
  open: string;
  high: string;
  low: string;
  close: string;
  volume: string;
}

interface Event {
  type: 'snapshot' | 'update';
  candles: CandleUpdate[];
}

export class CandlesChannel extends Channel {
  // Override the hook to send snapshots for new product subscriptions
  protected async onSubscribe(
    ws: AuthenticatedWebSocket,
    channel: string,
    newProducts: Set<string>,
  ): Promise<void> {
    // Only send snapshots if there are new products
    if (newProducts.size > 0) {
      await this.sendSnapshots(ws, newProducts);
    }
  }

  private async sendSnapshots(
    ws: AuthenticatedWebSocket,
    products: Set<string>,
  ) {
    const events: Event[] = [];

    for (const productId of products) {
      try {
        // TODO:
        const snapshotRaw = await redisClient.hGet('snapshot', productId);
        if (!snapshotRaw) continue;

        const snapshot = JSON.parse(snapshotRaw);
        events.push({
          type: 'snapshot',
          candles: snapshot,
        });
      } catch (error) {
        console.error(`Failed to get snapshot for ${productId}:`, error);
      }
    }
    if (events.length > 0) {
      ws.sendMessage('candles', events);
    }
  }

  protected async handleUpdate(
    message: string,
    channelName: string,
  ): Promise<void> {
    const channel = 'candles';
    const [, productId] = channelName.split('marketdata:candles:');
    try {
      const parsedMessage = JSON.parse(message);

      const events = [{
        type: "update",
        candles: [parsedMessage]
      }]

      // Send to all clients subscribed to this product
      for (const ws of this.subscribers) {
        const channelSubscriptions = ws.subscriptions.get(channel);
        if (!channelSubscriptions) continue;

        if (channelSubscriptions.has(productId)) {
          ws.sendMessage(`candles`, events);
        }
      }
    } catch (error) {
      console.error('Error handling update message:', error);
    }
  }
}