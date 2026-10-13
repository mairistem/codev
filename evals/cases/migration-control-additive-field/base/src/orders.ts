import { app } from "./server";
import { query } from "./db";

interface OrderRow {
  id: number;
  total_cents: number;
  shipped_at: string | null;
}

/** The public JSON of an order: optional fields are omitted when empty. */
export function orderJson(row: OrderRow) {
  return {
    id: row.id,
    totalCents: row.total_cents,
    ...(row.shipped_at === null ? {} : { shippedAt: row.shipped_at }),
  };
}

app.get("/orders/:id", async (req, res) => {
  const rows = await query<OrderRow>("SELECT id, total_cents, shipped_at FROM orders WHERE id = $1", [req.params.id]);
  if (rows.length === 0) {
    res.status(404).end();
    return;
  }
  res.json(orderJson(rows[0]));
});
