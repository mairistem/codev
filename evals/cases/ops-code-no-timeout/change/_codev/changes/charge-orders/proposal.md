# Proposal: Charge orders

## Why

Orders are charged by an operator in the provider's dashboard.

## What Changes

- `charge(order_id, amount_cents)` charges an order through the provider's API.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `app/payments.py` (new).

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [x] operations — a new outgoing call on the checkout path
- [ ] migration

## Release note

- Added: Orders are charged automatically.
