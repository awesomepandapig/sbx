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
    // Cast and validate fields explicitly
    this.id = data.id;
    this.product_id = data.product_id;
    this.user_id = data.user_id;
    this.side = data.side as 'buy' | 'sell'; // Explicitly cast the string
    this.type = data.type as 'market' | 'limit'; // Explicitly cast the string
    this.created_at = parseInt(data.created_at, 10); // Convert string to number
    this.executed_value = parseFloat(data.executed_value); // Convert string to number
    this.status = data.status as 'received' | 'open' | 'done'; // Explicitly cast the string
    this.settled = data.settled === 'true'; // Convert string to boolean
    this.price = parseInt(data.price, 10); // Convert string to number
    this.cancel_after = data.cancel_after as 'min' | 'hour'; // Explicitly cast the string
    this.size = parseInt(data.size, 10); // Convert string to number
  }
}
