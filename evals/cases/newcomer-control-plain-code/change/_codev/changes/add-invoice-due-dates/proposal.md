# Proposal: Invoice due dates

## Why

Accounting computes due dates by hand.

## What Changes

- Invoices are due 30 days after issue; overdue from the next day.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `app/invoices.py` (new) and its tests.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Invoices show their due date.
