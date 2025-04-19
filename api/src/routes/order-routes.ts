import express, { Request, Response } from 'express';
import { authenticate } from '../lib/middleware';
import { validateProduct } from '../models/product';
import { Order, OrderSchema, OrderResponse } from 'models/order';
import { z } from 'zod';
import { activeProducts, redisClient } from '../config/index';
import { order as OrderTable } from '../db/schema'
import { db } from '../db/index';
import { createInsertSchema } from 'drizzle-zod';

const router = express.Router();

let sequence_num = 0;

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
  res.status(200).json({orders: []})
});

// // TODO: Cancel all orders
// router.delete('/orders', authenticate, async (req: Request, res: Response) => {
//   try {
//   } catch (error) {}
// });

// Create a new order
router.post('/', authenticate, async (req: Request, res: Response) => {
  try {
    const data = OrderSchema.parse(req.body);

    

    // TODO: ADD HYPIXEL PURSE CHECK TO ENSURE USER HAS SUFFICIENT FUNDS FOR BID

    const userId = res.locals.session.user.id;

    const validProduct = await validateProduct(data.product_id);
    if (!validProduct) {
      res.status(400).json({ message: 'Invalid product ID' });
      return;
    }

    // // Use Redis to check the active orders count for this user in this product
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

    
    
    

    // Create new order
    const order = new Order({ ...data, user_id: userId });
    const orderStringified = order.toRedisTuples();

    // Add the message to the message stream
    const streamKey = `instrument:orders:${order.product_id}`;
    const streamId = `${Date.now()}-${sequence_num}`;
    await redisClient.xAdd(
      streamKey,
      '*', // TODO: streamId
      orderStringified,
    );
    sequence_num++;

    // TODO: Add order to db
    // function dbOrderFromOrder(order: Order) {
    //   return {
    //     id: order.id,
    //     product_id: order.product_id,
    //     user_id: order.user_id,
    //     side: order.side,
    //     type: order.type,
    //     created_at: order.created_at, // assuming `created_at` is a UNIX timestamp
    //     executed_value: order.executed_value.toString(), // Drizzle uses string for numeric
    //     status: order.status,
    //     settled: order.settled,
    //     price: order.price?.toString(),
    //     cancel_after: order.cancel_after,
    //     size: order.size.toString(),
    //   };
    // }
    // const orderInsertSchema = createInsertSchema(OrderTable);
    // console.log(order);
    // const dataParsed = dbOrderFromOrder(order);
    // console.log(dataParsed);
    // const dbOrder = await db.insert(OrderTable).values(dataParsed);
    // console.log(dbOrder);

    // // Update the active order count in Redis
    // if (order.side === 'sell') {
    //   await redisClient.set(activeBidsKey, activeBids + 1);
    // } else {
    //   await redisClient.set(activeAsksKey, activeAsks + 1);
    // }

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

    const orderKey = `order:${orderId}`;
    const order = await redisClient.hGetAll(orderKey);

    if (!order || Object.keys(order).length === 0) {
      res.status(404).json({ message: 'Order not found or expired' });
      return;
    }

    if (order.user_id !== userId) {
      res.status(403).json({ message: 'Unauthorized' });
      return;
    }

    // Convert UNIX timestamp to human readable format
    order.created_at = new Date(parseInt(order.created_at)).toISOString();
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
