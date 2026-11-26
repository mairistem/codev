import express from "express";
import { all, create } from "./tasks";

export const app = express();
app.use(express.json());

app.get("/tasks", (_req, res) => res.json(all()));

app.post("/tasks", (req, res) => {
  try {
    res.status(201).json(create(String(req.body.title ?? "")));
  } catch (e) {
    res.status(400).json({ error: (e as Error).message });
  }
});

if (require.main === module) {
  app.listen(3000);
}
