import 'dotenv/config';
import Ajv from 'ajv';
import addFormats from 'ajv-formats';

export const PROD = process.env.NODE_ENV === 'production';
export const DOMAIN = process.env.DOMAIN || 'localhost';

const ajv = new Ajv();
addFormats(ajv);

export { ajv };
