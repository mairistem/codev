# Review record: document-token-rotation

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Exposed token stays valid until the new one is deployed

```yaml
severity: low
confidence: high
location: "README.md:18-19"
evidence: |
  2. Deploy the integration with the new token.
  3. Revoke the old token from the same page. Requests that still carry it
recommendation: >-
  Keep the create-deploy-revoke order for planned rotation (a departing team
  member), but say that when a token may have been exposed it should be
  revoked at once, accepting the downtime, since whoever holds the leaked
  token keeps full API access until step 3 and tokens never expire on their
  own. Documentation advice, so low (CWE-324 is not certain; no CWE cited).
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
