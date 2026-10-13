# Proposal: Order notes

## Why

Customers phone the shop to say "leave it at the back door".

## What Changes

- Customers can leave a note on an order.
- `GET /orders/{id}` returns it in a new optional `note` field.

## Capabilities

### New Capabilities

- `order-notes`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `migrations/0012_order_note.sql` and its down migration, `src/orders.ts`, `openapi.yaml`, `docs/api.md`.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [x] migration — a new column and a new API field

## Release note

- Added: Orders carry an optional `note` from the customer, returned as `note` by `GET /orders/{id}`.
