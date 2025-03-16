import { WebSocket } from 'ws';

export interface AuthenticatedWebSocket extends WebSocket {
  authenticated?: boolean;
  user_id?: string;
}
