# Review record: document-token-rotation

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Revocation guarantee documented without enforcing code in the repository

```yaml
severity: medium
confidence: low
location: "README.md:19-20"
evidence: |
  3. Revoke the old token from the same page. Requests that still carry it
     are refused with `401 Unauthorized`.
recommendation: >-
  Confirm that revoking a token is enforced server-side on every request
  (the token is checked against its revoked state, not only hidden in the
  Settings page), and point the README at that behaviour. The repository
  holds no token issuance, revocation or 401 path (src/api.rs, src/db.rs,
  src/lib.rs), so the claim cannot be verified here. If revocation were not
  enforced, an integrator following step 3 after an exposure would believe
  the leaked token dead while an attacker holding it keeps full API access.
  If the token logic lives in another service, cite it; otherwise drop the
  guarantee until it exists.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
