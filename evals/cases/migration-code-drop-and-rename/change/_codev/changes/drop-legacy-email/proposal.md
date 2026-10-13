# Proposal: Clean up the accounts table

## Why

`legacy_email` has not been written since 2023, and our API mixes naming styles.

## What Changes

- Drop `users.legacy_email`.
- Account JSON uses camelCase.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `migrations/0005_drop_legacy_email.sql`, `src/api.rs`.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [x] migration — a schema change

## Release note

- Changed: Account JSON uses camelCase.
