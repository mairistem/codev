import { describe, expect, it, vi } from "vitest";

vi.mock("./db", () => ({ query: vi.fn(async () => []) }));

import { query } from "./db";
import { invoices } from "./invoices";

describe("invoices", () => {
  it("lists only the caller's organization", async () => {
    const list = invoices.stack.find((layer) => layer.route?.path === "/invoices")!;
    const res = { json: vi.fn() };
    await list.route!.stack[0].handle({ user: { id: 1, orgId: 7 } } as never, res as never, vi.fn());
    expect(query).toHaveBeenCalledWith(expect.stringContaining("WHERE org_id = $1"), [7]);
  });
});
