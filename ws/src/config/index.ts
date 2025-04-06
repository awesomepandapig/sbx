import 'dotenv/config';
import { createClient } from 'redis';

export const PROD = process.env.NODE_ENV === 'production';
export const DOMAIN = process.env.DOMAIN || 'localhost';

// Create Redis client with options
const createRedisClient = () => {
  const client = createClient({
    url: process.env.REDIS_URL,
    socket: {
      reconnectStrategy: (retries) => {
        // Exponential backoff with maximum delay
        const delay = Math.min(1000 * Math.pow(2, retries), 30000);
        return delay;
      },
    },
  });

  return client;
};

// Main client
export const redisClient = createRedisClient();
redisClient.on('error', (err) => console.error('Redis Client Error', err));
redisClient.on('reconnecting', () => console.log('Redis client reconnecting'));
redisClient.on('connect', () => console.log('Redis client connected'));

// Subscriber client
export const redisSubscriber = createRedisClient();
redisSubscriber.on('error', (err) =>
  console.error('Redis Subscriber Error', err),
);
redisSubscriber.on('reconnecting', () =>
  console.log('Redis subscriber reconnecting'),
);
redisSubscriber.on('connect', () => console.log('Redis subscriber connected'));

let redisInitialized = false;

// Initialize connection
export const initRedis = async () => {
  if (redisInitialized) return;

  try {
    await redisClient.connect();
    await redisSubscriber.connect();

    // Load initial active products from Redis
    const initialProducts = await redisClient.sMembers('products:active');
    initialProducts.forEach((product) => activeProducts.add(product));
    console.log(`Loaded ${initialProducts.length} active products from Redis`);

    // Subscribe to product updates
    await redisSubscriber.subscribe('products:new', (message) => {
      // Update the local product list
      activeProducts.add(message);
      console.log(`Product added: ${message}`);
    });

    console.log('Redis connections established');
    redisInitialized = true;
  } catch (error) {
    console.error('Failed to initialize Redis connections:', error);
    process.exit(1);
  }
};

// Handle graceful shutdown
export const closeRedisConnections = async () => {
  try {
    await redisClient.quit();
    await redisSubscriber.quit();
    console.log('Redis connections closed');
  } catch (error) {
    console.error('Error closing Redis connections:', error);
  }
};

export const waitForRedis = async () => {
  while (!redisInitialized) {
    // Keep checking if Redis is initialized (with a slight delay to avoid tight looping)
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
};

initRedis();
export const activeProducts = new Set<string>();
