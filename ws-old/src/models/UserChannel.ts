import { AuthenticatedWebSocket } from './index';
import { Channel } from './index';

import { db } from 'db';
import { order } from 'db/schema';
import { eq, and } from 'drizzle-orm';

interface Order {
  id: string;
  product_id: string;
  user_id: string;
  side: 'buy' | 'sell';
  type: 'market' | 'limit';
  created_at: string;
  executed_value: string;
  status: 'open' | 'done' | 'cancelled';
  settled: boolean;
  price: string | null;
  cancel_after: 'min' | 'hour' | null;
  size: string;
}

interface Event {
  type: 'snapshot' | 'update';
  product_id: string;
  updates: Order[];
}

export class UserChannel extends Channel {
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
        const userId = ws.user_id;
        if (!userId) return;

        // TODO: get list of user's open orders from db
        const rows = await db
          .select()
          .from(order)
          .where(
            and(
              eq(order.status, 'open'),
              eq(order.user_id, userId),
              eq(order.product_id, productId),
            ),
          );

        const batchSize = 50;
        let batch: Order[] = [];

        for (const ord of rows) {
          batch.push(ord);
          if (batch.length === batchSize) {
            events.push({
              type: 'snapshot',
              product_id: productId,
              updates: [...batch],
            });
            batch = [];
          }
        }

        // Add any remaining orders to the snapshot
        if (batch.length > 0) {
          events.push({
            type: 'snapshot',
            product_id: productId,
            updates: [...batch],
          });
        }
      } catch (error) {
        console.error(`Failed to get snapshot for ${productId}:`, error);
      }
    }
    const streamKey = 'user';
    for (const event of events) {
      ws.sendMessage(streamKey, [event]);
    }
  }

  protected async handleUpdate(message: string): Promise<void> {
    const channel = 'user';
    try {
      const parsedMessage = JSON.parse(message);
      const { product_id: productId } = parsedMessage[0];

      // Send to all clients subscribed to this product
      for (const ws of this.subscribers) {
        const channelSubscriptions = ws.subscriptions.get(channel);
        if (!channelSubscriptions) continue;

        if (channelSubscriptions.has(productId)) {
          ws.sendMessage(`user`, [parsedMessage]);
        }
      }
    } catch (error) {
      console.error('Error handling update message:', error);
    }
  }
}
