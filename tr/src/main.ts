import 'dotenv/config';
import {
  createClient,
  RedisClientType,
  RedisModules,
  RedisFunctions,
  RedisScripts,
} from 'redis';
import { db } from 'db';
import { order } from 'db/schema';
import { createInsertSchema, createUpdateSchema } from 'drizzle-zod';
import { parseOrder } from 'models/order';
import { eq } from 'drizzle-orm';

const orderInsertSchema = createInsertSchema(order);
const orderUpdateSchema = createUpdateSchema(order);

const CONSUMER_GROUP_NAME = 'database-service';
const CONSUMER_NAME = 'alice'; // TODO: REPLACE WITH POD_NAME

const REDIS_BLOCK_TIMEOUT_MS = 5000;
const REDIS_READ_COUNT = 1000;

type ClientType = RedisClientType<RedisModules, RedisFunctions, RedisScripts>;

async function readFromStream(
  client: ClientType,
  productId: string,
): Promise<[string, Record<string, string>][]> {
  const streamName = `instrument:events:${productId}`;

  const results = await client.xReadGroup(
    CONSUMER_GROUP_NAME,
    CONSUMER_NAME,
    {
      key: streamName,
      id: '>',
    },
    {
      COUNT: REDIS_READ_COUNT,
      BLOCK: REDIS_BLOCK_TIMEOUT_MS,
    },
  );

  const orders: [string, Record<string, string>][] = [];

  if (results) {
    for (const stream of results) {
      for (const msg of stream.messages) {
        orders.push([msg.id, msg.message]);
      }
    }
  }

  return orders;
}

async function insertOrder(fields: Record<string, string>) {
  const data = parseOrder(fields);
  const parsed = orderInsertSchema.parse(data);
  await db.insert(order).values(parsed);
}

async function updateOrder(fields: Record<string, string>) {
  const data = parseOrder(fields);
  const parsed = orderUpdateSchema.parse(data);
  await db.update(order).set(parsed).where(eq(order.id, fields.id));
}

async function main() {
  const redisURL = process.env.REDIS_URL;
  const productId = process.env.PRODUCT_ID;

  if (!redisURL) throw new Error('Missing REDIS_URL in environment');
  if (!productId) throw new Error('Missing PRODUCT_ID in environment');

  const client = createClient({ url: redisURL });
  await client.connect();

  while (true) {
    const orders = await readFromStream(client, productId);
    if (orders.length == 0) {
      continue;
    }

    for (const [messageId, ord] of orders) {
      const streamName = `instrument:events:${productId}`;
      switch (ord.action) {
        case 'create':
          insertOrder(ord);
          break;
        case 'match':
          updateOrder(ord);
          break;
        case 'cancel':
          // TODO: Have matching engine change the status
          ord.status = 'cancelled';
          updateOrder(ord);
          break;
        case 'cancel_reject':
          break;
        default:
          break;
      }
      await client.xAck(streamName, CONSUMER_GROUP_NAME, messageId);
    }
  }
}

main().catch(console.error);
