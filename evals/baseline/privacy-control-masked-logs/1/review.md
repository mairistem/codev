# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Addresses with non-ASCII characters are logged in full

```yaml
severity: high
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The local-part and domain classes are ASCII-only, so any address with an
  accented or other non-ASCII character fails to match and is written to the
  logs unmasked: `logInfo("signup", { email: "josé@example.fr" })` prints
  `"email":"josé@example.fr"`, and `ada@exämple.org` likewise passes through.
  Local parts with other legal characters leak a prefix (`o'brien@example.com`
  becomes `o'b***@example.com`). These customers' addresses then reach the
  hosting provider's logs, kept 90 days (GDPR Art. 5(1)(c) and (f), Art. 32),
  contrary to the release note. Match on `\S+@\S+`-style boundaries (or
  Unicode classes with the `u` flag, e.g. `[^\s@]+@[^\s@]+\.[^\s@]+`) and add
  tests for a non-ASCII local part, a non-ASCII domain and an apostrophe.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Masked form keeps the first letter and the full domain

```yaml
severity: medium
confidence: low
location: "src/logger.ts:5-7"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
recommendation: >-
  The logs keep the first letter of the local part and the whole domain. For
  a personal or small-company domain (`a***@lovelace-family.org`), that is
  enough to single out the customer, so the log line stays personal data
  (GDPR Art. 4(1), Art. 5(1)(c)). Depends on the customers' domains, which
  the reviewer could not see. Ask the team whether the domain is needed for
  diagnosis; if not, replace the whole address with a fixed token or a keyed
  hash.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 URL-encoded addresses are not masked

```yaml
severity: medium
confidence: low
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The pattern requires a literal `@`. A logged URL or query string such as
  `/reset?email=ada%40example.org` keeps the address in full in the logs
  (GDPR Art. 5(1)(f)). Depends on callers logging URLs, none of which the
  change shows yet. Also match `%40` (case-insensitive) as the separator, and
  test it.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
