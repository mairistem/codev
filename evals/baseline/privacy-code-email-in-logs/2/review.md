# Review record: add-signup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 E-mail address and display name written to the application log

```yaml
severity: high
confidence: high
location: "app/signup.py:14"
evidence: |
  log.info("new signup email=%s name=%s id=%s", email, display_name, user_id)
recommendation: Log only the opaque id (`log.info("new signup id=%s", user_id)`). As written, every call to `signup("ada@example.org", "Ada")` copies the member's e-mail address and display name into the INFO log, where they escape the member's retention and erasure (GDPR Art. 5(1)(c) data minimisation, 5(1)(e) storage limitation, Art. 17 erasure) and reach whoever reads or ships the logs.
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
recommendation: Drop `email` from the event properties; counting sign-ups (the proposal's stated purpose) needs at most the opaque `user_id`. As written, each sign-up posts the member's e-mail address to `https://beacon.example.com/v1/events` (app/analytics.py:5), a third-party processor, where the member's erasure and access rights do not reach it (GDPR Art. 5(1)(b)-(c) purpose limitation and minimisation, Art. 28 processors).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 New analytics processor not identified in the plan

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""
recommendation: Question for the team — the proposal's Impact and Domains do not name Beacon as a new processor. Record it with its data processing agreement (GDPR Art. 28), where it hosts the data (Chapter V if outside the EEA), and how long it keeps the `signup` events and the `user_id` it receives. Depends on contracts and hosting the reviewer could not see.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 No stated purpose, lawful basis or retention for the collected e-mail and display name

```yaml
severity: medium
confidence: low
location: "app/signup.py:13"
evidence: |
  user_id = insert_user(email=email, display_name=display_name)
recommendation: Question for the team — the proposal states that sign-up "collects e-mail addresses and display names" but not why each is needed, on which lawful basis (GDPR Art. 6), how long they are kept, nor how account deletion erases them, backups included (Art. 5(1)(e), Art. 17). Only the controller can say; state it in the proposal. `app/db.py` is outside the change, so the existing retention could not be checked.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
