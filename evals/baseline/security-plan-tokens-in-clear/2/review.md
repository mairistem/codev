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
  for a high-entropy secret; no bcrypt needed) in a `token_hash` column, hash
  the presented token in the middleware and look it up with
  `WHERE token_hash = $1` — still a simple indexed equality, so the stated
  reason for storing it as given does not hold. Anyone who reads the table
  (a SQL injection elsewhere, a leaked backup, a replica, a support
  dump) otherwise gets bearer credentials for every account, usable at once
  and, since revocation is deferred, impossible to cancel (CWE-312,
  CWE-256). Update task 1.1 and 1.2 accordingly.
history:
  - status: open
    by: security
```

#### R-security-2 The Authorization header is written to the logs

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Redact credential headers before logging: drop or mask `Authorization`
  and `Cookie` (the middleware already authenticates by the `session=`
  cookie, so every browser session would leak too). Anyone with access to
  the logs — support, the log pipeline, a log aggregator — can copy a token
  or session and call the API as that user, with no way to revoke it in
  this change (CWE-532). To identify which token an integration sent, log
  a non-secret token id or a short prefix of its hash instead. Task 1.3's
  test `logs_requests` should assert the header is absent from the log.
history:
  - status: open
    by: security
```

### skeptic

Disabled by configuration.
