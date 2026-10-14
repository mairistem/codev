# Review record: document-token-rotation

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Exposed-token rotation keeps the leaked token valid until after redeploy

```yaml
severity: low
confidence: low
location: "README.md:18-19"
evidence: |
  2. Deploy the integration with the new token.
  3. Revoke the old token from the same page. Requests that still carry it
recommendation: >-
  Keep the create → deploy → revoke order for routine rotation, but add a
  line for a token known to be exposed: revoke it first (accepting the
  downtime) or as soon as the new token is created, so an attacker holding
  the leaked token loses access without waiting for the redeploy. The
  procedure otherwise handles the secret soundly (secret store, never in
  source code); the 401 behavior and the Settings page could not be checked
  against code in this repository.
history:
  - status: open
    by: security
```
