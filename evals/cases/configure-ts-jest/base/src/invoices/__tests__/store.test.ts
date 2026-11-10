import { InvoiceStore } from "../store";

describe("InvoiceStore", () => {
  it("numbers invoices from 1", () => {
    const store = new InvoiceStore();
    expect(store.add({ customer: "ACME", amountCents: 1200 }).id).toBe(1);
  });

  it("lists a copy", () => {
    const store = new InvoiceStore();
    store.add({ customer: "ACME", amountCents: 1200 });
    store.list().pop();
    expect(store.list()).toHaveLength(1);
  });
});
