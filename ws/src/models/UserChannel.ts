import { AuthenticatedWebSocket } from './index';
import { Channel } from './index';

interface Order {
  id: string;
  product_id: string;
  user_id: string;
  side: string;
  type: string;
  created_at: string;
  status: string;
  executed_value: string;
  settled: string;
  size: string;
  price?: string;
  cancel_after: string;
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
    let batch = [];
    for (const productId of products) {
      try {
        const userId = ws.user_id;
        if (!userId) return;

        // TODO: get list of user's orders from db
        // const orders = await redisClient.hGet('snapshot', productId);
        // if (!orders) continue;
        // for (const order of orders) {
        //     if (order.status === 'open') {
        //         batch.push(order);
        //         if (batch.length === 50) {
        //             events.push({
        //                 type: 'snapshot',
        //                 product_id: productId,
        //                 orders: [...batch],
        //               });
        //               batch = [];
        //             }
        //     }
        // }
        // // Add any remaining orders to the snapshot
        // if (batch.length > 0) {
        //     events.push({
        //       type: 'snapshot',
        //       product_id: productId,
        //       orders: [...batch],
        //     });
        //   }
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
