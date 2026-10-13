# Proposal: Batch cart prices

## Why

The cart total queried each product's price separately; large carts were slow.

## What Changes

- `cartTotal` reads all prices in one query. Carts are limited to 100 items by the existing validation.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/cart.ts`.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [x] performance — removes a query per cart item
- [ ] operations
- [ ] migration

## Release note

- Changed: Large carts load faster.
