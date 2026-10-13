# Proposal: Orders carry their currency

## Why

The shop now sells in pounds as well as euros; partners cannot tell which currency an order's total is in.

## What Changes

- The order JSON gains a `currency` field.
- The configuration file sets the currency of orders that do not name one.
- The sales report's internal type gets clearer names.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/api.rs`, `src/config.rs`, `src/report.rs`, `config.example.toml`, `docs/api.md`.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [x] migration — a new field in the public API and a new configuration key

## Release note

- Added: Orders carry their `currency`.
