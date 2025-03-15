import { WebSocketServer } from 'ws';
import * as jose from 'jose';
import { randomUUID } from 'crypto';

const JWKS = jose.createRemoteJWKSet(
  new URL('http://localhost:8000/api/auth/jwks'),
);

const PORT = 8080;
const connections: Record<string, WebSocket> = {};

const wss = new WebSocketServer({ port: PORT });
console.log(`Server started on ${PORT}`);

wss.on('connection', function connection(ws) {
  const uuid = randomUUID();
  connections[uuid] = ws;

  ws.on('error', console.error);

  ws.on('message', async function message(data) {
    console.log('received: %s', data);

    try {
      // Parse the incoming message
      const message = JSON.parse(data.toString());

      if (message.type === 'authenticate' && message.token) {
        // Extract the token from the authenticate message
        const token = message.token;

        // Verify the JWT using the JWKS
        const { payload } = await jose.jwtVerify(token, JWKS, {
          issuer: 'http://localhost:8000',
        });

        console.log('JWT verified, payload:', payload);

        // Respond to the client with the authentication status
        ws.send(JSON.stringify({ type: 'authenticated', payload }));
      } else {
        ws.send(
          JSON.stringify({
            type: 'error',
            message: 'Invalid message or token missing',
          }),
        );
      }
    } catch (error) {
      console.error(error);
    }
  });

  ws.send('something');
});
