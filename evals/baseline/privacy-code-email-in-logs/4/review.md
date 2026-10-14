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
  Log only the opaque id (`log.info("new signup id=%s", user_id)`). As written,
  every sign-up copies the member's e-mail address and display name into the
  INFO logs, where they escape the users table's retention and erasure, and
  reach whoever reads or ships the logs (GDPR Art. 5(1)(c) data minimisation,
  Art. 5(1)(e) storage limitation, Art. 32). Add a test asserting the e-mail
  does not appear in the captured log records.
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
  Drop `email` from the event properties: counting sign-ups, the purpose the
  proposal states ("Sign-ups are counted in product analytics"), needs no
  e-mail address, and `user_id` (or no identifier at all) suffices. As written,
  each sign-up discloses the member's e-mail to a third-party processor,
  beacon.example.com (app/analytics.py:5), which the proposal does not
  identify as a processor nor locate (inside or outside the EEA) (GDPR
  Art. 5(1)(c), Art. 28, Chapter V). Identify Beacon as a processor in the
  proposal, and add a test asserting `track` is called without the e-mail.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 No purpose, lawful basis or retention stated for the collected e-mail and display name

```yaml
severity: medium
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — collects e-mail addresses and display names
recommendation: >-
  Question for the team: state in the proposal why each field is collected
  (is the display name needed at sign-up?), the lawful basis, how long a
  member's row in `users` is kept, and how it is erased (backups included) —
  `signup()` stores both fields through `insert_user` with no retention or
  deletion path in the change (GDPR Art. 5(1)(b), 5(1)(e), Art. 6, Art. 17).
  Only the controller can say what was decided.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
