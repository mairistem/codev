import { app } from "./server";
import { query } from "./db";

/** The public JSON of an order. */
export interface OrderResponse {
  id: number;
  status: "open" | "paid" | "shipped" | "cancelled";
}

app.get("/orders/:id", async (req, res) => {
  const rows = await query<{ id: number; state: OrderResponse["status"] }>(
    "SELECT id, state FROM orders WHERE id = $1",
    [req.params.id],
  );
  if (rows.length === 0) {
    res.status(404).end();
    return;
  }
  const response: OrderResponse = { id: rows[0].id, status: rows[0].state };
  res.json(response);
});
