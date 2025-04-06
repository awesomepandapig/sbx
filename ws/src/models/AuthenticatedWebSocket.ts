import { WebSocket } from 'ws';
import { randomUUID } from 'crypto';

export class AuthenticatedWebSocket {
  // Authentication properties
  public authenticated = false;
  public user_id?: string;

  // Message tracking
  public readonly client_id: string;
  private sequence_num = 0;

  constructor(private ws: WebSocket) {
    this.client_id = randomUUID();
    this.sequence_num = 0;
  }

  public sendMessage(channel: string, events: object[]): void {
    const message = {
      channel,
      client_id: this.client_id,
      timestamp: new Date().toISOString(),
      sequence_num: this.sequence_num++,
      events,
    };
    this.ws.send(JSON.stringify(message));
  }

  public sendError(message: string): void {
    const error = {
      type: 'error',
      message,
    };
    this.ws.send(JSON.stringify(error));
  }

  // Forward the original WebSocket methods
  public on(
    event: string | symbol,
    listener: (...args: Parameters<WebSocket['on']>) => void,
  ) {
    this.ws.on(event, listener);
  }

  public close() {
    this.ws.close();
  }
}
