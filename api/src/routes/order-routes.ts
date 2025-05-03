import express, { Request, Response } from 'express';
import { authenticate } from '../lib/middleware';
import { validateProduct } from '../models/product';
import { Order, OrderSchema, OrderResponse } from 'models/order';
import { z } from 'zod';
import { activeProducts, redisClient } from '../config/index';

const router = express.Router();

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
    const data = OrderSchema.parse(req.body);
    const userId = res.locals.session.user.id;

    const validProduct = await validateProduct(data.product_id);
    if (!validProduct) {
      res.status(400).json({ message: 'Invalid product ID' });
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

    // Create new order
    const order = new Order({ ...data, user_id: userId, action: "create"});
    const orderStringified = order.toRedisTuples();

    // Add the message to the message stream
    const streamKey = `instrument:orders:${order.product_id}`;
    await redisClient.xAdd(
      streamKey,
      '*',
      orderStringified,
    );

    // TODO: Update the active order count in Redis
    // if (order.side === 'sell') {
    //   await redisClient.set(activeBidsKey, activeBids + 1);
    // } else {
    //   await redisClient.set(activeAsksKey, activeAsks + 1);
    // }

    // Return the response
    const { action, ...orderWithoutAction } = order; // Destructure to omit 'action'
    const orderResponse: OrderResponse = {
      ...orderWithoutAction,
      created_at: new Date(orderWithoutAction.created_at * 1000).toISOString(),
    };

    res.status(201).json(orderResponse);
  } catch (error) {
    // TODO: Handle error
    console.log(error);
    res.status(400).json({ message: 'bad request' });
  }
});

// // Get a single order
// router.get('/:id', authenticate, async (req: Request, res: Response) => {
//   try {
//     const orderId = z.string().uuid().parse(req.params.id);
//     const userId = res.locals.session.user.id;

//     const orderKey = `order:${orderId}`;
//     TODO: get order from DB
//     const order = await redisClient.hGetAll(orderKey);

//     if (!order || Object.keys(order).length === 0) {
//       res.status(404).json({ message: 'Order not found or expired' });
//       return;
//     }

//     TODO: Validate ownership
//     if (order.user_id !== userId) {
//       res.status(403).json({ message: 'Unauthorized' });
//       return;
//     }

//     // Convert UNIX timestamp to human readable format
//     order.created_at = new Date(parseInt(order.created_at)).toISOString();
//     res.status(200).json({ order });
//   } catch (error) {
//     // TODO: if error is zod error 400
//     // TODO: else 500
//     console.log(error);
//   }
// });

// Cancel an order
router.delete(
  '/:order_id',
  authenticate,
  async (req: Request, res: Response) => {
    try {
      // Get order id and product_id from params
      const orderId = z.string().uuid().parse(req.params.order_id);
      // TODO: Make productID optional since we can get the order from the db without knowing the id... is just slower
      const productId = req.query.product_id;
      if(!productId) {
        console.warn("PRODUCT ID MISSING")
      }

      // TODO: Lookup order in DB and validate ownership

      const payload = {
        action: "cancel",
        id: orderId
      }

      // Send cancel request to the matching engine
      const streamKey = `instrument:orders:${productId}`;
      await redisClient.xAdd(
        streamKey,
        '*', // TODO: streamId
        payload,
      );

      // TODO: AWAIT A RESPONSE ON THE OUTPUT STREAM
      
      // Return the response
      const response = {};
  
      res.status(204).json(response);
    } catch (error) {
      // TODO: Handle
      console.log(error);
      res.status(400).json({ message: 'bad request' });
    }
});

export default router;
