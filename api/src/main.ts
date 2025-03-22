import path from 'path';
import cors from 'cors';
import express, { Request, Response, NextFunction } from 'express';
import { toNodeHandler } from 'better-auth/node';
// import { apiReference } from "@scalar/express-api-reference";
import { verifyIgn, orderRoutes } from './routes/index';
import { auth } from './lib/auth';
import { DOMAIN, PROD } from './config/index';

const app = express();
const port = 8000;

app.use(
  cors({
    origin: PROD
      ? [`https://${DOMAIN}`, `https://api.${DOMAIN}`]
      : ['http://localhost:3000'],
    methods: ['GET', 'POST', 'DELETE', 'OPTIONS', 'PATCH'],
    credentials: true,
  }),
);

app.get('/api/openapi', (_req, res) => {
  const filePath = path.resolve(__dirname, '../docs/openapi.yaml');
  res.sendFile(filePath);
});

// app.get(
//   "/",
//   apiReference({
//     theme: "saturn",
//     metaData: {
//       title: "Skyblock.Exchange API",
//     },
//     spec: {
//       url: PROD ? "/openapi" : "/api/openapi",
//     },
//   })
// );

app.use(express.json());

app.use((err: unknown, req: Request, res: Response, next: NextFunction) => {
  if (err instanceof SyntaxError) {
    res.status(400).json({ error: 'Invalid JSON' });
    return;
  }
  next(err);
});

app.use('/api/auth/verify-ign', express.json(), verifyIgn);
app.all('/api/auth/*', toNodeHandler(auth));
app.use('/api/orders', express.json(), orderRoutes);

app.listen(port, () => {
  console.log(`Server started on ${port}`);
});
