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
recommendation: Check the README's claims (tokens never expire, a revoked token is refused with 401 at once, the token is shown once) against the service that issues and checks tokens. This repository has no token, revocation or authentication code (src/api.rs, src/db.rs, src/lib.rs), so the claims could not be verified here. If revocation is cached or delayed, say so in step 3, so that an integrator rotating an exposed token does not take it to be dead while it is still accepted.
history:
  - status: open
    by: security
```
