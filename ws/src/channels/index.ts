import * as level2 from './level2';
// import * as ticker from './ticker';
// import * as user from './user';

import type { AuthenticatedWebSocket } from '../models/index';

interface ChannelHandler {
  subscribe: (ws: AuthenticatedWebSocket, products: Set<string>) => void;
  unsubscribe: (ws: AuthenticatedWebSocket, products: Set<string>) => void;
  cleanup: (ws: AuthenticatedWebSocket) => void;
}

export const channelHandlers: Record<string, ChannelHandler> = {
  level2,
  // ticker,
  // user,
};
