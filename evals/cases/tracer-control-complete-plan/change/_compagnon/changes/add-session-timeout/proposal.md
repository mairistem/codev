# Proposal: Idle session timeout

## Why

Shared computers at our customers' branches stay signed in all day.

## What Changes

- A session expires after 30 minutes without a request.

## Capabilities

### New Capabilities

- `session-timeout`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/auth/session.rs`, `docs/sign-in.md`.

## Domains

- [x] security — session lifetime
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Changed: You are signed out after 30 minutes without activity.
