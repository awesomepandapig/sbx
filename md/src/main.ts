import { activeProducts, redisClient, redisSubscriber, closeRedisConnections } from './config/index';
import { OrderBook, Order } from './models/index';

const OrderBooks = new Map<string, OrderBook>();
const orderStreamIds = new Map<string, string>();
const matchStreamIds = new Map<string, string>();
const dirtyFlags = new Map<string, boolean>();

async function readFromStream(
  productId: string,
  suffix: string,
  streamIdMap: Map<string, string>,
) {
  const streamKey = `product:${productId}:${suffix}`;
  const lastSeenId = streamIdMap.get(productId) ?? '0-0';

  const result = await redisClient.xRead(
    [{ key: streamKey, id: lastSeenId }]
  );
  if (!result?.length) {
    return null;
  }

  const stream = result[0];
  const lastId = stream.messages.at(-1)?.id;
  if (lastId) {
    streamIdMap.set(productId, lastId);
  }

  return stream.messages;
}

async function processNewOrders(productId: string) {
  const messages = await readFromStream(productId, 'new', orderStreamIds);
  if (!messages) return;

  console.log(`[${productId}] processNewOrders called, got ${messages ? messages.length : 0} messages`);

  const book = OrderBooks.get(productId);
  if (!book) return;

  for (const { message } of messages) {
    const order = new Order(message);
    book.addOrder(order);
  }

  dirtyFlags.set(productId, true);
}

async function processNewMatches(productId: string) {
  const messages = await readFromStream(productId, 'match', matchStreamIds);
  if (!messages) return;

  const streamKey = `product:${productId}:user`;
  const payload = messages.map((entry) => entry.message);

  await redisClient.publish(streamKey, JSON.stringify(payload));

  const book = OrderBooks.get(productId);
  if (!book) return;

  for (const { message } of messages) {
    const order = new Order(message);

    book.updatePriceData(order);
    book.removeOrder(order);
    book.updateBestPrices();
    publishTickerUpdate(order.product_id, book);
  }

  dirtyFlags.set(productId, true);
}

async function publishTickerUpdate(productId: string, book: OrderBook) {
  const streamKey = `product:${productId}:ticker`;
  
  const tickerData = book.getTickerData();
  const bestBidQty = book.getQty('buy', tickerData.best_bid);
  const bestAskQty = book.getQty('sell', tickerData.best_ask);

  await redisClient.publish(
    streamKey,
    JSON.stringify({
      type: 'ticker',
      product_id: productId,
      ...tickerData,
      best_bid_quantity: bestBidQty,
      best_ask_quantity: bestAskQty
    }),
  );
}

async function level2Updates(productId: string) {
  if (!dirtyFlags.get(productId)) return;

  const book = OrderBooks.get(productId);
  if (!book) return;

  
  const snapshotKey = `snapshot`;
  book.updateSnapshot();
  const currentSnapshot = book.getSnapshot();

  const previousRaw = await redisClient.hGet(snapshotKey, productId);
  const previousSnapshot = previousRaw ? JSON.parse(previousRaw) : [];

  const diffs = book.getDiffs(currentSnapshot, previousSnapshot);
  if (diffs.length === 0) return;

  console.log(`Publishing ${diffs.length} updates for ${productId}`);

  const streamKey = `product:${productId}:l2_data`;
  await redisClient.publish(
    streamKey,
    JSON.stringify({
      type: 'update',
      product_id: productId,
      updates: diffs,
    }),
  );
  await redisClient.hSet(
    snapshotKey,
    productId,
    JSON.stringify(currentSnapshot),
  );

  dirtyFlags.set(productId, false);
}

async function processAllProducts() {
  const tasks = Array.from(activeProducts).map(async (productId) => {
    if (!OrderBooks.has(productId)) {
      OrderBooks.set(productId, new OrderBook());
    }
    await processNewOrders(productId);
    await processNewMatches(productId);
    await level2Updates(productId);
  });

  await Promise.all(tasks);
}

let interval: NodeJS.Timeout;

async function main() {
  interval = setInterval(async () => {
    await processAllProducts();
  }, 100);
}

// Graceful shutdown handler
function shutdown() {
  console.log('Received SIGTERM, shutting down...');

  clearInterval(interval); // Stop the interval

  // Close Redis connections
  closeRedisConnections().then(() => {
    console.log('Redis connections closed');
    process.exit(0); // Exit process gracefully
  }).catch((err) => {
    console.error('Error while closing Redis connections', err);
    process.exit(1); // Exit with error code if Redis closing fails
  });
}

// Listen for SIGTERM (sent by process managers or manually)
process.on('SIGTERM', shutdown);

// Listen for SIGINT (Ctrl+C)
process.on('SIGINT', shutdown);

// Initialize and run the main logic
(async () => {
  await main();
})();