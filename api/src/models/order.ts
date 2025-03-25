import { randomUUID, UUID } from 'crypto';
import { z } from 'zod';

export const OrderSchema = z
  .object({
    product_id: z.string(),
    side: z.enum(['buy', 'sell']),
    type: z.enum(['market', 'limit']),
    price: z.number().int().gt(0).optional(),
    size: z.number().int().gt(0),
  })
  .refine(
    (data) => {
      if (data.type === 'limit' && data.price === undefined) {
        return false;
      }
      if (data.type === 'market' && data.price !== undefined) {
        return false;
      }
      return true;
    },
    {
      message:
        'Price must be defined for limit orders and omitted for market orders',
      path: ['price'],
    },
  );

export class Order {
  id: UUID;
  product_id: string;
  user_id: string;
  side: 'buy' | 'sell';
  type: 'market' | 'limit';
  created_at: number;
  executed_value: number;
  status: 'received' | 'open' | 'done';
  settled: boolean;
  price?: number;
  cancel_after?: string;
  size: number;

  constructor(data: {
    product_id: string;
    user_id: string;
    side: 'buy' | 'sell';
    type: 'market' | 'limit';
    price?: number;
    size: number;
  }) {
    this.id = randomUUID();
    this.product_id = data.product_id;
    this.user_id = data.user_id;
    this.side = data.side;
    this.type = data.type;
    this.created_at = Math.floor(Date.now() / 1000);
    this.executed_value = 0;
    this.status = 'received';
    this.settled = false;
    this.size = data.size;
    this.price = data.price;

    if (this.type === 'limit') {
      this.cancel_after = 'min'; // Set to min for mineshaft markets
    }
  }

  toRedisTuples() {
    const json = {
      id: this.id.toString(),
      product_id: this.product_id,
      user_id: this.user_id,
      side: this.side.toString(),
      type: this.type.toString(),
      created_at: this.created_at.toString(),
      executed_value: this.executed_value.toString(),
      status: this.status.toString(),
      settled: this.settled.toString(),
      size: this.size.toString(),
    } as Record<string, string>;

    if (this.price !== undefined) {
      json.price = this.price.toString();
    }

    if (this.cancel_after !== undefined) {
      json.cancel_after = this.cancel_after;
    }

    return json;
  }
}

export type OrderResponse = {
  id: UUID;
  product_id: string;
  side: 'buy' | 'sell';
  type: 'market' | 'limit';
  created_at: string;
  executed_value: number;
  status: 'received' | 'open' | 'done';
  settled: boolean;
  price?: number;
  cancel_after?: string;
  size: number;
};