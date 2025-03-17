import * as fs from 'fs';
import * as path from 'path';
import { ajv } from '../config/index';
import { productSchema } from './product';

const __dirname = path.dirname(new URL(import.meta.url).pathname);

ajv.addSchema(productSchema);

const orderSchema = JSON.parse(
  fs.readFileSync(
    path.resolve(__dirname, '../../../schemas/order.schema.json'),
    'utf8',
  ),
);

const orderRequestSchema = JSON.parse(
  fs.readFileSync(
    path.resolve(__dirname, '../../../schemas/order-request.schema.json'),
    'utf8',
  ),
);

export const validateOrder = ajv.compile(orderSchema);
export const validateOrderRequest = ajv.compile(orderRequestSchema);
