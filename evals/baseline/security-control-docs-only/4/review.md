# Review record: document-token-rotation

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Exposed token stays valid until the new one is deployed

```yaml
severity: low
confidence: high
location: "README.md:13-20"
evidence: |
  API tokens do not expire. Rotate a token when a person who knew it leaves
  the team, or when it may have been exposed:
recommendation: >-
  The same create → deploy → revoke order is given for a planned rotation and
  for a suspected exposure. For an exposure, whoever holds the leaked token
  keeps full API access for the whole deployment window of step 2, and
  indefinitely if the deployment stalls, since tokens do not expire. Add a
  sentence for the exposure case: revoke the old token first (accepting the
  integration's downtime), or deploy and revoke without delay.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
