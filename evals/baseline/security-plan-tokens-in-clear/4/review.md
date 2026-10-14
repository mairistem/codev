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
recommendation: >
  Store only a hash of the token (e.g. `token_hash`, SHA-256 of the 32 random
  bytes — a fast hash is enough for a high-entropy secret, unlike passwords)
  and look it up with `WHERE token_hash = $1`, bound as a parameter: the
  lookup stays an indexed equality, so the stated reason for storing the
  token as given does not hold. Show the token once at creation (as the spec
  already requires) and never persist it. Update task 1.1 and its test
  accordingly. CWE-312 / CWE-522.
history:
  - status: open
    by: security
```

Attack: anyone who reads the `api_tokens` table — a leaked backup or dump, a
read replica, a support or analytics account with database access, or a SQL
injection anywhere in the service — gets every live bearer token and can call
the API as each of their owners with `Authorization: Bearer <token>`. Since
revocation is out of scope, those tokens stay usable indefinitely.

#### R-security-2 The Authorization header, and so every bearer token, is written to the logs

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >
  Never log the `Authorization` header (nor `Cookie` or any credential
  header): redact it to `Bearer <redacted>`, or log a non-secret token
  identifier (a token id, or a short prefix of the token hash) so support can
  still tell which token an integration used. Change task 1.3 so that
  `logs_requests` asserts the header value is absent from the log output.
  CWE-532.
history:
  - status: open
    by: security
```

Attack: every request carries its token in `Authorization`, so every token in
use ends up at `info` level in the logs; anyone with access to the logs or the
log pipeline (support staff, a log aggregation service, a leaked log archive)
replays the header and acts as the token's owner. The same middleware also
handles password-based requests, so any `Authorization: Basic` credential
would be logged too, exposing the account passwords this change aims to
protect. Task 1.3 (`Request logging in the middleware, verified by cargo test
logs_requests`) implements this decision as written.

#### R-security-3 A leaked token can never be invalidated

```yaml
severity: medium
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  - [A user loses a token] → they create another one; revoking the old one comes with the later revocation change.
recommendation: >
  Either ship a minimal revocation (delete a token by id, owned by the
  caller, checked on the server) or an expiry (`expires_at` checked by the
  middleware) in this change, or state explicitly in the design that tokens
  must not be released to users before the revocation change ships. The
  proposal's own motivation is that a leaked credential exposes the account;
  without any way to invalidate a token, a leaked token exposes it
  permanently.
history:
  - status: open
    by: security
```

Attack: an integration's token leaks (a committed config file, a compromised
integration host); the attacker calls the API as the user, and neither the user
nor an operator has any supported way to stop it — creating a new token, the
only mitigation the plan offers, leaves the leaked one valid.
