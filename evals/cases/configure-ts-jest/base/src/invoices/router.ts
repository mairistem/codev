import { Router } from "express";
import { z } from "zod";
import { InvoiceStore } from "./store";

const NewInvoice = z.object({
  customer: z.string().min(1),
  amountCents: z.number().int().positive(),
});

const store = new InvoiceStore();

export const invoicesRouter = Router();

invoicesRouter.get("/", (_req, res) => {
  res.json(store.list());
});

invoicesRouter.post("/", (req, res) => {
  const parsed = NewInvoice.safeParse(req.body);
  if (!parsed.success) {
    // 400 with every issue, so a client fixes its request in one pass.
    return res.status(400).json({ issues: parsed.error.issues });
  }
  res.status(201).json(store.add(parsed.data));
});
