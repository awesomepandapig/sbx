import * as fs from 'fs';
import * as path from 'path';
import { ajv } from '../config/index';
import { productSchema } from './product';
import { FromSchema } from 'json-schema-to-ts';

const __dirname = path.dirname(new URL(import.meta.url).pathname);

const orderSchema = JSON.parse(
  fs.readFileSync(
    path.resolve(__dirname, '../../../schemas/order.schema.json'),
    'utf8',
  ),
);

export type Order = FromSchema<typeof orderSchema>;
export const validateOrder = ajv.addSchema(productSchema).compile(orderSchema);
