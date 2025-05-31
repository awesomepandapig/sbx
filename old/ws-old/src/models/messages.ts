import { z } from 'zod';
import { productSchema } from './product';

export enum MessageType {
  SUBSCRIBE = 'subscribe',
  UNSUBSCRIBE = 'unsubscribe',
}

export const BaseMessageSchema = z.object({
  type: z.nativeEnum(MessageType),
});

// Type of subcribe we send product_ids and chanells to subscribe to
export const SubscribeMessageSchema = BaseMessageSchema.extend({
  type: z.literal(MessageType.SUBSCRIBE),
  product_ids: z.array(productSchema),
  channel: z.string(),
  jwt: z.string().optional(),
});

// Unsubscribe message we send product_ids and channels to unsubscribe from
export const UnsubscribeMessageSchema = BaseMessageSchema.extend({
  type: z.literal(MessageType.UNSUBSCRIBE),
  product_ids: z.array(productSchema),
  channel: z.string(),
});

export const MessageSchema = z.union([
  SubscribeMessageSchema,
  UnsubscribeMessageSchema,
]);

export type SubscribeMessage = z.infer<typeof SubscribeMessageSchema>;
export type UnsubscribeMessage = z.infer<typeof UnsubscribeMessageSchema>;
export type Message = z.infer<typeof MessageSchema>;
