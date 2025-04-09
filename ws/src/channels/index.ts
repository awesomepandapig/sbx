import * as level2 from './level2';
import * as user from './user';
// import * as ticker from './ticker';

import type { AuthenticatedWebSocket, SubscribeMessage, UnsubscribeMessage } from '../models/index';

interface ChannelHandler {
  subscribe: (ws: AuthenticatedWebSocket, message: SubscribeMessage) => void;
  unsubscribe: (ws: AuthenticatedWebSocket, message: UnsubscribeMessage) => void;
  cleanup: (ws: AuthenticatedWebSocket) => void;
}

export const channelHandlers: Record<string, ChannelHandler> = {
  level2,
  user,
  // ticker,
};
