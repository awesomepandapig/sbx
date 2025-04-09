import { activeProducts, redisClient, waitForRedis } from 'config';
import { handleAuth } from 'messages';
import { AuthenticatedWebSocket, SubscribeMessage, UnsubscribeMessage } from 'models';

// Tracks subscriptions by WebSocket
const subscriptions = new Map<AuthenticatedWebSocket, Set<string>>();

// Tracks active sockets by user ID
const sockets = new Map<string, AuthenticatedWebSocket>();

export async function subscribe(
  ws: AuthenticatedWebSocket,
  message: SubscribeMessage
) {
    // Reject subscription if user is already subscribed
    if(subscriptions.get(ws)) return;

    // Authenticate user
    await handleAuth(ws, message);
    if(!ws.user_id) return;

    // If no products, subscribe to all products
    let products = new Set(message.product_ids);  
  if (products.size === 0) {
    products = new Set(activeProducts);
  }

  // Set subscription
  subscriptions.set(ws, products);

  // Send subscription confirmation
  ws.sendMessage('subscriptions', [
    { subscriptions: { user: Array.from(products) } },
  ]);
  await sendSnapshots(ws, products);
}

export function unsubscribe(ws: AuthenticatedWebSocket,
    message: UnsubscribeMessage
) {
    let products = new Set(message.product_ids);

  // Get the current subscription
  const subscribedProducts = subscriptions.get(ws);

  // If not subscribed, send empty subscription message
  if (!subscribedProducts) {
    ws.sendMessage('subscriptions', [{ subscriptions: {} }]);
    return;
  }

  // If no products specified, unsubscribe from all
  if (products.size === 0) {
    subscriptions.delete(ws);
    ws.sendMessage('subscriptions', [{ subscriptions: {} }]);
    return;
  }

  // Remove specified products
  for (const product of products) {
    subscribedProducts.delete(product);
  }

  // Update subscription state
  if (subscribedProducts.size === 0) {
    subscriptions.delete(ws);
  }

  // Send updated subscription
  ws.sendMessage('subscriptions', [
    { subscriptions: { user: Array.from(subscribedProducts) } },
  ]);
}

export function cleanup(ws: AuthenticatedWebSocket) {
  subscriptions.delete(ws);
}

function handleUpdate(message: string) {
  // console.log(message);
  

  //TODO:
  // const ws = sockets.get(order.user_id);
  // if (!ws) return;

  // const events = [{ type: "update", orders: [order] }];
  // ws.sendMessage("user", events);
}

async function sendSnapshots(
  ws: AuthenticatedWebSocket,
  products: Set<string>,
) {
  // For each product
  for (const productId of products) {
    const userId = ws.user_id;
    if(!userId) return;
    
    const orderIds = await redisClient.sMembers(`user:${userId}:order:${productId}`);
    if (!orderIds) return;

    const orders = [];
    for(const orderId of orderIds) {
        const orderStr = await redisClient.hGetAll(`order:${orderId}`);
        orders.push(
            orderStr
        )
    }    

    const events = [];
    let batch = [];
    for (const order of orders) {
      if (order.status === 'open') {
        batch.push(order);

        if (batch.length === 50) {
          events.push({
            type: 'snapshot',
            product_id: productId,
            orders: [...batch],
          });
          batch = [];
        }
      }
    }

    if (batch.length > 0) {
      events.push({
        type: 'snapshot',
        product_id: productId,
        orders: [...batch],
      });
    }

    for (const event of events) {
      ws.sendMessage('user', [event]);
    }
  }
}

let initialized = false;

// async function initialize() {
//   if (initialized) return;

//   await waitForRedis();

//   const redisPubSub = redisClient.duplicate();
//   await redisPubSub.connect();

//   const channels = Array.from(activeProducts).map(
//     (productId) => `${productId}:updates`,
//   );
//   await redisPubSub.subscribe(channels, (message) => {
//     handleUpdate(message);
//   });

//   console.log('User channel initialized with pub/sub model');
//   initialized = true;
// }

// initialize();
