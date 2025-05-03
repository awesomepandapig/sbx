import { AuthenticatedWebSocket } from './index';
import { Channel } from './index';

interface Ticker {
  product_id: string;
  price: string;
  volume_24_h: string;
  low_24_h: string;
  high_24_h: string;
  low_52_w: string;
  high_52_w: string;
  price_percent_chg_24_h: string;
  best_bid: string;
  best_bid_quantity: string;
  best_ask: string;
  best_ask_quantity: string;
}

interface Event {
  type: 'snapshot' | 'update';
  tickers: Ticker[];
}

export class TickerChannel extends Channel {
  protected async onSubscribe(): Promise<void> {
    // TODO: Send snapshot of last ticker quote (we can store this in memory)
    return Promise.resolve();
  }

  protected async handleUpdate(
    message: string,
    channelName: string,
  ): Promise<void> {
    const channel = 'ticker';
    const [, productId] = channelName.split('marketdata:ticker:');
    try {
      const parsedMessage = JSON.parse(message);

      const events: Event[] = [
        {
          type: 'update',
          tickers: [parsedMessage],
        },
      ];

      // Send to all clients subscribed to this product
      for (const ws of this.subscribers) {
        const channelSubscriptions = ws.subscriptions.get(channel);
        if (!channelSubscriptions) continue;

        if (channelSubscriptions.has(productId)) {
          ws.sendMessage(`ticker`, events);
        }
      }
    } catch (error) {
      console.error('Error handling update message:', error);
    }
  }
}
