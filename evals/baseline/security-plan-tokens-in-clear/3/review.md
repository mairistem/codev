# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

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
  Store only a hash of the token (SHA-256 of the 32 random bytes is enough
  for high-entropy tokens; no bcrypt needed) in a `token_hash` column, hash
  the presented bearer token in the middleware and look it up with
  `WHERE token_hash = $1` — still a single indexed equality, so the stated
  reason for storing it as given does not hold. Anyone who reads the
  `api_tokens` table (a database backup, a replica, a read-only SQL access,
  an injection elsewhere) otherwise obtains every live token and can call
  the API as every user who created one; with revocation deferred, those
  tokens stay valid indefinitely (CWE-312, CWE-256). Update tasks 1.1 and
  1.2 accordingly.
history:
  - status: open
    by: security
```

#### R-security-2 The Authorization header, and so every bearer token, is written to the logs

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Redact the `Authorization` header (and `Cookie`) before logging; if
  support needs to identify the token, log the token's id or a short
  fingerprint of its hash, never the token itself. As designed, every
  request with `Authorization: Bearer <token>` puts a working credential in
  the logs at `info`, so anyone with access to the logs or the log
  pipeline (support staff, log aggregator, shipped log files) can replay
  it and act as the token's owner, with no way to revoke it (CWE-532).
  Adjust task 1.3 so `logs_requests` asserts the header is redacted.
history:
  - status: open
    by: security
```

#### R-security-3 A leaked token cannot be invalidated: revocation is deferred and tokens never expire

```yaml
severity: medium
confidence: high
location: "design.md#Goals / Non-Goals"
evidence: |
  **Non-Goals:** OAuth, scopes, revocation (a later change).
recommendation: >-
  Ship at least a minimal way to invalidate a token in this change — a
  delete of the user's token row, or an expiry date checked by the
  middleware — or state the risk explicitly and get a human to accept it.
  The proposal's own motivation is that "a leaked integration exposes the
  account"; today the user can change the password to cut a leaked
  integration off, but with this design a token leaked from an
  integration's configuration keeps full API access to the account for
  ever, and the Risks section only answers a lost token, not a leaked one.
history:
  - status: open
    by: security
```

### skeptic

Disabled by configuration.
