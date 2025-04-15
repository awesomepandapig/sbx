import 'dotenv/config';
import { createClient } from 'redis';

export const PROD = process.env.NODE_ENV === 'production';
export const REDIS_URL = process.env.REDIS_URL || 'redis://localhost:6379';

export const activeProducts = new Set<string>();
export const redisClient = createClient({ url: REDIS_URL, name: "market-data-client" });
export const redisSubscriber = createClient({ url: REDIS_URL, name: "market-data-subscriber" });

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
    if (redisClient.isOpen) {
      try {
        await redisClient.quit();
        console.log('Redis client disconnected');
      } catch (err) {
        console.error('Error shutting down Redis client:', err);
      }
    }
  
    if (redisSubscriber.isOpen) {
      try {
        await redisSubscriber.quit();
        console.log('Redis subscriber disconnected');
      } catch (err) {
        console.error('Error shutting down Redis subscriber:', err);
      }
    }
    process.exit(0);
  } catch (error) {
    console.error('Error closing Redis connections:', error);
  }
}

process.on('SIGINT', closeRedisConnections);
process.on('SIGTERM', closeRedisConnections);


initRedis();