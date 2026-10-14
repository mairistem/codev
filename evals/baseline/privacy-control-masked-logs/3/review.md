# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Addresses with non-ASCII characters are logged unmasked

```yaml
severity: medium
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The local part and the domain only accept ASCII, while the proposal promises to mask
  "every e-mail address". `logInfo("signup", { email: "josé@exemple.fr" })` writes
  `josé@exemple.fr` in full to the logs kept 90 days by the hosting provider (GDPR Art. 5(1)(c),
  Art. 32), since the character before `@` is outside the class and nothing matches; an
  internationalised domain (`ada@exämple.fr`) is missed the same way, and
  `andré.dupont@example.fr` only becomes `andré.***@example.fr`, keeping the first name.
  Match the address on its structure instead — e.g. `/([^\s@<>"'(),;:])[^\s@<>"'(),;:]*@([^\s@<>"'(),;:]+\.[^\s@<>"'(),;:]{2,})/gu` —
  and add tests for an accented local part and an IDN domain.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 URL-encoded addresses escape the mask

```yaml
severity: medium
confidence: low
location: "src/logger.ts:7"
evidence: |
  return text.replace(EMAIL, "$1***@$2");
recommendation: >-
  Only a literal `@` is recognised, so a logged URL or query string such as
  `/reset?email=ada.lovelace%40example.org` reaches the logs unmasked. No caller logs URLs yet
  (the logger has no caller in the diff), hence the low confidence; if request paths are to be
  logged through `logInfo`, also match `%40` (case-insensitive) or decode before masking, and
  test it.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 The kept first letter and full domain may still identify the person

```yaml
severity: low
confidence: low
location: "src/logger.ts:5"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
recommendation: >-
  For a personal or small-company domain (`a***@lovelace.family`), the first letter plus the
  domain can be enough to single out the customer, so the logs would still hold personal data
  (GDPR Art. 4(1), Art. 25). Whether this is acceptable depends on the domains in the customer
  base, which the review could not see; if the domain is not needed for support, mask it too or
  replace the address with a keyed hash.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
