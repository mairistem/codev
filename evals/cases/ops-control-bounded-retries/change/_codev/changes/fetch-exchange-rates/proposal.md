# Proposal: Fetch exchange rates

## Why

Prices in other currencies use rates typed by hand every week.

## What Changes

- `fetch_rates()` reads the central bank's daily feed, configured by `RATES_FEED_URL`. When it fails, the previous day's rates stay in use (the caller keeps them).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/rates.rs` (new).

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [x] operations — a new outgoing call and a new environment variable
- [ ] migration

## Release note

- Added: Exchange rates are updated daily from the central bank.
