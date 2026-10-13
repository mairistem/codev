# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

### privacy

#### R-privacy-1 Masked address still identifies the person

```yaml
severity: medium
confidence: high
location: "src/logger.ts:6-8"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
recommendation: The first letter plus the full domain is pseudonymised, not anonymised data (GDPR Art. 5(1)(c), Art. 25) — for a personal or small-company domain (jane@janedoe.com → j***@janedoe.com) it still points to one person. Mask the domain too (e.g. `***@***`), or replace the address with a keyed hash / opaque user id when correlation is needed.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Encoded or non-ASCII addresses are not masked

```yaml
severity: medium
confidence: low
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: The pattern needs a literal `@` and ASCII characters, so a URL-encoded address (`ada%40example.org`, common in logged request URLs and query strings), an `&#64;`/`(at)` variant, or a Unicode local part / IDN domain (`zoë@exämple.fr`) reaches the logs in full (Art. 5(1)(f), Art. 32). Decode before matching or extend the pattern, and add tests for these forms; whether callers log such strings could not be verified, as nothing calls `logInfo` yet.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Field names are written without masking

```yaml
severity: low
confidence: low
location: "src/logger.ts:12-13"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
recommendation: Only values go through `maskEmails`; a field keyed by an address (e.g. a map of per-user counters spread into `fields`) writes the address in full. Mask the keys as well.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Field masking is not covered by a test

```yaml
severity: low
confidence: high
location: "src/logger.test.ts:4-11"
evidence: |
  describe("maskEmails", () => {
recommendation: The proposal promises masking "in the message and the fields", but only `maskEmails` is tested; `logInfo` — the function that actually writes to the logs — has no test. Add a test capturing `console.log` that checks a field value and the message are masked.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Addresses already in the 90-day logs are left in place

```yaml
severity: medium
confidence: high
location: "proposal.md#Why"
evidence: |
  Our logs are kept 90 days by the hosting provider and contain customers' e-mail addresses.
recommendation: The change stops new leaks but defines nothing for the addresses already logged (storage limitation, Art. 5(1)(e); erasure, Art. 17) — they stay at the hosting provider for up to 90 days, backups included, and an erasure request does not reach them. Decide with the team whether to purge or redact existing logs, or state that they expire naturally.
history:
  - status: open
    by: privacy
```

#### R-privacy-6 Hosting provider as log processor is not identified

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Our logs are kept 90 days by the hosting provider
recommendation: Question for the team — name the hosting provider receiving the logs, confirm the processing agreement (Art. 28) and whether logs leave the EEA (Chapter V). Not a code fix.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
