import { describe, expect, it } from "vitest";
import { maskEmails } from "./logger";

describe("maskEmails", () => {
  it("keeps the first letter and the domain", () => {
    expect(maskEmails("from ada.lovelace@example.org")).toBe("from a***@example.org");
  });
  it("leaves text without an address unchanged", () => {
    expect(maskEmails("order 42 shipped")).toBe("order 42 shipped");
  });
});
