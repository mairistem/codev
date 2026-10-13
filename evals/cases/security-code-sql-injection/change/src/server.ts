import express from "express";
import { requireRole } from "./auth";
import { searchUsers } from "./users";

export const app = express();
app.use(express.json());

app.get("/health", (_req, res) => {
  res.json({ status: "ok" });
});

app.get("/users/search", requireRole("support"), async (req, res) => {
  const term = typeof req.query.q === "string" ? req.query.q.trim() : "";
  if (term.length < 2 || term.length > 100) {
    res.status(400).json({ error: "q must be 2 to 100 characters" });
    return;
  }
  res.json(await searchUsers(term));
});
