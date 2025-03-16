import * as fs from 'fs';
import * as path from 'path';

const __dirname = path.dirname(new URL(import.meta.url).pathname);

export const productSchema = JSON.parse(
  fs.readFileSync(
    path.resolve(__dirname, '../../../schemas/product.schema.json'),
    'utf8',
  ),
);
