import { NextFunction, Request, Response } from 'express';
import { fromNodeHeaders } from 'better-auth/node';
import { auth } from './auth';

const ERR_BAD_REQUEST = 'Bad Request'; // 400
const ERR_UNAUTHORIZED = 'Unauthorized'; // 401
const ERR_FORBIDDEN = 'Forbidden'; // 403
const ERR_INTERNAL_SERVER = 'Internal Server Error'; // 500

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
      res.status(401).json({ message: ERR_UNAUTHORIZED });
      return;
    }

    // Attach the session to the request
    res.locals.session = session;
    next();
  } catch (error) {
    if(error.statusCode && error.statusCode == '401') {
      res.status(401).json({ message: ERR_UNAUTHORIZED });
      return;
    } else {
      console.error(error);
      res.status(500).json({ message: ERR_INTERNAL_SERVER });
    }
  }
};
