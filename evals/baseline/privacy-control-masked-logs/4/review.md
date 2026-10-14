# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Addresses with a non-ASCII character escape the mask and are logged in full

```yaml
severity: high
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The pattern only matches when the character right before `@` and every
  character of the domain are ASCII. `logInfo("login failed for josé@exemple.fr")`
  or `logInfo("signup", { email: "ada@exämple.de" })` match nothing, so the
  full address reaches the hosting provider's logs, kept 90 days — the very
  leak the proposal removes (GDPR Art. 5(1)(c), Art. 25, Art. 32). Match any
  non-space, non-`@` run around the `@` (e.g. `[^\s@]+@[^\s@]+`, with the
  `u` flag) rather than an ASCII allow-list, and add tests with a non-ASCII
  local part and an internationalized domain.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 URL-encoded addresses (`%40`) are logged in full

```yaml
severity: medium
confidence: high
location: "src/logger.ts:6-8"
evidence: |
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
  }
recommendation: >-
  `maskEmails` only recognises a literal `@`. A logged URL or query string
  such as `logInfo("GET /reset?email=ada%40example.org")` keeps the address
  in clear, since `%40` is not `@`; e-mail addresses in URLs are a common
  path into logs. Also match the percent-encoded form (`%40`, case
  insensitive), or decode before masking, and cover it with a test.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 A one-letter local part is logged whole

```yaml
severity: medium
confidence: high
location: "src/logger.ts:5-7"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
recommendation: >-
  The mask keeps the first character and the whole domain, so for
  `a@example.org` the output `a***@example.org` still discloses the complete
  address — contrary to the logger's own promise that it "never writes a full
  e-mail address". Keep no character of the local part (e.g. `***@domain`),
  or at least none when the local part is a single character.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 The masked form still identifies people on personal domains

```yaml
severity: low
confidence: low
location: "src/logger.ts:7"
evidence: |
  return text.replace(EMAIL, "$1***@$2");
recommendation: >-
  First letter plus full domain stays personal data when the domain is small
  or personal (`j***@dupont-family.fr`), and it is still kept 90 days by the
  hosting provider. Whether that residual is acceptable depends on the
  customers' domains, which the review could not see: the team decides
  whether to also mask the domain or log an opaque identifier instead
  (GDPR Art. 5(1)(c), Art. 25).
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Field names are not masked

```yaml
severity: low
confidence: low
location: "src/logger.ts:12-13"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
recommendation: >-
  Only field values go through `maskEmails`; a caller that keys fields by
  address (`logInfo("quota", { [user.email]: 3 })`) would log it in full.
  No caller exists yet, so this depends on future use: mask the keys too,
  which costs one call.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
