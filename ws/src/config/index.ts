import 'dotenv/config';

export const PROD = process.env.NODE_ENV === 'production';
export const DOMAIN = process.env.DOMAIN || 'localhost';
