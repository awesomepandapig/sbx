import { activeProducts, redisClient, waitForRedis } from '../config/index';
import { AuthenticatedWebSocket, SubscribeMessage, UnsubscribeMessage } from 'models/index';

interface Update {
  side: 'ask' | 'bid';
  event_time: string;
  price_level: number;
  new_quantity: number;
}

interface Event {
  type: 'snapshot' | 'update';
  product_id: string;
  updates: Update[];
}

// Track subscriptions: WebSocket -> Set of product IDs
const subscriptions = new Map<AuthenticatedWebSocket, Set<string>>();

export async function subscribe(
  ws: AuthenticatedWebSocket,
  message: SubscribeMessage
) {
  // If no products, subscribe to all products
  let products = new Set(message.product_ids);
  if (products.size === 0) {
    products = new Set(activeProducts);
  }

  // Initialize subscription map for this WebSocket if not exists
  if (!subscriptions.has(ws)) {
    subscriptions.set(ws, new Set());
  }

  const subscribedProducts = subscriptions.get(ws)!;
  const newProducts = new Set<string>();

  // Find products that are newly subscribed
  for (const productId of products) {
    if (!subscribedProducts.has(productId)) {
      newProducts.add(productId);
      subscribedProducts.add(productId);
    }
  }

  // Send subscription confirmation
  ws.sendMessage('subscriptions', [
    { subscriptions: { level2: Array.from(subscribedProducts) } },
  ]);

  // Fetch and send initial snapshots for newly subscribed products
  await sendSnapshots(ws, newProducts);
}

async function sendSnapshots(
  ws: AuthenticatedWebSocket,
  products: Set<string>,
) {
  const events: Event[] = [];

  for (const productId of products) {
    try {
      // TODO: Get snapshot from redis
      const snapshotKey = `snapshot`;
      const snapshotRaw = await redisClient.hGet(snapshotKey, productId);
      if (!snapshotRaw) return;
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
  // Send all snapshots to the client
  if (events.length > 0) {
    ws.sendMessage('l2_data', events);
  }
}

export function unsubscribe(ws: AuthenticatedWebSocket, products: Set<string>) {
  // Get the current subscription
  const subscribedProducts = subscriptions.get(ws);

  // If not subscribed, send empty subscription message
  if (!subscribedProducts) {
    ws.sendMessage('subscriptions', [{ subscriptions: {} }]);
    return;
  }

  // If no products specified, unsubscribe from all
  if (products.size === 0) {
    subscriptions.delete(ws);
    ws.sendMessage('subscriptions', [{ subscriptions: {} }]);
    return;
  }

  // Remove specified products
  for (const product of products) {
    subscribedProducts.delete(product);
  }

  // Update subscription state
  if (subscribedProducts.size === 0) {
    subscriptions.delete(ws);
  }

  // Send updated subscription
  ws.sendMessage('subscriptions', [
    { subscriptions: { level2: Array.from(subscribedProducts) } },
  ]);
}

export function cleanup(ws: AuthenticatedWebSocket) {
  subscriptions.delete(ws);
}

function handleUpdate(message: string) {
  const parsedMessage = JSON.parse(message);
  const productId = parsedMessage.product_id;
  // Send to all clients subscribed to this product
  for (const [ws, subscribedProducts] of subscriptions) {
    if (subscribedProducts.has(productId)) {
      ws.sendMessage('l2_data', [parsedMessage]);
    }
  }
}

let initialized = false;

async function initialize() {
  if (initialized) return;

  await waitForRedis();

  const redisPubSub = redisClient.duplicate();
  await redisPubSub.connect();
  
  const channels = Array.from(activeProducts).map(
    (productId) => `${productId}:updates`,
  );
  await redisPubSub.subscribe(channels, (message) => {
    // console.log(message);
    handleUpdate(message);
  });

  console.log('Level2 channel initialized with pub/sub model');
  initialized = true;
}

initialize();
