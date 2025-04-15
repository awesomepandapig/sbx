import { AuthenticatedWebSocket } from './index';
import { Channel } from './index';

export class TickerChannel extends Channel {
    protected async onSubscribe(): Promise<void> {
        return Promise.resolve();
    }

  protected async handleUpdate(message: string): Promise<void> {
    const channel = 'ticker';
    try {
      const parsedMessage = JSON.parse(message);
      const { product_id: productId } = parsedMessage;

      // Send to all clients subscribed to this product
      for (const ws of this.subscribers) {
        const channelSubscriptions = ws.subscriptions.get(channel);
        if (!channelSubscriptions) continue;

        if (channelSubscriptions.has(productId)) {
          ws.sendMessage(`ticker`, [parsedMessage]);
        }
      }
    } catch (error) {
      console.error('Error handling update message:', error);
    }
  }
}
