# Proposal: Rename an order's status to state

## Why

The API says `status` where the rest of the system says `state`.

## What Changes

- The order JSON's `status` field is renamed `state`.

## Capabilities

### New Capabilities

- `orders-api`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/orders.ts`, `openapi.yaml`.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [x] migration — a public field changes name

## Release note

- Changed: The order's `status` field is now called `state`.
