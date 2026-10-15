# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Addresses with non-ASCII or special local-part characters are logged in full or partly
```yaml
severity: medium
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The local-part class is ASCII-only and omits characters RFC 5321/6531 allow (accented letters, ' & ! # = ? etc.).
  `logInfo("from josé@example.org")` cannot match at all (é sits right before @), so the full address reaches
  the 90-day hosting logs, against the proposal's "masks every e-mail address" and GDPR Art. 5(1)(c)/25;
  `john&jane@example.org` matches only from `jane` and logs `john&j***@example.org`, leaking part of the
  local part. Match the local part as any run of non-space, non-@, non-delimiter characters (Unicode-aware,
  `u` flag) up to the @, and likewise accept non-ASCII domains (`ada@exämple.fr` is also left unmasked);
  add test cases for these inputs.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 URL-encoded addresses escape masking
```yaml
severity: medium
confidence: low
location: "src/logger.ts:6-8"
evidence: |
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
  }
recommendation: >-
  The pattern requires a literal `@`, so a logged URL or query string such as
  `GET /reset?email=ada%40example.org` is written unmasked. No caller logs URLs yet in this change;
  if request paths are to be logged through `logInfo`, also match `%40` (case-insensitive) or decode before masking.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Field keys are not masked
```yaml
severity: low
confidence: low
location: "src/logger.ts:12-15"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
recommendation: >-
  Only values go through `maskEmails`; a caller that keys fields by address (for example counts per user,
  `{ "ada@example.org": 3 }`) writes the address in full. Mask the key as well: `safe[maskEmails(key)] = …`.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Kept first letter and full domain may re-identify on small domains
```yaml
severity: low
confidence: low
location: "src/logger.ts:5-7"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
recommendation: >-
  `a***@lovelace-family.org` or a small company domain still points to one person, so the logs keep
  identifiable data (GDPR Art. 4(1)). Whether that is acceptable depends on the customers' domains, which
  the reviewer could not see; if not, mask the domain too or replace the address with a keyed hash.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
