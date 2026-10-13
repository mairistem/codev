# Review record: add-signup

<!-- codev-review: 1 -->

## code.review

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
  application logs, which have their own retention and access and are not
  reached by account erasure (GDPR Art. 5(1)(c) data minimisation, 5(1)(e)
  storage limitation, Art. 32).
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
  Drop `email` from the event properties: the stated purpose ("Sign-ups are
  counted in product analytics") needs no e-mail address, and `user_id` already
  identifies the event. As written, each sign-up ships the member's e-mail
  address to the third-party processor at beacon.example.com, where account
  erasure does not reach it (GDPR Art. 5(1)(c), Art. 25, Art. 28). Also identify
  Beacon as a processor in the plan and state whether it transfers data outside
  the EEA (Chapter V).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 No retention, erasure or lawful basis stated for the collected member data

```yaml
severity: medium
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — collects e-mail addresses and display names
recommendation: >-
  Question for the team: state the purpose and lawful basis for collecting the
  e-mail address and display name, their retention period (backups included),
  and how erasure and access reach the member row and the copies this change
  creates (logs, Beacon events). Only the controller can decide this; the
  proposal is silent (GDPR Art. 5(1)(e), Art. 6, Art. 15–20).
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
