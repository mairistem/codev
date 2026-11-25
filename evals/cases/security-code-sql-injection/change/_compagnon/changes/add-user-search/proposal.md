# Proposal: Search users by name

## Why

Support agents cannot find a customer without their id; they need to search by name.

## What Changes

- `GET /users/search?q=<term>` returns up to 50 users whose name contains the term.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/users.ts` (new), `src/server.ts` (route).

## Domains

- [x] security — a new endpoint reads a query-string parameter
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Search users by name with `GET /users/search?q=`.
