import { create } from "../tasks";

test("a task without a title is refused", () => {
  expect(() => create("  ")).toThrow(RangeError);
});
