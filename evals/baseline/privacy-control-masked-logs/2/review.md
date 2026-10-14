# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Percent-encoded addresses reach the logs unmasked

```yaml
severity: medium
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The pattern requires a literal `@`, so `logInfo("GET /reset?email=ada%40example.org")`
  writes the full address to the logs kept 90 days by the hosting provider (GDPR Art. 5(1)(c),
  Art. 25). Request paths and query strings are the typical place this encoding appears.
  Also match `%40` (case-insensitive) as the separator, or decode URL-encoded text before
  masking, and add a test with an encoded address.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Addresses with a non-ASCII local part are not masked

```yaml
severity: medium
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The local-part class is ASCII-only: `用户@example.cn` has no matching character before
  `@` and is logged in full, and `élodie@example.fr` becomes `él***@example.fr`, leaking
  a character the mask meant to hide. Internationalized addresses (RFC 6531) are e-mail
  addresses the proposal promises to mask. Use a Unicode-aware class (`[\p{L}\p{N}._%+-]`
  with the `u` flag) for the local part and the domain, and add a test for it.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Field names are written to the log without masking

```yaml
severity: low
confidence: high
location: "src/logger.ts:12-13"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
recommendation: >-
  Only values are masked; a field keyed by an address, such as
  `logInfo("attempts", { [email]: count })`, logs the address in full. Mask the key too:
  `safe[maskEmails(key)] = …`.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 The kept domain can identify the person

```yaml
severity: low
confidence: low
location: "src/logger.ts:7"
evidence: |
  return text.replace(EMAIL, "$1***@$2");
recommendation: >-
  The mask keeps the first letter and the full domain. With a personal or small-company
  domain (`a***@lovelace.name`), the masked value still points at one person, so the logs
  keep personal data (GDPR Art. 4(1), Art. 5(1)(c)). Depends on the customers' domains,
  which the review could not see; if that matters, mask the domain too or log an opaque
  identifier instead.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
