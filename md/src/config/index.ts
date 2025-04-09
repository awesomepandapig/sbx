import 'dotenv/config';
import { createClient } from 'redis';

export const PROD = process.env.NODE_ENV === 'production';
export const REDIS_URL = process.env.REDIS_URL || 'redis://localhost:6379';

export const activeProducts = new Set<string>();
export const redisClient = createClient({ url: REDIS_URL });
export const redisSubscriber = createClient({ url: REDIS_URL });

let redisInitialized = false;
async function initRedis() {
  if (redisInitialized) return;

  try {
    await redisClient.connect();
    await redisSubscriber.connect();

    const initialProducts = await redisClient.sMembers('product');
    initialProducts.forEach((product) => activeProducts.add(product));
    console.log(`Loaded ${initialProducts.length} active products from Redis`);

    await redisSubscriber.subscribe('products:new', (message) => {
      if (!activeProducts.has(message)) {
        activeProducts.add(message);
        console.log(`Discovered new product: ${message}`);
      }
    });

    console.log('Redis clients connected');
    redisInitialized = true;
  } catch (error) {
    console.error('Failed to initialize Redis:', error);
    process.exit(1);
  }
}

export async function waitForRedis() {
  while (!redisInitialized) {
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
}

export async function closeRedisConnections() {
  try {
    await redisClient.quit();
    await redisSubscriber.quit();
    console.log('Redis connections closed');
  } catch (error) {
    console.error('Error closing Redis connections:', error);
  }
}

initRedis();
