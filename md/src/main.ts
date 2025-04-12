import { activeProducts, redisClient } from './config/index';
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

  console.log(`[${productId}] processNewMatches called, got ${messages ? messages.length : 0} messages`);

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
    book.removeOrder(order);
  }

  dirtyFlags.set(productId, true);
}

async function level2updates() {

}

async function broadcastUpdates(productId: string) {
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
    await broadcastUpdates(productId);
  });

  await Promise.all(tasks);
}

async function main() {
  setInterval(async () => {
    await processAllProducts();
  }, 100);
}

(async () => {
  await main();
})();



/*

interface Event {
    type: 'snapshot';
    tickers: Ticker[];
}

interface Ticker {
    type: 'ticker',
    product_id: string,
    price: string
    volume_24_h: string,
    low_24_h: string,
    high_24_h: string,
    low_52_w: string,
    high_52_w: string,
    price_percent_chg_24_h: string,
    best_bid: string,
    best_bid_quantity: string,
    best_ask: string,
    best_ask_quantity: string
}
  */