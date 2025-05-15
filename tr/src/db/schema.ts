import {
  pgTable,
  uuid,
  text,
  numeric,
  timestamp,
  boolean,
  pgEnum,
} from 'drizzle-orm/pg-core';

export const orderSideEnum = pgEnum('order_side', ['buy', 'sell']);
export const orderTypeEnum = pgEnum('order_type', ['market', 'limit']);
export const orderStatusEnum = pgEnum('order_status', ['open', 'done', 'cancelled']);
export const cancelAfterEnum = pgEnum('cancel_after', ['min', 'hour']);

export const order = pgTable('order', {
  id: uuid('id').primaryKey(),
  product_id: text('product_id')
    .notNull()
    .references(() => product.id, { onDelete: 'cascade' }),
  user_id: text('user_id')
    .notNull()
    .references(() => user.id, { onDelete: 'cascade' }),
  side: orderSideEnum('side').notNull(),
  type: orderTypeEnum('type').notNull(),
  created_at: numeric('created_at').notNull(),
  executed_value: numeric('executed_value').notNull().default('0'),
  status: orderStatusEnum('status').notNull(),
  settled: boolean('settled').notNull(),
  price: numeric('price'),
  cancel_after: cancelAfterEnum('cancel_after'),
  size: numeric('size').notNull(),
});

export const product = pgTable('product', {
  id: text('id').primaryKey().notNull().unique(),
});

export const user = pgTable('user', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  email: text('email').notNull().unique(),
  emailVerified: boolean('email_verified').notNull(),
  image: text('image'),
  createdAt: timestamp('created_at').notNull(),
  updatedAt: timestamp('updated_at').notNull(),
  minecraftId: text('minecraft_id'),
});