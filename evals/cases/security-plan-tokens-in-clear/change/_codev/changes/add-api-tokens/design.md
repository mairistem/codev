# Design: API tokens

## Context

Accounts sign in with a password (bcrypt hashes in `users.password_hash`).
Integrations currently share a user's password. The service is written in
Rust; requests go through one middleware, `src/http/middleware.rs`.

## Goals / Non-Goals

**Goals:** personal tokens an integration can use; revocation.

**Non-Goals:** OAuth, scopes.

## Decisions

### Decision: Tokens are random 32-byte values, base64url-encoded

Generated with the operating system's random source.

**Alternatives considered**: UUIDv4 (less entropy than we want).

### Decision: Tokens are stored as given

The `api_tokens` table holds `user_id`, `token` and `created_at`; the
middleware looks a request's token up with `WHERE token = $1`. Storing the
token as given keeps the lookup a simple indexed equality.

**Alternatives considered**: none, the lookup must be fast.

### Decision: Every request is logged with its headers

To diagnose integration problems, the middleware logs each request's
method, path and headers at `info` level, including `Authorization`, so
support can see which token an integration sent.

**Alternatives considered**: logging only the path (not enough to debug).

## Risks / Trade-offs

- [A user loses a token] → they create another one and revoke the old one.
