import { z } from 'zod';
import { activeProducts } from '../config/index';

export const productSchema = z
  .string()
  .refine((productId) => activeProducts.has(productId), {
    message: 'Invalid product ID',
  });
