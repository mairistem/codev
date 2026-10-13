# Proposal: API tokens for integrations

## Why

Integrations use a person's password today; a leaked integration exposes the account.

## What Changes

- Users create and revoke personal API tokens.
- The API accepts `Authorization: Bearer <token>`.

## Capabilities

### New Capabilities

- `api-tokens`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/http/middleware.rs`, a new `api_tokens` table, `docs/api.md`.

## Domains

- [x] security — new credentials and authentication path
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Personal API tokens for integrations.
