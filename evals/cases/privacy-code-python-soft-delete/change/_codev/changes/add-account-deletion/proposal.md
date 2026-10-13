# Proposal: Delete my account

## Why

Members write to support to have their account and data erased; the law gives them that right and support handles it by hand.

## What Changes

- A "Delete my account" button erases the member's personal data and signs them out everywhere; their orders are kept for accounting, without personal data.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `app/account.py` (new).

## Domains

- [ ] security
- [x] personal-data — erasure of members' personal data on request
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Members can delete their account and its personal data themselves.
