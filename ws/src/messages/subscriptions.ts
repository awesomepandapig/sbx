import {
  SubscribeMessage,
  UnsubscribeMessage,
  AuthenticatedWebSocket,
} from '../models/index';
import { channelHandlers } from '../channels/index';

export async function handleSubscribe(
  message: SubscribeMessage,
  ws: AuthenticatedWebSocket,
): Promise<void> {
  const { channel } = message;

  const handler = channelHandlers[channel];
  if (!handler) {
    ws.sendError('Invalid channel');
    return;
  }

  await handler.subscribe(ws, message);
}

export async function handleUnsubscribe(
  message: UnsubscribeMessage,
  ws: AuthenticatedWebSocket,
): Promise<void> {
  const { channel } = message;

  const handler = channelHandlers[channel];
  if (!handler) {
    ws.sendError('Invalid channel');
    return;
  }

  await handler.unsubscribe(ws, message);
}
