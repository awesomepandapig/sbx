import { activeProducts, redisClient } from './config/index';
import { OrderBook, Order } from './models/index';

const OrderBooks = new Map<string, OrderBook>();
const orderStreamIds = new Map<string, string>();
const matchStreamIds = new Map<string, string>();

async function readFromStream(
  productId: string,
  suffix: string,
  streamIdMap: Map<string, string>,
) {
  const streamKey = `${productId}:${suffix}`;
  const lastSeenId = streamIdMap.get(productId) ?? '0-0';

  const result = await redisClient.xRead(
    [{ key: streamKey, id: lastSeenId }],
    { COUNT: 1000, BLOCK: 1 }
  );
  if (!result?.length) {
    console.log(`(${streamKey}) Waiting for new messages`);
    return null;
  }

  const stream = result[0];
  const lastId = stream.messages.at(-1)?.id;
  if (lastId) {
    console.log(`Updating stream ID for ${productId}: ${lastId}`);
    streamIdMap.set(productId, lastId);
  }

  return stream.messages;
}

async function processNewOrders(productId: string) {
  const messages = await readFromStream(productId, 'new', orderStreamIds);
  if (!messages) return;
  const book = OrderBooks.get(productId);
  if (!book) return;

  for (const { message } of messages) {
    const order = new Order(message);
    book.addOrder(order);
  }
}

async function processNewMatches(productId: string) {
  const messages = await readFromStream(productId, 'matches', matchStreamIds);

  if (!messages) return;
  const book = OrderBooks.get(productId);
  if (!book) return;

  for (const { message } of messages) {
    const order = new Order(message);
    book.removeOrder(order);
  }
}

async function broadcastUpdates(productId: string) {
  const book = OrderBooks.get(productId);
  if (!book) return;

  const snapshotKey = `snapshot`;
  const currentSnapshot = book.getSnapshot();
  const previousRaw = await redisClient.hGet(snapshotKey, productId);
  const previousSnapshot = previousRaw ? JSON.parse(previousRaw) : [];

  const diffs = book.getDiffs(currentSnapshot, previousSnapshot);
  if (diffs.length === 0) return;

  // console.log(diffs); // TODO:

  await redisClient.publish(
    `${productId}:updates`,
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
}

async function processAllProducts() {
  for (const productId of activeProducts) {
    // Register new products while running
    if (!OrderBooks.has(productId)) {
      OrderBooks.set(productId, new OrderBook());
    }
    await processNewOrders(productId);
    await processNewMatches(productId);
    await broadcastUpdates(productId);
  }
}

async function main() {
  while (true) {
    try {
      await processAllProducts();
    } catch (error) {
      console.error('Error in loop:', error);
    }
    await new Promise((r) => setTimeout(r, 100));
  }
}

(async () => {
  await main();
})();
