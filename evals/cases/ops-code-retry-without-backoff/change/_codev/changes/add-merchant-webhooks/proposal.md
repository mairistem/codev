# Proposal: Webhooks for merchants

## Why

Merchants poll the payments API every minute to learn that a payment succeeded.

## What Changes

- Each settled or refunded payment is posted to the merchant's webhook URL; a failed delivery is retried, up to five attempts. Merchants deduplicate by the `x-event-id` header.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/webhooks.ts` (new).

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [x] operations — outgoing calls to merchants' servers, retried
- [ ] migration

## Release note

- Added: Payment webhooks for merchants.
