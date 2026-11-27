# Proposal: Import bookmarks from a browser export

## Why

Each new user starts with an empty list while their browser holds hundreds
of bookmarks.

## What Changes

- `POST /bookmarks/import` reads a browser's HTML export (the Netscape
  bookmark format) and saves every link not already saved.

## Capabilities

### New Capabilities

### Modified Capabilities

- bookmarks

### Removed Capabilities

## Impact

- A new `src/import.ts`, `src/server.ts`, and their tests.

## Domains

None

## Release note

- Added: Bookmarks can be imported from a browser's HTML export.
