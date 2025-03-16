import express, { NextFunction, Request, Response } from 'express';
import { validateOrder } from '../models/index';
import { authenticate } from '../lib/middleware';

const router = express.Router();

const validateBody = async (
  req: Request,
  res: Response,
  next: NextFunction,
): Promise<void> => {
  const valid = validateOrder(req.body);
  if (!valid) {
    res.status(400).json({
      message: 'Invalid data',
      errors: validateOrder.errors,
    });
    return;
  }
  next();
};

// // List fills
// router.get('/fills', authenticate, async (req: Request, res: Response) => {
//   try {
//   } catch (error) {}
// });

// // List orders
// router.get('/orders', authenticate, async (req: Request, res: Response) => {
//   try {
//   } catch (error) {}
// });

// // Cancel all orders
// router.delete('/orders', authenticate, async (req: Request, res: Response) => {
//   try {
//   } catch (error) {}
// });

// Create a new order
router.post(
  '/',
  authenticate,
  validateBody,
  async (req: Request, res: Response) => {
    try {
      res.status(201).send('Order created');
    } catch (error) {
      console.log(error);
    }
  },
);

// // Get a single order
// router.get(
//   '/orders/:order_id',
//   authenticate,
//   async (req: Request, res: Response) => {
//     try {
//     } catch (error) {}
//   },
// );

// // Cancel an order
// router.delete(
//   '/orders/:order_id',
//   authenticate,
//   async (req: Request, res: Response) => {
//     try {
//     } catch (error) {}
//   },
// );

export default router;
