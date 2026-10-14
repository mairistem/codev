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
  Store only a hash of the token (SHA-256 of the 32 random bytes is enough
  for a high-entropy secret; no bcrypt needed) in a `token_hash` column with
  a unique index, and look the request's token up with
  `WHERE token_hash = $1` after hashing it — the lookup stays a simple indexed
  equality, so the stated reason for storing the token as given does not
  hold. Scenario that fails today: anyone who reads the `api_tokens` table —
  a database backup, a read replica, an SQL injection elsewhere in the
  service, a support operator with read access — gets every user's live
  bearer credential and can call the API as any user (CWE-312 / CWE-256).
  Unlike `users.password_hash`, which the design already keeps hashed with
  bcrypt, these tokens are directly usable. Update the "Alternatives
  considered" line, which wrongly states there is none, and task 1.1/1.2 to
  test that the stored value is not the token.
history:
  - status: open
    by: security
```

#### R-security-2 The middleware logs the Authorization header, writing every token to the logs

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Never log the `Authorization` header (nor `Cookie`): redact it to
  `Bearer ***`, or log a non-secret identifier instead — the token's row id
  or a short prefix of its hash — so support can still tell which token an
  integration sent. Scenario that fails as designed: every authenticated
  request writes its live bearer token at `info` level; anyone with access to
  the logs or the log pipeline (support, log aggregation vendor, a leaked log
  archive) replays it and acts as that user (CWE-532). Because every request
  goes through this one middleware, it also logs the credentials of every
  other authentication path that uses that header. Task 1.3
  (`cargo test logs_requests`) should assert that the header value is absent
  from the log output.
history:
  - status: open
    by: security
```

#### R-security-3 A leaked token cannot be invalidated by any means

```yaml
severity: medium
confidence: high
location: "design.md#Goals / Non-Goals"
evidence: |
  **Non-Goals:** OAuth, scopes, revocation (a later change).
recommendation: >-
  Revocation is deferred, and the design gives tokens no expiry and does not
  tie them to the password, so this change ships credentials that can never
  be turned off. Scenario: an integration's token leaks (the exact case the
  proposal's Why names — "a leaked integration exposes the account"); the
  user changes their password, which does not touch `api_tokens`, and the
  attacker keeps full API access indefinitely. Either bring a minimal
  revocation (delete a token by id, owned by the caller) into this change,
  or give tokens an expiry (`expires_at`, checked in the lookup), and in any
  case state in the Risks section what a user does when a token leaks. The
  current risk line only covers a lost token, not a leaked one.
history:
  - status: open
    by: security
```
