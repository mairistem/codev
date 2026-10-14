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
recommendation: Log only the opaque `user_id` (e.g. `log.info("new signup id=%s", user_id)`). As written, every sign-up copies the member's e-mail address and display name into the application log and every log sink behind it, where they escape the retention and erasure of the `users` table (GDPR Art. 5(1)(c) data minimisation, Art. 5(1)(e) storage limitation, Art. 17 erasure). Add a test asserting the e-mail does not appear in the captured log records.
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
recommendation: Drop `email` from the event properties and send only `user_id` (or no identifier at all, since the proposal only needs sign-ups to be "counted in product analytics"). As written, every sign-up posts the member's e-mail address to the third-party processor at https://beacon.example.com/v1/events (app/analytics.py:5), beyond what the stated purpose needs (GDPR Art. 5(1)(b)-(c), Art. 25 privacy by default, Art. 28 processor). Add a test asserting the `track` call carries no e-mail.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Beacon is a new processor not identified in the plan

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""
recommendation: The proposal's Impact and Domains do not name Beacon as a processor receiving member data (at least `user_id`), nor where it hosts that data. Question for the team — confirm a data processing agreement exists (GDPR Art. 28), whether the data leaves the EEA (Chapter V), and record Beacon in the records of processing (Art. 30).
history:
  - status: open
    by: privacy
```

#### R-privacy-4 No retention period or erasure path for the collected sign-up data

```yaml
severity: medium
confidence: low
location: "app/signup.py:13"
evidence: |
  user_id = insert_user(email=email, display_name=display_name)
recommendation: The plan states no retention period for the e-mail address and display name now collected by self sign-up, nor how account deletion reaches the `users` row, the logs and the Beacon events (backups included). Question for the team — define the retention and the erasure path (GDPR Art. 5(1)(e), Art. 17), and state the purpose and lawful basis of the collection (Art. 6).
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
