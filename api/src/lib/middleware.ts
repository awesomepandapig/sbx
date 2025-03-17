import { NextFunction, Request, Response } from 'express';
import { fromNodeHeaders } from 'better-auth/node';
import { auth } from './auth';
import type { ValidateFunction } from 'ajv';

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

export const validate = (validateFn: ValidateFunction<unknown>) => {
  return (req: Request, res: Response, next: NextFunction): void => {
    try {
      const valid = validateFn(req.body);
      if (!valid) {
        res.status(400).json({ message: 'Invalid request' });
        return;
      }
      next();
    } catch (error) {
      console.error('Validation error:', error);
      res.status(500).json({ message: 'Internal Server Error' });
    }
  };
};
