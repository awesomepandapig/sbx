import { randomUUID, UUID } from 'crypto';
import { Product } from '../models/index';

export class Order {
  id: UUID;
  product_id: Product;
  user_id: string;
  side: 'buy' | 'sell';
  type: 'market' | 'limit';
  created_at: string;
  executed_value: number;
  status: 'received' | 'open' | 'done';
  settled: boolean;
  price?: number;

  constructor(data: {
    product_id: string;
    user_id: string;
    side: 'buy' | 'sell';
    type: 'market' | 'limit';
    price?: number;
  }) {
    this.id = randomUUID();
    this.product_id = data.product_id;
    this.user_id = data.user_id;
    this.side = data.side;
    this.type = data.type;
    this.created_at = new Date().toISOString();
    this.executed_value = 0;
    this.status = 'received';
    this.settled = false;

    if (this.type === 'limit') {
      this.price = data.price;
    }
  }
}
