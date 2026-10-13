# Proposal: Import partner accounts

## Why

Partner accounts are created by hand from their monthly CSV.

## What Changes

- `import(db, rows)` creates missing accounts and their seats.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/import.rs` (new).

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Partner accounts are imported from their CSV export.
