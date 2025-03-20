import { db } from '../db/index';
import { product } from '../db/schema';
import { z } from 'zod';

export const productSchema = z.object({
  id: z.string(),
});

type Product = z.infer<typeof productSchema>;

export async function getProducts(): Promise<Product[]> {
  const products = await db.select().from(product);
  return products.map((product) => ({
    id: product.id,
  }));
}

export async function validateProduct(product_id: string): Promise<boolean> {
  const products = await getProducts();
  return products.some((product) => product.id === product_id);
}
