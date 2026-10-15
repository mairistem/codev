# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### security

#### R-security-1 API tokens are stored in clear in the database

```yaml
severity: high
confidence: high
location: "design.md#Decision: Tokens are stored as given"
evidence: |
  The `api_tokens` table holds `user_id`, `token` and `created_at`; the
  middleware looks a request's token up with `WHERE token = $1`. Storing the
  token as given keeps the lookup a simple indexed equality.
recommendation: >-
  Store only a hash of the token (e.g. SHA-256 of the 32 random bytes; a slow
  hash is not needed for high-entropy values) in a `token_hash` column and look
  requests up with `WHERE token_hash = $1` on the hash of the presented token —
  still a single indexed equality, so the lookup stays fast. Attack without it:
  anyone who reads the `api_tokens` table (a database backup, a read replica, a
  SQL injection elsewhere, an operator) gets bearer credentials usable as is
  against the API for every user, unlike `users.password_hash` which is bcrypt
  (CWE-312, CWE-256). Update tasks 1.1 and 1.2 accordingly, and drop
  "none, the lookup must be fast" from the alternatives, since hashing keeps it
  fast.
history:
  - status: open
    by: security
```

#### R-security-2 The Authorization header, hence every bearer token, is written to the logs

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Never log `Authorization` (nor `Cookie` / `Set-Cookie`): redact it, or log a
  non-secret identifier instead — the token's row id or a short prefix /
  fingerprint of its hash — so support can still tell which token was sent.
  Attack without it: anyone with read access to the logs (support staff, a log
  aggregation service, a leaked log archive) collects working tokens and calls
  the API as their owners (CWE-532); it also defeats R-security-1, since the
  clear tokens end up in the logs even once the table is hashed. Task 1.3
  ("Request logging in the middleware, verified by `cargo test
  logs_requests`") should assert that the header is absent from the log line.
history:
  - status: open
    by: security
```

#### R-security-3 A leaked token cannot be revoked, so the change ships without a way to cut off a compromised integration

```yaml
severity: medium
confidence: high
location: "design.md#Goals / Non-Goals"
evidence: |
  **Non-Goals:** OAuth, scopes, revocation (a later change).
recommendation: >-
  Either bring a minimal revocation (delete a token by id, owner-checked) into
  this change, or state in the proposal that it must not be released before the
  revocation change, and record that dependency. Attack without it: a token
  leaked from an integration (the very threat the proposal names: "a leaked
  integration exposes the account") stays valid indefinitely; the user cannot
  invalidate it, and unlike a password it is not changed by a password reset,
  so the attacker keeps full account access through the API. The Risks section
  only covers a lost token, not a leaked one.
history:
  - status: open
    by: security
```
