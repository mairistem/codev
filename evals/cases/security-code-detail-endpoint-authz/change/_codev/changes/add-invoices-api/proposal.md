# Proposal: Invoices API for the accounting app

## Why

The accounting app scrapes the invoices page; it needs an API to list invoices and read one.

## What Changes

- `GET /invoices` lists the caller's organization's invoices, newest first.
- `GET /invoices/:id` returns one invoice.
- Errors are answered as JSON.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/invoices.ts` (new), `src/server.ts` (routes and error handler).

## Domains

- [x] security — new endpoints serving each organization's invoices
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: An API to list invoices and read one, for the accounting app.
