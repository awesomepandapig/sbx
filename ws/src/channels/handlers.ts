import { activeProducts, waitForRedis } from '../config/index';
import { UserChannel, Level2Channel, TickerChannel } from '../models/index';
import { handleAuth } from 'messages';

import type {
  AuthenticatedWebSocket,
  SubscribeMessage,
  UnsubscribeMessage,
} from '../models/index';

interface ChannelHandler {
  subscribe: (ws: AuthenticatedWebSocket, message: SubscribeMessage) => void;
  unsubscribe: (
    ws: AuthenticatedWebSocket,
    message: UnsubscribeMessage,
  ) => void;
  cleanup: (ws: AuthenticatedWebSocket) => void;
}

async function initLevel2() {
  await waitForRedis();
  const channels = Array.from(activeProducts).map(
    (id) => `marketdata:level2:${id}`,
  );
  return Level2Channel.initialize(channels);
}

async function initUser() {
  await waitForRedis();
  const channels = Array.from(activeProducts).map((id) => `product:${id}:user`);
  return UserChannel.initialize(channels);
}

async function initTicker() {
  await waitForRedis();
  const channels = Array.from(activeProducts).map(
    (id) => `marketdata:ticker:${id}`,
  );
  return TickerChannel.initialize(channels);
}

const level2 = await initLevel2();
const user = await initUser();
const ticker = await initTicker();

export const channelHandlers: Record<string, ChannelHandler> = {
  level2: {
    subscribe: (...args) => level2.subscribe(...args),
    unsubscribe: (...args) => level2.unsubscribe(...args),
    cleanup: (...args) => level2.cleanup(...args),
  },
  ticker: {
    subscribe: (...args) => ticker.subscribe(...args),
    unsubscribe: (...args) => ticker.unsubscribe(...args),
    cleanup: (...args) => ticker.cleanup(...args),
  },
  user: {
    subscribe: async (ws, msg) => {
      await handleAuth(ws, msg);
      if (!ws.user_id) return;
      return user.subscribe(ws, msg);
    },
    unsubscribe: (...args) => user.unsubscribe(...args),
    cleanup: (...args) => user.cleanup(...args),
  },
};
