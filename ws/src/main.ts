import { WebSocketServer } from 'ws';
import { MessageSchema, AuthenticatedWebSocket } from './models/index';
import { handleAuth, handleSubscription } from './messages/index';
import { channelHandlers } from './channels/index';

const PORT = 8080;
const wss = new WebSocketServer({ port: PORT });

wss.on('connection', function connection(rawWs) {
  const ws = new AuthenticatedWebSocket(rawWs);
  ws.on('error', console.error); // TODO: replace with winston or pino

  // Disconnect if no subscribe message has been received within 5 seconds.
  const timeout = setTimeout(() => {
    ws.sendError('No subscribe message received within 5 seconds');
    ws.close();
  }, 5000);

  ws.on('message', async function message(data) {
    let message;
    try {
      message = MessageSchema.parse(JSON.parse(data.toString()));
    } catch (error: unknown) {
      if (error instanceof Error) {
        ws.sendError('Invalid message format.');
      }
      return;
    }

    switch (message.type) {
      case 'authenticate':
        await handleAuth(message, ws);
        break;
      case 'subscribe':
        clearTimeout(timeout);
        await handleSubscription(message, ws);
        break;
      case 'unsubscribe':
        await handleSubscription(message, ws);
        break;
      default:
        ws.sendError('Unknown message type.');
        break;
    }
  });

  ws.on('close', function close() {
    // Unsubscribe from all channels
    for (const channel in channelHandlers) {
      const handler = channelHandlers[channel];
      handler.cleanup(ws);
    }
  });
});

// Handle server shutdown gracefully
process.on('SIGTERM', () => {
  console.log('SIGTERM signal received: closing WebSocket server');
  wss.close(() => {
    console.log('WebSocket server closed');
    process.exit(0);
  });
});

console.log(`WebSocket server started on ${PORT}`);
