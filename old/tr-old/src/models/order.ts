import { UUID } from 'crypto';

export interface Order {
  id: UUID;
  product_id: string;
  user_id: string;
  side: 'buy' | 'sell';
  type: 'market' | 'limit';
  created_at: string;
  executed_value: string;
  status: 'open' | 'done' | 'cancelled';
  settled: boolean;
  price?: string;
  cancel_after?: 'min' | 'hour';
  size: string;
  action: 'create' | 'cancel';
}

export function parseOrder(fields: Record<string, string>): Order {
  return {
    id: fields.id as UUID,
    product_id: fields.product_id,
    user_id: fields.user_id,
    side: fields.side as 'buy' | 'sell',
    type: fields.type as 'market' | 'limit',
    created_at: fields.created_at,
    executed_value: fields.executed_value,
    status: fields.status as 'open' | 'done' | 'cancelled',
    settled: fields.settled === 'true',
    price: fields.price ? fields.price : undefined,
    cancel_after: fields.cancel_after as 'min' | 'hour' | undefined,
    size: fields.size,
    action: fields.action as 'create' | 'cancel',
  };
}
