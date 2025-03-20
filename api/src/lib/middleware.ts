import { NextFunction, Request, Response } from 'express';
import { fromNodeHeaders } from 'better-auth/node';
import { auth } from './auth';

export const authenticate = async (
  req: Request,
  res: Response,
  next: NextFunction,
): Promise<void> => {
  try {
    const session = await auth.api.getSession({
      headers: fromNodeHeaders(req.headers),
    });

    if (!session) {
      res.status(401).json({ error: 'Unauthorized' }); // TODO: EDIT ERROR MESSAGE
      return;
    }

    // Attach the session to the request
    res.locals.session = session;
    next();
  } catch (error) {
    console.error('Authentication error:', error); // TODO: EDIT ERROR MESSAGE
    res.status(500).json({ error: 'Internal Server Error' }); // TODO: EDIT ERROR MESSAGE
    return;
  }
};
