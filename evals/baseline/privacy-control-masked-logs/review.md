# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

### privacy

#### R-privacy-1 Addresses with non-ASCII characters are logged in full

```yaml
severity: medium
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The character classes are ASCII-only, so the character right before `@` must
  be ASCII and the domain must be ASCII. `logInfo("signup", { email: "zoë@example.org" })`
  or an internationalised domain such as `jean@exämple.fr` matches nothing and the
  full address reaches the hosting provider's logs, kept 90 days (GDPR Art. 5(1)(c),
  Art. 25), although the header promises the logger "never writes a full e-mail
  address". Match on the `@` with Unicode-aware classes (e.g. `[^\s@]+@[^\s@]+\.[^\s@]+`
  with the `u` flag) and add a test with a non-ASCII local part and domain.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Percent-encoded addresses escape the mask

```yaml
severity: medium
confidence: low
location: "src/logger.ts:6-8"
evidence: |
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
  }
recommendation: >-
  The pattern requires a literal `@`. A logged URL or query string such as
  `/reset?email=ada.lovelace%40example.org` is written unchanged, so the address
  lands in the logs in full. No caller logs URLs yet, hence the low confidence.
  Also match `%40` (case-insensitive) as the separator, or decode before masking,
  and cover it with a test.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Masked form keeps the first letter and the full domain

```yaml
severity: low
confidence: low
location: "src/logger.ts:5"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
recommendation: >-
  For a personal or small-company domain (`j***@janedoe.com`), the first letter
  plus the domain can be enough to re-identify the person, so the logs still hold
  personal data in pseudonymised form. Whether that is acceptable depends on the
  domains in real traffic, which the review could not see. If the domain is not
  needed for debugging, mask it too, or log a keyed hash of the address instead.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Field keys are written unmasked

```yaml
severity: low
confidence: low
location: "src/logger.ts:12-13"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
recommendation: >-
  Only the values go through `maskEmails`. A caller that builds `fields` from a
  map keyed by address (e.g. `{ "ada@example.org": 3 }`) writes the address in
  full. No such caller exists yet. Mask the keys as well: `safe[maskEmails(key)] = …`.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
