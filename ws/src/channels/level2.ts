import { activeProducts, waitForRedis, redisClient } from '../config/index';
import { AuthenticatedWebSocket, OrderBook, Update, Order } from 'models/index';

interface Event {
  type: 'snapshot' | 'update';
  product_id: string;
  updates: Update[];
}

const level2Subs = new Map<AuthenticatedWebSocket, Map<string, Update[]>>();
const OrderBooks = new Map<string, OrderBook>();

// Stream IDs for tracking Redis streams
let orderStreamId = '0-0';
let matchStreamId = '0-0';

export function subscribe(ws: AuthenticatedWebSocket, products: Set<string>) {
  // If no products, subscribe to all products
  if (products.size === 0) {
    products = activeProducts;
  }

  // Initialize subscription map for this WebSocket if not exists
  if (!level2Subs.has(ws)) {
    level2Subs.set(ws, new Map());
  }

  // Send subscription confirmation
  ws.sendMessage('subscriptions', [
    { subscriptions: { level2: Array.from(products) } },
  ]);

  // Send initial snapshot for each product
  const events: Event[] = [];
  for (const productId of products) {
    const orderBook = OrderBooks.get(productId);
    if (!orderBook) continue;

    const snapshot = orderBook.getSnapshot();
    // Store the snapshot for this product for future diff comparisons
    const subscriber = level2Subs.get(ws);
    if (!subscriber) continue;

    subscriber.set(productId, snapshot);

    events.push({
      type: 'snapshot',
      product_id: productId,
      updates: snapshot,
    });
  }
  ws.sendMessage('l2_data', events);
}

export function unsubscribe(ws: AuthenticatedWebSocket, products: Set<string>) {
  // Get the current subscription
  const subscribedProducts = level2Subs.get(ws);

  // If not subscribed, send empty subscription message
  if (!subscribedProducts) {
    ws.sendMessage('subscriptions', [{ subscriptions: {} }]);
    return;
  }

  // If no products specified, unsubscribe from all
  if (products.size === 0) {
    level2Subs.delete(ws);
    ws.sendMessage('subscriptions', [{ subscriptions: {} }]);
    return;
  }

  // Remove specified products
  for (const product of products) {
    subscribedProducts.delete(product);
  }

  // Update subscription state
  if (subscribedProducts.size === 0) {
    level2Subs.delete(ws);
  }

  // Send updated subscription
  ws.sendMessage('subscriptions', [
    { subscriptions: { level2: Array.from(subscribedProducts.keys()) } },
  ]);
}

export function cleanup(ws: AuthenticatedWebSocket) {
  level2Subs.delete(ws);
}

async function processNewOrders() {
  for (const productId of activeProducts) {
    const orderStream = `${productId}:new`;
    const result = await redisClient.xRead([
      { key: orderStream, id: orderStreamId },
    ]);

    if (!result?.length) continue;

    const stream = result[0];
    orderStreamId = stream.messages.at(-1)?.id ?? orderStreamId;

    const orderBook = OrderBooks.get(productId);
    if (!orderBook) continue;

    for (const message of stream.messages) {
      const order = new Order(message.message);
      orderBook.addOrder(order);
    }
  }
}

async function processNewMatches() {
  for (const productId of activeProducts) {
    const matchStream = `${productId}:matches`;
    const result = await redisClient.xRead([
      { key: matchStream, id: matchStreamId },
    ]);

    if (!result?.length) return;

    const stream = result[0];
    matchStreamId = stream.messages.at(-1)?.id ?? matchStreamId;

    const orderBook = OrderBooks.get(productId);
    if (!orderBook) continue;

    for (const message of stream.messages) {
      const order = new Order(message.message);
      orderBook.removeOrder(order);
    }
  }
}

function broadcastUpdates() {
  level2Subs.forEach((productMap, ws) => {
    const events: Event[] = [];

    productMap.forEach((previousSnapshot, productId) => {
      const orderBook = OrderBooks.get(productId);
      if (!orderBook) return;

      const currentSnapshot = orderBook.getSnapshot();
      const diffs = orderBook.getDiffs(currentSnapshot, previousSnapshot);

      if (diffs.length > 0) {
        events.push({
          type: 'update',
          product_id: productId,
          updates: diffs,
        });

        // Update the stored snapshot after sending diffs
        productMap.set(productId, currentSnapshot);
      }
    });

    if (events.length > 0) {
      ws.sendMessage('l2_data', events);
    }
  });
}

const initialize = async () => {
  // Wait for Redis initialization to complete
  await waitForRedis();

  // Initialize order books for all active products
  for (const productId of activeProducts) {
    OrderBooks.set(productId, new OrderBook());
  }

  // Start processing
  setInterval(async () => {
    try {
      await processNewOrders();
      await processNewMatches();
      broadcastUpdates();
    } catch (error) {
      console.error('Error in level2 processing:', error);
    }
  }, 1000);
};

initialize();
