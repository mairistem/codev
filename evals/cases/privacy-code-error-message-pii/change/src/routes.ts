import { currentCustomerId, requireRole } from "./auth";
import { app, errorHandler } from "./server";
import { chatWidgetSettings, customerByEmail, customerById } from "./support";

app.post("/support/customers/lookup", requireRole("support"), async (req, res, next) => {
  try {
    res.json(await customerByEmail(String(req.body.email ?? "")));
  } catch (error) {
    next(error);
  }
});

app.get("/chat/settings", async (req, res, next) => {
  try {
    res.json(chatWidgetSettings(await customerById(currentCustomerId(req))));
  } catch (error) {
    next(error);
  }
});

app.use(errorHandler);
