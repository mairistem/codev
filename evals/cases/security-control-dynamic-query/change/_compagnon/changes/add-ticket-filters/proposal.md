# Proposal: Filter and sort tickets in the console

## Why

Agents scroll through every open ticket to find theirs.

## What Changes

- The console filters tickets by status, assignee and priority, and sorts them by date or priority.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `app/search.py` (new).

## Domains

- [x] security — a query built from the console's parameters
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Filter and sort tickets in the support console.
