# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Addresses with non-ASCII characters are logged in full or partly unmasked

```yaml
severity: high
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >
  The pattern accepts only ASCII in the local part and the domain, so an
  address the proposal promises to mask reaches the logs kept 90 days by the
  hosting provider (GDPR Art. 5(1)(c), Art. 32). `logInfo("signup", { email:
  "josé@exemple.fr" })` writes `josé@exemple.fr` unchanged (the character
  before `@` is outside the class, so nothing matches); `ada@münchen.de` is
  not matched either; `zoë.martin@example.org` becomes
  `zoë.***@example.org`, leaking the first name. Match on Unicode letters
  (`\p{L}\p{N}` with the `u` flag) in both the local part and the domain, or
  mask any token containing `@`, and add tests for a non-ASCII local part
  and an IDN domain.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 The masked form keeps the domain and first letter

```yaml
severity: low
confidence: low
location: "src/logger.ts:5-7"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
recommendation: >
  For a personal or small-company domain (`a***@lovelace.family`), the
  first letter plus the domain can still identify the customer, so the
  log line stays personal data (GDPR Art. 4(1), Art. 5(1)(c)). Whether such
  domains appear in the logs could not be verified; if they do, replace the
  whole address with a fixed token or a keyed hash when correlation is
  needed.
history:
  - status: open
    by: privacy
```
