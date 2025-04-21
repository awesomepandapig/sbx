import { activeProducts, waitForRedis } from '../config/index';
import { UserChannel, Level2Channel, TickerChannel, TickerBatchChannel, CandlesChannel } from '../models/index';
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

async function initTickerBatch() {
  await waitForRedis();
  const channels = Array.from(activeProducts).map(
    (id) => `marketdata:ticker_batch:${id}`,
  );
  return TickerBatchChannel.initialize(channels);
}

async function initCandles() {
  await waitForRedis();
  const channels = Array.from(activeProducts).map(
    (id) => `marketdata:candles:${id}`,
  );
  return CandlesChannel.initialize(channels);
}

const level2 = await initLevel2();
const user = await initUser();
const ticker = await initTicker();
const ticker_batch = await initTickerBatch();
const candles = await initCandles();

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
  ticker_batch: {
    subscribe: (...args) => ticker_batch.subscribe(...args),
    unsubscribe: (...args) => ticker_batch.unsubscribe(...args),
    cleanup: (...args) => ticker_batch.cleanup(...args),
  },
  candles: {
    subscribe: (...args) => candles.subscribe(...args),
    unsubscribe: (...args) => candles.unsubscribe(...args),
    cleanup: (...args) => candles.cleanup(...args),
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
