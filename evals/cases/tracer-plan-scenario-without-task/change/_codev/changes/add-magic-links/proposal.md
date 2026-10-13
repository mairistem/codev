# Proposal: Magic sign-in links

## Why

Members forget their password and the reset flow loses a third of them.

## What Changes

- Members can sign in with a single-use link e-mailed to them, valid 15 minutes.

## Capabilities

### New Capabilities

- `magic-links`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/auth/links.rs` (new), `docs/sign-in.md`.

## Domains

- [x] security — a new way to sign in
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Sign in with a link sent by e-mail.
