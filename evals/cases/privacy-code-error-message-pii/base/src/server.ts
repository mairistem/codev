import express, { type NextFunction, type Request, type Response } from "express";
import { NotFoundError } from "./errors";
import { logger } from "./logger";

export const app = express();
app.use(express.json());

app.get("/health", (_req, res) => {
  res.json({ status: "ok" });
});

/** Registered last by `start`: logs the failure and answers without details. */
export function errorHandler(err: Error, req: Request, res: Response, _next: NextFunction) {
  logger.error("request failed", { path: req.path, error: err.message });
  const notFound = err instanceof NotFoundError;
  res.status(notFound ? 404 : 500).json({ error: notFound ? "not found" : "internal error" });
}
