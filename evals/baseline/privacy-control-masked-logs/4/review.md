# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Addresses with a non-ASCII character are logged in full

```yaml
severity: high
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The local part and the domain only accept ASCII letters, so an
  internationalized address does not match at all and passes through
  unmasked: `logInfo("signup", { user: "josé@example.fr" })` writes
  `josé@example.fr` (the character before `@` is `é`, outside the class),
  and `ada@exämple.de` is likewise left whole. The customer's e-mail
  address then reaches the hosting provider's logs, kept 90 days — the very
  leak the proposal removes (GDPR Art. 5(1)(c), Art. 25). Match any
  non-space, non-`@` characters around the `@` (e.g. Unicode-aware
  `[^\s@]+@[^\s@]+\.[^\s@]{2,}` with the `u` flag), and add a test with a
  non-ASCII local part and domain.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Percent-encoded addresses in URLs are not masked

```yaml
severity: high
confidence: low
location: "src/logger.ts:6-8"
evidence: |
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
  }
recommendation: >-
  The pattern requires a literal `@`, so an address carried in a URL or
  query string, such as `/reset?email=ada.lovelace%40example.org`, is
  logged in full when a caller logs a request path or URL. No caller exists
  in this diff, hence the low confidence. Also match the encoded `%40`
  form (case-insensitive), or decode before masking, and cover it with a
  test.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Field names are written to the log without masking

```yaml
severity: medium
confidence: low
location: "src/logger.ts:12-13"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
recommendation: >-
  Only values go through `maskEmails`; the keys are copied as they are. A
  caller that builds fields keyed by an address (e.g. a per-user counter
  `{ [email]: count }`) writes the full address to the log. It depends on
  callers not in this diff. Mask the keys too, or document that keys must
  be constant identifiers.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 The masked form keeps the domain and first letter

```yaml
severity: low
confidence: low
location: "src/logger.ts:5"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
recommendation: >-
  `a***@example.org` is pseudonymised, not anonymous: for a customer on a
  personal or small-company domain, the first letter plus the domain is
  enough to re-identify them in the 90-day logs. Whether that matters
  depends on the customer base, which the review could not see. If the
  domain is not needed for diagnosis, mask it too (e.g. `a***@***`) or log
  an opaque user id instead.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
