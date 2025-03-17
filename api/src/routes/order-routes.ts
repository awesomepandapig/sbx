import express, { Request, Response } from 'express';
import { validateOrderRequest } from '../models/index';
import { authenticate, validate } from '../lib/index';
import { Order } from 'entities/index';

const router = express.Router();

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
  validate(validateOrderRequest),
  async (req: Request, res: Response) => {
    try {
      const user_id = res.locals.session.user.id;
      console.log(user_id);

      const order = new Order({ ...req.body, user_id });
      console.log(order);

      res.status(201).json('Order created');
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
