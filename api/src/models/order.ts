// import { UUID } from 'crypto';
// import type { Product } from './product';

// export type OrderStatus = 'received' | 'open' | 'done';
// export type OrderSide = 'buy' | 'sell';
// export type OrderType = 'market' | 'limit';

// export interface BaseOrder {
//   id: UUID;
//   product_id: Product;
//   user_id: string;
//   side: OrderSide;
//   type: OrderType;
//   created_at: string;
//   executed_value: number;
//   status: OrderStatus;
//   settled: boolean;
// }

// export interface MarketOrder extends BaseOrder {
//   type: 'market';
// }

// export interface LimitOrder extends BaseOrder {
//   type: 'limit';
//   price: number;
// }

// export type Order = MarketOrder | LimitOrder;

import * as fs from 'fs';
import * as path from 'path';
import { ajv } from '../config/index';
import { productSchema } from './product';
import { FromSchema } from 'json-schema-to-ts';

const __dirname = path.dirname(new URL(import.meta.url).pathname);

const orderSchema = JSON.parse(
  fs.readFileSync(
    path.resolve(__dirname, '../../../schemas/order.schema.json'),
    'utf8',
  ),
);

export type Order = FromSchema<typeof orderSchema>;
export const validateOrder = ajv.addSchema(productSchema).compile(orderSchema);
