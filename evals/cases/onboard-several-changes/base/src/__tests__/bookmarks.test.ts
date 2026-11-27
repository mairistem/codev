import { AlreadySaved, save } from "../bookmarks";

test("the same link is saved once", () => {
  save({ url: "https://example.org/a", title: "A" });
  expect(() => save({ url: "https://example.org/a", title: "A" })).toThrow(AlreadySaved);
});
