# Review record: document-token-rotation

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Exposed-token rotation keeps the leaked token valid until redeploy

```yaml
severity: low
confidence: high
location: "README.md:18-20"
evidence: |
  2. Deploy the integration with the new token.
  3. Revoke the old token from the same page. Requests that still carry it
recommendation: >-
  Keep the create-deploy-revoke order for planned rotation (a person leaving),
  but add that when a token may have been exposed it should be revoked first,
  accepting the integration's downtime: as written, whoever holds a leaked token
  keeps calling the API, and since tokens never expire, for as long as step 2
  takes. Documentation advice, hence low.
history:
  - status: open
    by: security
```
