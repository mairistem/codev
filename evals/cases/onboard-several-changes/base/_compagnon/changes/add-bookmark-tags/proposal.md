# Proposal: Bookmarks carry tags

## Why

The reading list mixes work and leisure links; there is no way to see only
one kind.

## What Changes

- A bookmark MAY carry tags; `GET /bookmarks?tag=<tag>` lists those
  carrying it.

## Capabilities

### New Capabilities

### Modified Capabilities

- bookmarks

### Removed Capabilities

## Impact

- `src/bookmarks.ts`, `src/server.ts` and their tests.

## Domains

None

## Release note

- Added: Bookmarks can carry tags, and the list can be filtered by tag.
