# Proposal: Clearer setting and field names

## Why

Operators confuse `listen` with a log setting, and the code calls customers buyers everywhere but in the API type.

## What Changes

- The configuration file's `listen` becomes `bind_address`; `listen` is still read.
- The API type's `customer_id` field is called `buyer_id` in the code; the JSON is unchanged.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/config.rs`, `src/api.rs`, `config.example.toml`.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [x] migration — a configuration key renamed, an API type's field renamed

## Release note

- Deprecated: `listen` in the configuration file is now `bind_address`; `listen` is still read.
