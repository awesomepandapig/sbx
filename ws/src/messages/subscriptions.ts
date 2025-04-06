import {
  SubscribeMessage,
  UnsubscribeMessage,
  AuthenticatedWebSocket,
} from '../models/index';
import { channelHandlers } from '../channels/index';

export async function handleSubscription(
  message: SubscribeMessage | UnsubscribeMessage,
  ws: AuthenticatedWebSocket,
): Promise<void> {
  const { channel, product_ids = [] } = message;

  const handler = channelHandlers[channel];
  if (!handler) {
    ws.sendError('Invalid channel');
    return;
  }

  const method =
    message.type === 'subscribe' ? handler.subscribe : handler.unsubscribe;
  await method(ws, new Set(product_ids));
}
