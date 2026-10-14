# Review record: add-signup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 E-mail address and display name written to application logs

```yaml
severity: high
confidence: high
location: "app/signup.py:14"
evidence: |
  log.info("new signup email=%s name=%s id=%s", email, display_name, user_id)
recommendation: >-
  Log only the opaque member id (`log.info("new signup id=%s", user_id)`).
  As written, every sign-up copies the member's e-mail address and display
  name into the application logs, where they escape the users table's
  access control, retention and erasure (GDPR Art. 5(1)(c) data
  minimisation, 5(1)(e) storage limitation, Art. 17 erasure): deleting the
  member leaves their e-mail in every log file and log backup.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 E-mail address sent to the Beacon analytics SaaS

```yaml
severity: high
confidence: high
location: "app/signup.py:15"
evidence: |
  track("signup", {"user_id": user_id, "email": email})
recommendation: >-
  Drop `email` from the event properties. The stated purpose is that
  "Sign-ups are counted in product analytics" (proposal.md#What Changes),
  which needs no e-mail address; yet every sign-up posts the member's
  e-mail to the third party at https://beacon.example.com/v1/events
  (app/analytics.py:5), beyond what the purpose needs (GDPR Art. 5(1)(c),
  Art. 25), and out of reach of the member's erasure request (Art. 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Beacon is a new processor not identified in the plan

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1-5"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""
recommendation: >-
  The change introduces Beacon as a recipient of member data (at least the
  member id, a pseudonymous identifier) but the proposal's Impact and
  Domains name no processor. Question for the team: is a processing
  agreement in place with Beacon (GDPR Art. 28), where does it host the
  data (Chapter V, transfers outside the EEA), and is it listed in the
  records of processing (Art. 30)? State it in the proposal.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 No purpose, lawful basis or retention stated for the collected e-mail and display name

```yaml
severity: medium
confidence: low
location: "app/signup.py:11-13"
evidence: |
  def signup(email: str, display_name: str) -> int:
      """Creates the member and returns their id."""
      user_id = insert_user(email=email, display_name=display_name)
recommendation: >-
  The proposal only says the change "collects e-mail addresses and display
  names" (proposal.md#Domains). Question for the team: state the purpose
  and lawful basis of each field (Art. 6), how long a member's row in
  `users` is kept and how it is deleted, backups included (Art. 5(1)(e),
  Art. 17). Only the controller can tell what was decided.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
