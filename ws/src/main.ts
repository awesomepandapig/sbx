import { WebSocketServer, WebSocket } from 'ws';
import { randomUUID } from 'crypto';
import { authenticate } from './lib/middleware';

interface AuthenticatedWebSocket extends WebSocket {
  authenticated?: boolean;
}

const connections = new Map<string, AuthenticatedWebSocket>();
const PORT = 8080;
const wss = new WebSocketServer({ port: PORT });

const handleMessage = async (ws: WebSocket, bytes: Buffer, uuid: string) => {
  const message = JSON.parse(bytes.toString());
  const connection = connections.get(uuid);

  if (!connection) {
    console.log('Connection not found');
    return;
  }

  if (message.type === 'authenticate') {
    const isAuthenticated = await authenticate(message.token);
    connection.authenticated = isAuthenticated;
    return;
  }

  if (connection.authenticated) {
    //  TODO: (add message handlers)
    ws.send("authenticated!");
  } else {
    // close connection
  }
};

wss.on('connection', function connection(ws) {
  const uuid = randomUUID();
  connections.set(uuid, ws as AuthenticatedWebSocket);

  ws.on('error', console.error);

  ws.on('message', async function message(data: Buffer) {
    await handleMessage(ws, data, uuid);
  });

  ws.on('close', function close() {
    connections.delete(uuid);
  });
});

console.log(`WebSocket server started on ${PORT}`);
