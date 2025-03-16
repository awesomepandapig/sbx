import { z } from 'zod';
import { productSchema } from './product';

export enum MessageType {
  AUTHENTICATE = 'authenticate',
  SUBSCRIBE = 'subscribe',
  UNSUBSCRIBE = 'unsubscribe',
}

export const BaseMessageSchema = z.object({
  type: z.nativeEnum(MessageType),
});

export const AuthenticateMessageSchema = BaseMessageSchema.extend({
  type: z.literal(MessageType.AUTHENTICATE),
  token: z.string(),
});

// Type of subcribe we send product_ids and chanells to subscribe to
export const SubscribeMessageSchema = BaseMessageSchema.extend({
  type: z.literal(MessageType.SUBSCRIBE),
  product_ids: z.array(z.nativeEnum(productSchema)),
});

// Unsubscribe message we send product_ids and chanells to unsubscribe from
export const UnsubscribeMessageSchema = BaseMessageSchema.extend({
  type: z.literal(MessageType.UNSUBSCRIBE),
  product_ids: z.array(z.nativeEnum(productSchema)),
});

export const MessageSchema = z.union([
  AuthenticateMessageSchema,
  SubscribeMessageSchema,
  UnsubscribeMessageSchema,
]);

export type AuthenticateMessage = z.infer<typeof AuthenticateMessageSchema>;
export type SubscribeMessage = z.infer<typeof SubscribeMessageSchema>;
export type UnsubscribeMessage = z.infer<typeof UnsubscribeMessageSchema>;
export type Message = z.infer<typeof MessageSchema>;

// subscriptiins message contains all the challens client is subsrcibed to

// TODO: REPLACE WITH AJV SCHEMAS INSTEAD OF ZOD
