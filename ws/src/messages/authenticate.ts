import { authenticate } from '../lib/index';
import { AuthenticateMessage, AuthenticatedWebSocket } from '../models/index';

export const handleAuthMessage = async (
  msg: AuthenticateMessage,
  connection: AuthenticatedWebSocket,
): Promise<void> => {
  try {
    const authResult = await authenticate(msg.token);
    if (authResult.authenticated) {
      connection.authenticated = true;
      connection.user_id = authResult.user_id;
      connection.send(
        JSON.stringify({
          type: 'authenticated',
          message: 'Authentication successful!',
        }),
      );
    } else {
      connection.send(
        JSON.stringify({
          type: 'error',
          message: 'Authentication failed. Closing connection.',
        }),
      );
      connection.close(1008, 'Authentication failed');
    }
  } catch (error) {
    console.error('Error during authentication:', error);
    connection.send(
      JSON.stringify({
        type: 'error',
        message: 'An error occurred during authentication.',
      }),
    );
    connection.close(1011, 'Internal error');
  }
};
