# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

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
  Store only a hash of the token (SHA-256 is enough for a 32-byte random
  value, no salt or slow hash needed) and look it up with `WHERE token_hash = $1`
  on the hash of the presented token; the lookup stays an indexed equality.
  Anyone who reads the table (backup, replica, SQL injection elsewhere, DBA)
  otherwise holds working credentials for every user (CWE-312, CWE-256).
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
  Redact `Authorization` (and `Cookie`) before logging; to let support identify
  a token, log a non-secret identifier such as the token's id or a short prefix
  of its hash. Anyone with log access can otherwise replay any integration's
  token and act as its user (CWE-532). Task 1.3 `logs_requests` should assert
  the header is redacted.
history:
  - status: open
    by: security
```

#### R-security-3 Revocation is announced but neither specified nor planned

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - Users create and revoke personal API tokens.
recommendation: >-
  Add a requirement and scenario in `specs/api-tokens/spec.md` that a user
  revokes one of their own tokens and a revoked token gets 401, with the
  ownership check on the revoke operation (CWE-862), and a task in `tasks.md`
  to implement and test it. Without it a leaked token stays valid forever,
  while the design's only risk mitigation relies on revoking it.
history:
  - status: open
    by: security
```

### skeptic

Disabled by configuration.
