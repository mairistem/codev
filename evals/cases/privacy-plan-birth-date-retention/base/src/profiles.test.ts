import { describe, expect, it, vi } from "vitest";

vi.mock("./db", () => ({ query: vi.fn(async () => []) }));

import { query } from "./db";
import { saveProfile } from "./profiles";

describe("profiles", () => {
  it("saves the display name", async () => {
    await saveProfile({ userId: 7, displayName: "Ada" });
    expect(query).toHaveBeenCalledWith(expect.stringContaining("UPDATE profiles"), [7, "Ada"]);
  });
});
