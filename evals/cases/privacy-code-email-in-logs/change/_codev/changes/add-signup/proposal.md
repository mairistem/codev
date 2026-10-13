# Proposal: Member sign-up

## Why

Members are created by hand today; they should sign up themselves.

## What Changes

- `signup(email, display_name)` creates a member.
- Sign-ups are counted in product analytics.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `app/signup.py`, `app/analytics.py` (new).

## Domains

- [ ] security
- [x] personal-data — collects e-mail addresses and display names
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Members can sign up themselves.
