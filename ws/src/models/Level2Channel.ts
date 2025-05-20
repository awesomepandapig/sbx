import { AuthenticatedWebSocket } from './index';
import { Channel } from './index';

interface Update {
  side: 'buy' | 'sell';
  event_time: string;
  price_level: number;
  new_quantity: number;
  message_id: string;
}

interface Event {
  type: 'snapshot' | 'update';
  product_id: string;
  updates: Update[];
}

interface BufferedUpdateItem {
  productId: string;
  update: Update;
}

interface WSState {
  snapshottedProducts: Set<string>;
  updateBuffer: BufferedUpdateItem[];
  sequenceMap: Map<string, string>;
}

function isSeqAfter(a: string, b: string): boolean {
  const [aTime, aSeq] = a.split('-').map(Number);
  const [bTime, bSeq] = b.split('-').map(Number);

  if (aTime > bTime) return true;
  if (aTime < bTime) return false;

  return aSeq > bSeq;
}

export class Level2Channel extends Channel {
  private socketState = new WeakMap<AuthenticatedWebSocket, WSState>();
  private static readonly MAX_BUFFER_SIZE = 10000;

  protected async onSubscribe(
    ws: AuthenticatedWebSocket,
    channel: string,
    newProducts: Set<string>,
  ): Promise<void> {
    // Only send snapshots if there are new products
    if (newProducts.size == 0) {
      return;
    }

    let state = this.socketState.get(ws);
    if (!state) {
      state = {
        snapshottedProducts: new Set<string>(),
        updateBuffer: [],
        sequenceMap: new Map(),
      };
      this.socketState.set(ws, state);
    }

    for (const productId of newProducts) {
      if (state.snapshottedProducts.has(productId)) continue;

      state.snapshottedProducts.delete(productId);
      state.sequenceMap.delete(productId);

      // #3. Make a REST request for the order book snapshot from the REST feed.
      const snapshot = await this.fetchSnapshot(productId);
      if (!snapshot) continue;

      const snapshotSeq = snapshot.sequence_num;
      state.sequenceMap.set(productId, snapshotSeq);

      const snapshotEvent: Event = {
        type: 'snapshot',
        product_id: productId,
        updates: snapshot.updates,
      };

      ws.sendMessage('l2_data', [snapshotEvent]);

      // #4. Emit any queued messages, discarding sequence numbers before or equal to the snapshot sequence number.
      const buffered = state.updateBuffer.filter(
        (item) =>
          item.productId === productId &&
          isSeqAfter(item.update.message_id, snapshotSeq),
      );

      if (buffered.length > 0) {
        const updates: Event = {
          type: 'update',
          product_id: productId,
          updates: buffered.map((b) => b.update),
        };
        ws.sendMessage('l2_data', [updates]);
      }

      // Clear only the used part of buffer
      state.updateBuffer = state.updateBuffer.filter(
        (item) =>
          item.productId !== productId ||
          isSeqAfter(item.update.message_id, snapshotSeq),
      );

      // #5. After playback is complete, emit real-time stream messages as they arrive.
      state.snapshottedProducts.add(productId);
    }
  }

  private async fetchSnapshot(
    productId: string,
  ): Promise<{ updates: Update[]; sequence_num: string } | null> {
    try {
      // const response = await fetch(`http://localhost:3000/snapshot?productId=${productId}`);
      const response = await fetch(`http://localhost:3000`);
      if (!response.ok) {
        console.error(
          `Error fetching snapshot for ${productId}: ${response.status} ${response.statusText}`,
        );
        return null;
      }
      return await response.json();
    } catch (error) {
      console.error(`Snapshot fetch failed for ${productId}:`, error);
      return null;
    }
  }

  protected async handleUpdate(
    message: string,
    channelName: string,
  ): Promise<void> {
    const channel = 'level2';
    const [, productId] = channelName.split(`marketdata:${channel}:`);
    try {
      const parsedMessage = JSON.parse(message) as Update;

      // #2. Queue any messages received over the Redis channel.
      const event: Event[] = [
        {
          type: 'update',
          product_id: productId,
          updates: [parsedMessage],
        },
      ];

      // Send to all clients subscribed to this product
      for (const ws of this.subscribers) {
        const subs = ws.subscriptions.get(channel);
        if (!subs) continue;

        const state = this.socketState.get(ws);
        if (!state) continue;

        if (state.snapshottedProducts.has(productId)) {
          // #5 continued: Emit real-time stream messages as they arrive.
          ws.sendMessage('l2_data', event);
        } else {
          // #2 continued: Queue updates in memory until snapshot is sent
          // Hard cap: limit buffer size
          if (state.updateBuffer.length >= Level2Channel.MAX_BUFFER_SIZE) {
            state.updateBuffer.shift();
          }
          state.updateBuffer.push({ productId, update: parsedMessage });
        }
      }
    } catch (error) {
      console.error(
        `Error parsing or handling update message for ${productId}:`,
        message,
        error,
      );
    }
  }
}
