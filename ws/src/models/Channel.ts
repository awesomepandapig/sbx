import RedisClient from '@redis/client/dist/lib/client';
import { RedisFunctions, RedisModules, RedisScripts } from 'redis';
import { activeProducts, redisSubscriber, waitForRedis } from '../config/index';
import {
  AuthenticatedWebSocket,
  SubscribeMessage,
  UnsubscribeMessage,
} from 'models/index';

export abstract class Channel {
  protected subscribers = new Set<AuthenticatedWebSocket>();

  protected redisPubSub!: RedisClient<
    RedisModules,
    RedisFunctions,
    RedisScripts
  >;

  constructor(protected channels: string[]) {}

  public static async initialize<T extends Channel>(
    this: new (channels: string[]) => T,
    channels: string[],
  ): Promise<T> {
    await waitForRedis();

    const redisPubSub = await redisSubscriber.duplicate();
    await redisPubSub.connect();

    const channel = new this(channels);
    channel.redisPubSub = redisPubSub;

    await Promise.all(
      channels.map((channelName) =>
        redisPubSub
          .subscribe(channelName, async (message, channelName) => {
            await channel.handleUpdate(message, channelName);
          })
          .then(() => {
            console.log(`${channelName} channel initialized`);
          }),
      ),
    );

    return channel;
  }

  protected sendSubscriptions(ws: AuthenticatedWebSocket) {
    const subscriptions: Record<string, string[]> = {};
    for (const [channel, products] of ws.subscriptions.entries()) {
      subscriptions[channel] = Array.from(products);
    }
    ws.sendMessage('subscriptions', [{ subscriptions }]);
  }

  protected addProductsToChannel(
    ws: AuthenticatedWebSocket,
    channel: string,
    products: Set<string>,
  ): Set<string> {
    // Initialize subscription map for this channel if not exists
    if (!ws.subscriptions.has(channel)) {
      ws.subscriptions.set(channel, new Set());
    }

    // Get the current subscription
    const channelSubscriptions = ws.subscriptions.get(channel);
    if (!channelSubscriptions) return new Set();

    // Add the products to the subscription list and track new ones
    const newProducts = new Set<string>();
    for (const productId of products) {
      if (!channelSubscriptions.has(productId)) {
        channelSubscriptions.add(productId);
        newProducts.add(productId);
      }
    }

    return newProducts;
  }

  protected abstract onSubscribe(
    ws: AuthenticatedWebSocket,
    channel: string,
    newProducts: Set<string>,
  ): Promise<void>;

  public async subscribe(
    ws: AuthenticatedWebSocket,
    message: SubscribeMessage,
  ) {
    const channel = message.channel;
    let products = new Set(message.product_ids);

    // If no products, subscribe to all products
    if (products.size === 0) {
      products = new Set(activeProducts);
    }

    // Add socket to subscriber list
    this.subscribers.add(ws);

    // Add products and get newly added ones
    const newProducts = this.addProductsToChannel(ws, channel, products);

    // Send updated subscriptions to the client
    this.sendSubscriptions(ws);

    // Call the hook for subclasses to extend behavior
    await this.onSubscribe(ws, channel, newProducts);
  }

  public unsubscribe(ws: AuthenticatedWebSocket, message: UnsubscribeMessage) {
    const channel = message.channel;
    const products = new Set(message.product_ids);

    // Get the current subscription
    const channelSubscriptions = ws.subscriptions.get(channel);
    if (!channelSubscriptions) {
      this.sendSubscriptions(ws);
      return;
    }

    // If no products specified, unsubscribe from all
    if (products.size === 0) {
      ws.subscriptions.delete(channel);
      this.sendSubscriptions(ws);
      return;
    }

    // Remove specified products
    for (const product of products) {
      channelSubscriptions.delete(product);
    }

    // Remove the channel if no products remain
    if (channelSubscriptions.size === 0) {
      ws.subscriptions.delete(channel);
    }

    this.sendSubscriptions(ws);
  }

  protected abstract handleUpdate(
    message: string,
    channelName: string,
  ): Promise<void>;

  public cleanup(ws: AuthenticatedWebSocket) {
    this.subscribers.delete(ws);
  }
}
