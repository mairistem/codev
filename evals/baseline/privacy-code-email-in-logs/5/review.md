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
  Log only the opaque `user_id` (e.g. `log.info("new signup id=%s", user_id)`).
  As written, every sign-up copies the member's e-mail address and display name
  into the application logs, which have their own retention, access and shipping,
  and which account erasure does not reach (GDPR Art. 5(1)(c) data minimisation,
  Art. 5(1)(e) storage limitation, Art. 17 erasure).
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
  Drop `email` from the event: the stated purpose is only to count sign-ups
  ("Sign-ups are counted in product analytics."), which `user_id` alone — or no
  identifier at all — serves. As written, every sign-up sends the member's e-mail
  address to a third-party processor (`https://beacon.example.com/v1/events`,
  app/analytics.py:5), where the change defines no retention and no erasure path
  (GDPR Art. 5(1)(c), Art. 17). Beacon is also a new processor the proposal does
  not identify: name it in the proposal, with its location (EEA or not, GDPR
  Chapter V) and its data processing agreement (Art. 28).
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
