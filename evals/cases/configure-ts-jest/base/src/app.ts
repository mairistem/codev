import express from "express";
import { invoicesRouter } from "./invoices/router";

// The app is built apart from the server so that tests can drive it with
// supertest without opening a port.
export function buildApp() {
  const app = express();
  app.use(express.json());
  app.use("/invoices", invoicesRouter);
  return app;
}
