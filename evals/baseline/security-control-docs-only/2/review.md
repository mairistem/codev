# Review record: document-token-rotation

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Revocation guarantee documented without code to back it

```yaml
severity: low
confidence: low
location: "README.md:19-20"
evidence: |
  3. Revoke the old token from the same page. Requests that still carry it
     are refused with `401 Unauthorized`.
recommendation: >-
  Confirm against the service that serves Settings → API tokens that a
  revoked token is refused immediately on every endpoint (no cache or
  grace period); if revocation is delayed, say so in this step, since an
  integrator rotating an exposed token relies on the old one being dead.
  Nothing in this repository (src/api.rs, src/db.rs) handles tokens, so
  the claim could not be checked here.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
