# Review record: document-token-rotation

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Exposed token stays valid until the new one is deployed

```yaml
severity: low
confidence: high
location: "README.md:18-20"
evidence: |
  2. Deploy the integration with the new token.
  3. Revoke the old token from the same page. Requests that still carry it
recommendation: Split the procedure by cause. Planned rotation (a person leaves) keeps create → deploy → revoke for zero downtime; a token that may have been exposed is revoked first, accepting the downtime until the new token is deployed, since tokens never expire and whoever holds the leaked token keeps full access for the whole deploy window if the documented order is followed.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
