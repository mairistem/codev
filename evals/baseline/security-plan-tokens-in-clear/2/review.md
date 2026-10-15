# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### security

#### R-security-1 The Authorization header, bearer tokens included, is written to the logs

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Never log credential-bearing headers (CWE-532). Redact `Authorization`,
  `Cookie` and `Set-Cookie` in the middleware's request log (e.g. log
  `Bearer <redacted>` or a short non-reversible fingerprint such as the
  first bytes of the token's SHA-256 if support needs to tell tokens apart).
  As designed, anyone who can read the `info` logs — support, a log
  aggregation service, a leaked log archive — copies a live token and calls
  the API as its owner; the same line also captures the passwords of
  integrations still using Basic auth. Update task 1.3 so `logs_requests`
  asserts the header value is absent from the log output.
history:
  - status: open
    by: security
```

#### R-security-2 API tokens are stored in clear in the database

```yaml
severity: high
confidence: high
location: "design.md#Decision: Tokens are stored as given"
evidence: |
  The `api_tokens` table holds `user_id`, `token` and `created_at`; the
  middleware looks a request's token up with `WHERE token = $1`. Storing the
  token as given keeps the lookup a simple indexed equality.
recommendation: >-
  Store only a hash of the token (CWE-312 / CWE-256): since tokens are
  32 random bytes, an unsalted SHA-256 is enough, and the lookup stays a
  simple indexed equality (`WHERE token_hash = $1` with the hash of the
  presented token), so the stated reason for storing it as given does not
  hold. As designed, anyone who reads the `api_tokens` table — a database
  backup, a read replica, an SQL injection elsewhere, an operator — gets
  every user's working credential, while the same database already protects
  passwords with bcrypt. Hashing also removes the comparison of the raw
  secret inside the database. Update the table in task 1.1 and the spec's
  "shown once" requirement then holds by construction.
history:
  - status: open
    by: security
```

#### R-security-3 A leaked token cannot be revoked and never expires

```yaml
severity: medium
confidence: high
location: "design.md#Goals / Non-Goals"
evidence: |
  **Non-Goals:** OAuth, scopes, revocation (a later change).
recommendation: >-
  The proposal's own motivation is that "a leaked integration exposes the
  account"; with no revocation and no expiry, a leaked token gives
  permanent, full-account API access and the user has no way to stop it
  (the Risks section only offers creating another token, which leaves the
  old one valid). Either bring a minimal revocation (delete a token by id,
  owner-checked) into this change, or give tokens an expiry and a
  documented operator procedure until the revocation change ships —
  and make the deferral an explicit risk for a human to accept.
history:
  - status: open
    by: security
```

#### R-security-4 The plan does not say whether a bearer token may create new tokens

```yaml
severity: medium
confidence: low
location: "specs/api-tokens/spec.md#Requirement: A user creates an API token"
evidence: |
  The service SHALL let a signed-in user create an API token, shown once, and SHALL accept it in the `Authorization: Bearer` header of later requests.
recommendation: >-
  State that token creation requires an interactive (password) session and
  refuse it for requests authenticated by an API token. If the middleware
  treats a bearer-authenticated request as "signed in", a stolen token can
  mint fresh tokens and keep access after the original is replaced — which
  matters all the more with no revocation (R-security-3). Add a scenario
  for it and a test in task 1.2.
history:
  - status: open
    by: security
```

### skeptic

Disabled by configuration.
