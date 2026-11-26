import express from "express";
import { AlreadySaved, list, save } from "./bookmarks";

export const app = express();
app.use(express.json());

app.get("/bookmarks", (_req, res) => res.json(list()));

app.post("/bookmarks", (req, res) => {
  try {
    save({ url: req.body.url, title: req.body.title });
    res.status(201).end();
  } catch (e) {
    res.status(e instanceof AlreadySaved ? 409 : 400).end();
  }
});

if (require.main === module) {
  app.listen(3000);
}
