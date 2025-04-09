export class Order {
  public id: string;
  public product_id: string;
  public user_id: string;
  public side: 'buy' | 'sell';
  public type: 'market' | 'limit';
  public created_at: number;
  public executed_value: number;
  public status: 'received' | 'open' | 'done';
  public settled: boolean;
  public price: number;
  public cancel_after: 'min' | 'hour';
  public size: number;

  constructor(data: Record<string, string>) {
    this.id = data.id;
    this.product_id = data.product_id;
    this.user_id = data.user_id;
    this.side = data.side as 'buy' | 'sell';
    this.type = data.type as 'market' | 'limit';
    this.created_at = parseInt(data.created_at, 10);
    this.executed_value = parseFloat(data.executed_value);
    this.status = data.status as 'received' | 'open' | 'done';
    this.settled = data.settled === 'true';
    this.price = parseInt(data.price, 10);
    this.cancel_after = data.cancel_after as 'min' | 'hour';
    this.size = parseInt(data.size, 10);
  }
}
