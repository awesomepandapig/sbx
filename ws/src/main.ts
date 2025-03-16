import { WebSocketServer } from 'ws';
import { randomUUID } from 'crypto';
import type { AuthenticatedWebSocket } from './models/index';
import { MessageSchema } from './models/index';
import { handleAuthMessage } from 'messages/index';

const connections = new Map<string, AuthenticatedWebSocket>();
const PORT = 8080;
const wss = new WebSocketServer({ port: PORT });

wss.on('connection', function connection(ws) {
  const uuid = randomUUID();
  connections.set(uuid, ws as AuthenticatedWebSocket);

  ws.on('error', console.error);

  ws.on('message', async function message(data) {
    const connection = connections.get(uuid);
    if (!connection) {
      return;
    }

    let message;
    try {
      message = MessageSchema.parse(JSON.parse(data.toString()));
    } catch (error) {
      console.log(error);
      connection.send(
        JSON.stringify({
          type: 'error',
          message: 'Invalid message format.',
        }),
      );
      return;
    }

    switch (message.type) {
      case 'authenticate':
        await handleAuthMessage(message, connection);
        break;
      case 'subscribe':
        // Handle subscribe message
        break;
      default:
        console.warn('Unknown message type:', message.type);
        connection.send(
          JSON.stringify({
            type: 'error',
            message: 'Unknown message type.',
          }),
        );
        break;
    }
  });

  ws.on('close', function close() {
    connections.delete(uuid);
  });
});

console.log(`WebSocket server started on ${PORT}`);
