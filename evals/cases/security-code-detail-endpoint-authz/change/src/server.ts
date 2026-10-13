import express, { type NextFunction, type Request, type Response } from "express";
import { requireUser } from "./auth";
import { invoices } from "./invoices";

export const app = express();
app.use(express.json());

app.get("/health", (_req, res) => {
  res.json({ status: "ok" });
});

app.use(requireUser);
app.use(invoices);

app.use((err: Error, _req: Request, res: Response, _next: NextFunction) => {
  console.error(err);
  res.status(500).json({ error: err.message, stack: err.stack });
});
