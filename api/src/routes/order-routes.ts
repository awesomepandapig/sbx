import express, { Request, Response } from 'express';
import { authenticate } from '../lib/middleware';
import { validateProduct } from '../models/product';
import { Order, OrderSchema, OrderResponse } from 'models/order';
import { z } from 'zod';
import { redisClient } from '../config/index';

const router = express.Router();

let sequence_num = 0;

// // TODO: List fills
// router.get('/fills', authenticate, async (req: Request, res: Response) => {
//   try {
//   } catch (error) {}
// });

// List orders
router.get('/', authenticate, async (req: Request, res: Response) => {
  try {
    const userId = res.locals.session.user.id;

    // TODO: PAGINATE RESPONSE

    // Get all user's order ids
    const orderIds = await redisClient.sMembers(`user_orders:${userId}`);

    // // Lookup all orders in hashmap
    const orders = await Promise.all(
      orderIds.map(async (orderId) => {
        return await redisClient.hGetAll(`orders:${orderId}`);
      }),
    );

    // TODO: DON'T RETURN EXPIRED OR CANCELLED ORDERS

    // const orders = await redisClient.hGetAll(`order:${order.id}`);
    res.status(200).json({ orders });
  } catch (error) {
    // TODO: Handle
    console.log(error);
  }
});

// // TODO: Cancel all orders
// router.delete('/orders', authenticate, async (req: Request, res: Response) => {
//   try {
//   } catch (error) {}
// });

// Create a new order
router.post('/', authenticate, async (req: Request, res: Response) => {
  try {
    // Validate user input

    const data = OrderSchema.parse(req.body);

    // TODO: ADD HYPIXEL PURSE CHECK TO ENSURE USER HAS SUFFICIENT FUNDS FOR BID

    const userId = res.locals.session.user.id;

    const validProduct = await validateProduct(data.product_id);
    if (!validProduct) {
      res.status(400).json({ message: 'Invalid product ID' });
      return;
    }

    const order = new Order({ ...data, user_id: userId });
    const orderStringified = order.toRedisTuples();

    // Add the message to the message stream
    const streamId = `${Date.now()}-${sequence_num}`;
    await redisClient.xAdd(
      `${order.product_id}:new`,
      streamId,
      orderStringified,
    );
    sequence_num++;

    // Cache the message in Redis for O(1) metadata lookups
    await redisClient.hSet(`orders:${order.id}`, orderStringified);

    // Cache the message id for O(1) Per-User Order Retrieval
    await redisClient.sAdd(`user_orders:${order.user_id}`, order.id);
    
    // Return the response
    const orderResponse: OrderResponse = {
        ...order,
        created_at: new Date(order.created_at * 1000).toISOString(), // Convert timestamp to ISO
    };
  
    res.status(201).json(orderResponse);
  } catch (error) {
    // TODO: Handle
    console.log(error);
    res.status(400).json({ message: 'bad request' });
  }
});

// Get a single order
router.get('/:id', authenticate, async (req: Request, res: Response) => {
  try {
    const orderId = z.string().uuid().parse(req.params.id);
    const userId = res.locals.session.user.id;

    const orderKey = `orders:${orderId}`;
    let order = await redisClient.hGetAll(orderKey);

    if (!order || Object.keys(order).length === 0) {
      res.status(404).json({ message: 'Order not found or expired' });
      return;
    }

    if (order.user_id !== userId) {
      res.status(403).json({ message: 'Unauthorized' });
      return;
    }

    // Convert UNIX timestamp to human readable format
    order.created_at = new Date(order.created_at).toISOString();
    res.status(200).json({ order });
  } catch (error) {
    // TODO: if error is zod error 400
    // TODO: else 500
    console.log(error);
  }
});

// // Cancel an order
// router.delete(
//   '/orders/:order_id',
//   authenticate,
//   async (req: Request, res: Response) => {
//     try {
// if (order.status !== 'open') {
//   res
//   .status(400)
//   .json({ message: 'Order already filled or was previously canceled.' });
// return;
// }
// TODO: Send cancel request to matching engine (via Redis Stream)
//     } catch (error) {}
//   },
// );

export default router;
