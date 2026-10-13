import { Router } from "express";
import { currentUser } from "./auth";
import { query } from "./db";

export interface InvoiceRow {
  id: number;
  org_id: number;
  number: string;
  total_cents: number;
  issued_on: string;
}

export const invoices = Router();

/** The caller's organization's invoices, newest first. */
invoices.get("/invoices", async (req, res) => {
  const user = currentUser(req);
  const rows = await query<InvoiceRow>(
    "SELECT id, org_id, number, total_cents, issued_on FROM invoices WHERE org_id = $1 ORDER BY issued_on DESC LIMIT 100",
    [user.orgId],
  );
  res.json(rows);
});

/** One invoice, by id. */
invoices.get("/invoices/:id", async (req, res) => {
  const id = Number(req.params.id);
  if (!Number.isInteger(id)) {
    res.status(400).json({ error: "id must be a number" });
    return;
  }
  const rows = await query<InvoiceRow>(
    "SELECT id, org_id, number, total_cents, issued_on FROM invoices WHERE id = $1",
    [id],
  );
  if (rows.length === 0) {
    res.status(404).json({ error: "not found" });
    return;
  }
  res.json(rows[0]);
});
