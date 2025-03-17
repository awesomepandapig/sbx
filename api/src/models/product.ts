import { db } from '../db/index';
import { product } from '../db/index';
import { FromSchema } from 'json-schema-to-ts';

const productSchemaTemplate = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://skyblock.exchange/product.schema.json',
  title: 'Order',
  description: 'A product in the catalog',
  type: 'string',
  enum: [''] as string[],
} as const;

let productSchema = productSchemaTemplate;

async function updateSchema() {
  const products = await db.select().from(product);
  productSchema = {
    ...productSchemaTemplate,
    enum: products.map((product) => product.name),
  };
}

// Cache the schema for 30 minutes and then refetch
setInterval(updateSchema, 30 * 60 * 1000);

updateSchema();
export { productSchema };
export type Product = FromSchema<typeof productSchema>;
