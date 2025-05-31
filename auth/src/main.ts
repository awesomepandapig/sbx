// import path from 'path';
import cors from 'cors';
import express, { Request, Response, NextFunction } from 'express';
import { toNodeHandler } from 'better-auth/node';
import { verifyIgn } from './routes/index';
import { auth } from './lib/auth';
import { DOMAIN, PROD } from './config/index';

const app = express();
const port = 8001;

app.use(
  cors({
    origin: PROD
      ? [`https://${DOMAIN}`, `https://api.${DOMAIN}`]
      : ['http://localhost:5173'],
    methods: ['GET', 'POST', 'DELETE', 'OPTIONS', 'PATCH'],
    credentials: true,
  }),
);

app.all('/api/auth/*', toNodeHandler(auth));

/* IMPORTANT:
Don’t use express.json() before the Better Auth handler. Use it only for other routes, or the client API will get stuck on "pending".
 */

app.use(express.json());
app.use((err: unknown, req: Request, res: Response, next: NextFunction) => {
  if (err instanceof SyntaxError) {
    res.status(400).json({ error: 'Invalid JSON' });
    return;
  }
  next(err);
});

app.use('/api/verify-ign', express.json(), verifyIgn);

app.listen(port, () => {
  console.log(`Server started on ${port}`);
});
