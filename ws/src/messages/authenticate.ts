import { authenticate } from '../utils/index';
import { SubscribeMessage, AuthenticatedWebSocket } from '../models/index';

export const handleAuth = async (
  ws: AuthenticatedWebSocket,
  message: SubscribeMessage,
): Promise<void> => {
  try {
    if (!message.jwt) {
      ws.sendError('JWT missing. Closing connection.');
      ws.close();
      return;
    }
    const authResult = await authenticate(message.jwt);
    if (authResult.authenticated) {
      ws.authenticated = true;
      ws.user_id = authResult.user_id;
    } else {
      ws.sendError('Authentication failed. Closing connection.');
      ws.close();
    }
  } catch (error) {
    console.error('Error during authentication:', error);
    ws.sendError('Authentication failed. Closing connection.');
    ws.close();
  }
};
