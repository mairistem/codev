import express from "express";
import { book } from "./appointments";

const app = express();
app.use(express.json());

app.post("/appointments", async (req, res) => {
  await book({ ...req.body, at: new Date(req.body.at) });
  res.status(201).end();
});

app.listen(3000);
