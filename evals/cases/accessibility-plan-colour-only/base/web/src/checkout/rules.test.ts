import { describe, expect, it } from "vitest";
import { nameError, postcodeError } from "./rules";

describe("checkout rules", () => {
  it("accepts a 5-digit postcode", () => {
    expect(postcodeError("75011")).toBeNull();
  });
  it("explains an invalid postcode", () => {
    expect(postcodeError("ABC")).toBe("Enter a 5-digit postcode, such as 75011.");
  });
  it("requires a name", () => {
    expect(nameError(" ")).toBe("Enter the name on the parcel.");
  });
});
