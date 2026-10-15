# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Percent-encoded e-mail addresses reach the logs in full

```yaml
severity: high
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >
  The pattern only matches a literal `@`. A message or field holding a URL or
  query string — `logInfo("GET /reset?email=ada.lovelace%40example.org")` — has
  no `@`, so `maskEmails` returns it unchanged and the full address is written
  to the hosting provider's logs, kept 90 days (GDPR Art. 5(1)(c) and Art. 32).
  Also match the encoded separator (`%40`, case-insensitive) — or decode before
  matching — and add a test with an encoded address.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Addresses with characters outside the ASCII class escape masking

```yaml
severity: medium
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >
  The local part and the domain accept only `[A-Za-z0-9._%+-]` and
  `[A-Za-z0-9.-]`. `andré@example.org` (accented local part) and
  `ada@exämple.fr` (internationalised domain) do not match at all and are
  logged in full; `jean.o'neil@example.org` matches only from `neil`, giving
  `jean.o'n***@example.org`, which keeps most of the person's name. The
  proposal promises to mask "every e-mail address". Widen the classes to the
  characters RFC 5322/6531 allow (for instance `[^\s@<>()"]` for the local
  part and `[\p{L}\p{N}.-]` with the `u` flag for the domain), and cover these
  cases in `src/logger.test.ts`.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Field keys are written to the log without masking

```yaml
severity: medium
confidence: low
location: "src/logger.ts:12-15"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
  }
  console.log(JSON.stringify({ level: "info", message: maskEmails(message), ...safe }));
recommendation: >
  Only values go through `maskEmails`; keys are copied as is. A caller logging
  a map keyed by address — `logInfo("quota", { "ada@example.org": 3 })` —
  writes the full address to the logs. Whether any caller does depends on code
  not in this change. Mask the key too (`safe[maskEmails(key)] = …`).
history:
  - status: open
    by: privacy
```

#### R-privacy-4 The kept domain and first letter may re-identify a customer

```yaml
severity: low
confidence: low
location: "src/logger.ts:7"
evidence: |
  return text.replace(EMAIL, "$1***@$2");
recommendation: >
  `a***@example.org` keeps the whole domain and the first letter. For a
  customer on a personal or small company domain (`a***@lovelace-family.fr`),
  the masked value still points to one person, so the log stays personal data
  (GDPR Art. 4(1), Art. 5(1)(c)). Depends on the customer base, which the
  review cannot see. If the domain is not needed for diagnosis, replace the
  address with a fixed token or a keyed hash.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
