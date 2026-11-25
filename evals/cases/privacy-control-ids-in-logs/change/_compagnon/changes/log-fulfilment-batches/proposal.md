# Proposal: Log what fulfilment ships

## Why

When a customer asks where their parcel is, operations cannot tell whether the nightly fulfilment run booked its pickup.

## What Changes

- Each shipped order is logged with its id and parcel count, and each batch with its counts and duration.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `app/fulfilment.py` (new).

## Domains

- [ ] security
- [x] personal-data — new log lines about customers' orders
- [ ] user-interface
- [ ] performance
- [x] operations — logs of the nightly fulfilment run
- [ ] migration

## Release note

- Added: The fulfilment run logs each order it ships.
