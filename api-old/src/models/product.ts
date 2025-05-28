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

let productCache: Set<string>;
let lastFetched = 0;
const CACHE_DURATION_MS = 30_000; // 30 seconds

async function refreshProductCache(): Promise<Set<string>> {
  const products = await db.select().from(product);
  productCache = new Set(products.map((p) => p.id));
  lastFetched = Date.now();
  return productCache;
}

export async function validateProduct(product_id: string): Promise<boolean> {
  if (!productCache || Date.now() - lastFetched > CACHE_DURATION_MS) {
    await refreshProductCache();
  }
  return productCache.has(product_id);
}
