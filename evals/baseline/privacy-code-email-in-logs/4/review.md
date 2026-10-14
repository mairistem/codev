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
raised_by: [privacy]
recommendation: >-
  Log only the opaque `user_id` (e.g. `log.info("new signup id=%s", user_id)`).
  Every sign-up currently copies the member's e-mail address and display name
  into the application logs, which have no stated purpose, no retention rule
  and are not reached by an erasure request (GDPR Art. 5(1)(c) data
  minimisation, Art. 5(1)(e) storage limitation, Art. 17 erasure).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 E-mail address sent to the third-party analytics processor Beacon

```yaml
severity: high
confidence: high
location: "app/signup.py:15"
evidence: |
  track("signup", {"user_id": user_id, "email": email})
raised_by: [privacy]
recommendation: >-
  Drop `email` from the event: the proposal's purpose is only that "Sign-ups
  are counted in product analytics", which the event name (and at most the
  opaque `user_id`) already serves. As written, every sign-up posts the
  member's e-mail address to `https://beacon.example.com/v1/events`
  (app/analytics.py:5,9), an external processor the proposal never names —
  beyond what the purpose needs (Art. 5(1)(c)), out of reach of erasure and
  access requests (Art. 15, 17), and with no processor agreement or transfer
  location identified (Art. 28, Chapter V). Also list Beacon as a processor
  in the proposal's Impact, with its hosting region.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Purpose, lawful basis and retention of the stored member data not stated

```yaml
severity: medium
confidence: low
location: "app/signup.py:13"
evidence: |
  user_id = insert_user(email=email, display_name=display_name)
raised_by: [privacy]
recommendation: >-
  Question for the team: the proposal says the change "collects e-mail
  addresses and display names" but states no purpose per field, no lawful
  basis, no retention period and no deletion path (backups included) for the
  stored member record. Only the controller can decide these; record them in
  the proposal and make sure account deletion removes the record.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
