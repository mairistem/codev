# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Addresses with a non-ASCII domain are logged in full

```yaml
severity: medium
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The domain class only accepts ASCII, so `logInfo("signup", { email: "ada@exämple.org" })`
  finds no match and writes the whole address to the logs kept 90 days by the hosting
  provider (GDPR Art. 5(1)(c), Art. 32), against the proposal's "masks every e-mail
  address". A non-ASCII local part leaks its prefix too (`jörg@example.de` becomes
  `jö` + `r***@example.de`). Match Unicode letters (`\p{L}`/`\p{N}` with the `u` flag)
  in the local part and the domain, and add a test for an internationalised address.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 URL-encoded addresses pass through unmasked

```yaml
severity: medium
confidence: high
location: "src/logger.ts:6-8"
evidence: |
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
  }
recommendation: >-
  The pattern requires a literal `@`, so a logged request path or query string such as
  `logInfo("GET /reset?email=ada%40example.org")` keeps the full address in the logs
  (GDPR Art. 5(1)(c)). Also match the percent-encoded separator (`%40`, case-insensitive)
  — or decode before masking — and add a test with an encoded address.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
