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
recommendation: >-
  Keep the create–deploy–revoke order for planned rotation (departure), but
  add that a token known to be exposed should be revoked first, accepting the
  integration's downtime until the new token is deployed; otherwise whoever
  holds the leaked token keeps full API access for the whole deploy window,
  and since tokens never expire nothing else bounds it.
history:
  - status: open
    by: security
```
