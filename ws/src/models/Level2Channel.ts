import { redisClient } from '../config/index';
import { AuthenticatedWebSocket } from './index';
import { Channel } from './index';

interface Update {
  side: 'buy' | 'sell';
  event_time: string;
  price_level: number;
  new_quantity: number;
}

interface Event {
  type: 'snapshot' | 'update';
  product_id: string;
  updates: Update[];
}

export class Level2Channel extends Channel {
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
        const snapshotRaw = await redisClient.hGet('snapshot', productId);
        if (!snapshotRaw) continue;

        const snapshot = JSON.parse(snapshotRaw);
        events.push({
          type: 'snapshot',
          product_id: productId,
          updates: snapshot,
        });
      } catch (error) {
        console.error(`Failed to get snapshot for ${productId}:`, error);
      }
    }
    if (events.length > 0) {
      ws.sendMessage('l2_data', events);
    }
  }

  protected async handleUpdate(
    message: string,
    channelName: string,
  ): Promise<void> {
    const channel = 'level2';
    const [, productId] = channelName.split('marketdata:level2:');
    try {
      const parsedMessage = JSON.parse(message);

      const events = [
        {
          type: 'update',
          product_id: productId,
          updates: [parsedMessage],
        },
      ];

      // Send to all clients subscribed to this product
      for (const ws of this.subscribers) {
        const channelSubscriptions = ws.subscriptions.get(channel);
        if (!channelSubscriptions) continue;

        if (channelSubscriptions.has(productId)) {
          ws.sendMessage(`l2_data`, events);
        }
      }
    } catch (error) {
      console.error('Error handling update message:', error);
    }
  }
}
