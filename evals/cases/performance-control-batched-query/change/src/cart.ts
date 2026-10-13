import { query } from "./db";

export interface CartItem {
  productId: number;
  quantity: number;
}

interface PriceRow {
  id: number;
  price_cents: number;
}

/** The cart's total, with one query for all its products. */
export async function cartTotal(items: CartItem[]): Promise<number> {
  if (items.length === 0) {
    return 0;
  }
  const ids = [...new Set(items.map((item) => item.productId))];
  const rows = await query<PriceRow>("SELECT id, price_cents FROM products WHERE id = ANY($1)", [ids]);
  const prices = new Map(rows.map((row) => [row.id, row.price_cents]));
  return items.reduce((total, item) => total + (prices.get(item.productId) ?? 0) * item.quantity, 0);
}
