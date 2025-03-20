import 'dotenv/config';
import { createClient } from 'redis';

export const PROD = process.env.NODE_ENV === 'production';
export const DOMAIN = process.env.DOMAIN || 'localhost';

export const redisClient = createClient();
redisClient.on('error', (err) => console.log('Redis Client Error', err));
await redisClient.connect();
