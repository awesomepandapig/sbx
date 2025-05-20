import express, { Request, Response } from 'express';
import { authenticate } from '../lib/middleware';
import { validateProduct } from '../models/product';
import { Order, OrderSchema, OrderResponse } from 'models/order';
import { activeProducts, redisClient } from '../config';
import { createSelectSchema } from 'drizzle-zod';
import { order } from 'db/schema';
import { db } from 'db';
import { eq } from 'drizzle-orm';
import { z } from 'zod';

const router = express.Router();
const orderSelectSchema = createSelectSchema(order);

const ERR_INVALID_PRODUCT_ID = 'Invalid product ID';
const ERR_MAX_BIDS = 'Maximum number of active bids reached';
const ERR_MAX_ASKS = 'Maximum number of active asks reached';
const ERR_ORDER_NOT_FOUND = 'Order not found or expired';
const ERR_BAD_REQUEST = 'Bad Request'; // 400
const ERR_UNAUTHORIZED = 'Unauthorized'; // 401
const ERR_FORBIDDEN = 'Forbidden'; // 403
const ERR_INTERNAL_SERVER = 'Internal Server Error'; // 500

// // TODO: List fills
// router.get('/fills', authenticate, async (req: Request, res: Response) => {
//   try {
//   } catch (error) {}
// });

// List orders
router.get('/', authenticate, async (req: Request, res: Response) => {
  // try {
  //   const userId = res.locals.session.user.id;

  //   // TODO: PAGINATE RESPONSE

  //   // Get all user's orders
  //   const orderIds = [];
  //   for (const productId of activeProducts) {
  //     const orderIdList = await redisClient.sMembers(
  //       `user:${userId}:order:${productId}`,
  //     );
  //     orderIds.push(...orderIdList);
  //   }

  //   // Lookup all orders in hashmap
  //   const orders = await Promise.all(
  //     orderIds.map(async (orderId) => {
  //       return await redisClient.hGetAll(`order:${orderId}`);
  //     }),
  //   );

  //   // Make timestamp
  //   for (const order of orders) {
  //     order.created_at = new Date(parseInt(order.created_at)).toISOString();
  //   }

  //   // TODO: DON'T RETURN EXPIRED OR CANCELLED ORDERS

  //   // res.status(200).json({ orders });
  // } catch (error) {
  //   // TODO: Handle
  //   console.log(error);
  // }
  res.status(200).json({ orders: [] });
});

// // TODO: Cancel all orders
// router.delete('/orders', authenticate, async (req: Request, res: Response) => {
//   try {
//   } catch (error) {}
// });

// Create a new order
router.post('/', authenticate, async (req: Request, res: Response) => {
  try {
    const orderData = OrderSchema.parse(req.body);
    const userId = res.locals.session.user.id;

    const isValidProduct = await validateProduct(orderData.product_id);
    if (!isValidProduct) {
      res.status(400).json({ message: ERR_INVALID_PRODUCT_ID });
      return;
    }

    // TODO: ADD HYPIXEL PURSE CHECK TO ENSURE USER HAS SUFFICIENT FUNDS FOR BID

    // TODO: Use Redis to check the active orders count for this user in this product
    // const activeBidsKey = `user:${userId}:product:${data.product_id}:active_bids`;
    // const activeAsksKey = `user:${userId}:product:${data.product_id}:active_asks`;

    // let activeBids = Number(await redisClient.get(activeBidsKey));
    // let activeAsks = Number(await redisClient.get(activeAsksKey));

    // // Default to 0 if the key doesn't exist
    // activeBids = activeBids ? activeBids : 0;
    // activeAsks = activeAsks ? activeAsks : 0;

    // // Reject the order if it exceeds the active order limit
    // if (data.side === 'buy' && activeBids >= 20) {
    //   res.status(400).json({ message: 'Maximum number of active bids reached' });
    //   return;
    // } else if (data.side === 'sell' && activeAsks >= 20) {
    //   res.status(400).json({ message: 'Maximum number of active asks reached' });
    //   return;
    // }

    // TODO: Update the active order count in Redis
    // if (order.side === 'sell') {
    //   await redisClient.set(activeBidsKey, activeBids + 1);
    // } else {
    //   await redisClient.set(activeAsksKey, activeAsks + 1);
    // }

    const newOrder = new Order({
      ...orderData,
      user_id: userId,
      action: 'create',
    });

    await redisClient.xAdd(
      `instrument:orders:${orderData.product_id}`,
      '*',
      newOrder.toRedisTuples(),
    );

    const { action, ...orderWithoutAction } = newOrder;
    const orderResponse: OrderResponse = {
      ...orderWithoutAction,
      created_at: new Date(orderWithoutAction.created_at * 1000).toISOString(),
    };
    res.status(201).json(orderResponse);
  } catch (error) {
    if (error instanceof z.ZodError) {
      res.status(400).json({ message: ERR_BAD_REQUEST });
    } else {
      console.error(error);
      res.status(500).json({ message: ERR_INTERNAL_SERVER });
    }
  }
});

// Get a single order
router.get('/:id', authenticate, async (req: Request, res: Response) => {
  try {
    const orderId = z.string().uuid().parse(req.params.id);
    const userId = res.locals.session.user.id;

    const result = await db.select().from(order).where(eq(order.id, orderId));
    const orderRecord = orderSelectSchema.parse(result[0]);

    if (!orderRecord) {
      res.status(401).json({ message: ERR_UNAUTHORIZED });
      return;
    }

    if (orderRecord.user_id !== userId) {
      res.status(403).json({ message: ERR_FORBIDDEN });
      return;
    }

    // Convert UNIX timestamp to human readable format
    orderRecord.created_at = new Date(
      parseInt(orderRecord.created_at),
    ).toISOString();
    res.status(200).json({ order: orderRecord });
  } catch (error) {
    if (error instanceof z.ZodError) {
      res.status(400).json({ message: ERR_BAD_REQUEST });
    } else {
      console.error(error);
      res.status(500).json({ message: ERR_INTERNAL_SERVER });
    }
  }
});

// Cancel an order
router.delete(
  '/:order_id',
  authenticate,
  async (req: Request, res: Response) => {
    try {
      const orderId = z.string().uuid().parse(req.params.order_id);
      const userId = res.locals.session.user.id;

      // TODO: Lookup order in DB and validate ownership
      const result = await db.select().from(order).where(eq(order.id, orderId));
      const orderRecord = orderSelectSchema.parse(result[0]);

      if (!orderRecord) {
        res.status(401).json({ message: ERR_UNAUTHORIZED });
        return;
      }

      if (orderRecord.user_id !== userId) {
        res.status(403).json({ message: ERR_FORBIDDEN });
        return;
      }

      // If order is done or cancelled fail fast
      if (orderRecord.status != 'open') {
        // TODO: Send cancel reject
        res.status(403).json({ message: ERR_FORBIDDEN });
        return;
      }

      const cancelPayload = { action: 'cancel', id: orderId };
      const streamKey = `instrument:orders:${orderRecord.product_id}`;

      await redisClient.xAdd(streamKey, '*', cancelPayload);

      // TODO: AWAIT A RESPONSE ON THE OUTPUT STREAM
      res.status(204).send();
    } catch (error) {
      if (error instanceof z.ZodError) {
        res.status(400).json({ message: ERR_BAD_REQUEST });
      } else {
        console.error(error);
        res.status(500).json({ message: ERR_INTERNAL_SERVER });
      }
    }
  },
);

export default router;
