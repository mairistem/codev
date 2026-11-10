import request from "supertest";
import { buildApp } from "../../app";

describe("POST /invoices", () => {
  it("refuses a negative amount with 400", async () => {
    const res = await request(buildApp())
      .post("/invoices")
      .send({ customer: "ACME", amountCents: -5 });
    expect(res.status).toBe(400);
    expect(res.body.issues).toHaveLength(1);
  });
});
