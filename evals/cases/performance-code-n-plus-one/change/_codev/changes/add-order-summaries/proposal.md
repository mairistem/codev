# Proposal: Order summaries

## Why

The customer page shows each order's total, computed in the browser from every line.

## What Changes

- `order_summaries` returns the last 100 orders of a customer with their totals.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `api/src/orders.rs`; the web page already calls the endpoint.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [x] performance — runs on every customer page view
- [ ] operations
- [ ] migration

## Release note

- Changed: Order totals are computed by the API.
