# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Addresses with non-ASCII or percent-encoded characters reach the logs unmasked

```yaml
severity: high
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The pattern requires the character right before `@` to be ASCII and the
  domain to be ASCII, and needs a literal `@`. `logInfo("signup", { email:
  "josé@example.fr" })`, `"ada@exämple.de"` or a logged URL holding
  `ada%40example.org` match nothing and are written in full to the logs the
  hosting provider keeps 90 days — the very leak the change promises to stop
  (GDPR Art. 5(1)(c) and 32). Match any non-space, non-`@` run around `@`
  (e.g. `/([^\s@<>"'])[^\s@<>"']*(?:@|%40)([^\s@<>"']+\.[^\s@<>"'.]{2,})/giu`)
  and add test cases for a Unicode local part, an IDN domain and a
  percent-encoded `@`.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 The masked form keeps the first letter and the full domain

```yaml
severity: low
confidence: low
location: "src/logger.ts:5-7"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
recommendation: >-
  `a***@example.org` is still personal data when the domain is a small
  company or a personal domain: with the 90-day retention and the other
  fields of the log line, the customer can be re-identified (GDPR Art. 5(1)(c),
  Art. 25). Whether this matters depends on the customers' domains, which the
  reviewer could not see; if no log reader needs the domain, replace the
  whole address with a fixed token or a keyed hash, and ask the team whether
  the first letter and domain have a stated purpose.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Field names are logged without masking

```yaml
severity: low
confidence: high
location: "src/logger.ts:12-15"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
  }
recommendation: >-
  Only the values pass through `maskEmails`; a field map keyed by address,
  such as `logInfo("quota", { [user.email]: 3 })`, writes the full address as
  a JSON key. Mask the key too (`safe[maskEmails(key)] = …`) so the module
  keeps its promise of never writing a full e-mail address.
history:
  - status: open
    by: privacy
```
