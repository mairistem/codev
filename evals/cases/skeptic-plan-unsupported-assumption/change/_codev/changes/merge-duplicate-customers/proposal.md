# Proposal: Merge duplicate customers

## Why

Guest checkouts create a new customer each time; support sees five records for one person.

## What Changes

- A nightly job merges customers that share an e-mail address.

## Capabilities

### New Capabilities

- `customer-merge`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `jobs/merge_customers.py` (new).

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Changed: Customers who checked out several times as guests now have a single record.
