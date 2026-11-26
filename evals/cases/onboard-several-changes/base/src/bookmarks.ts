export interface Bookmark {
  url: string;
  title: string;
}

const saved = new Map<string, Bookmark>();

export class AlreadySaved extends Error {}

export function save(bookmark: Bookmark): void {
  if (saved.has(bookmark.url)) {
    throw new AlreadySaved(bookmark.url);
  }
  saved.set(bookmark.url, bookmark);
}

export function list(): Bookmark[] {
  return [...saved.values()];
}
