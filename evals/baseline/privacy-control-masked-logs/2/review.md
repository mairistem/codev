# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Field keys are written to the log unmasked

```yaml
severity: medium
confidence: high
location: "src/logger.ts:12-15"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
  }
  console.log(JSON.stringify({ level: "info", message: maskEmails(message), ...safe }));
recommendation: >-
  Mask the keys too (`safe[maskEmails(key)] = …`), so that a caller logging a
  map keyed by address — `logInfo("retries", { "ada.lovelace@example.org": 3 })`
  — does not write `{"ada.lovelace@example.org":3}` in full to the logs the
  hosting provider keeps 90 days (GDPR Art. 5(1)(c), 25). The proposal promises
  masking of "every e-mail address in the message and the fields"; add a
  `logInfo` test covering a key and a value.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Addresses with non-ASCII characters escape the mask entirely

```yaml
severity: medium
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The character classes are ASCII-only and the local part must run unbroken up
  to the `@`, so `josé@example.org` or `ada@exämple.org` (valid internationalised
  addresses, RFC 6531) match nothing and are logged in full — the proposal's
  "every e-mail address" is not met. Match the local part and domain on any
  non-space, non-`@` characters (e.g. `/([^\s@])[^\s@]*@([^\s@]+\.[^\s@]{2,})/gu`)
  and add a test with an accented address.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Percent-encoded addresses in logged URLs are not masked

```yaml
severity: low
confidence: low
location: "src/logger.ts:6-8"
evidence: |
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
  }
recommendation: >-
  A logged URL such as `/reset?email=ada%40example.org` carries the address
  without a literal `@`, so it passes through unmasked. Whether callers log
  such URLs could not be verified; if they do, also match `%40` as the separator.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 The mask keeps the first letter and the full domain

```yaml
severity: low
confidence: low
location: "src/logger.ts:5-7"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
recommendation: >-
  For a personal or small custom domain (`a***@lovelace.family`) the kept
  initial and domain can re-identify the customer. Confirm with the team that
  the domain is needed for the logs' purpose; otherwise mask it as well, or
  replace the address with a keyed hash (data minimisation, GDPR Art. 5(1)(c)).
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
