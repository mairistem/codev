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
  logs, which have their own retention and access and are not reached by
  erasure requests (GDPR Art. 5(1)(c) data minimisation, Art. 5(1)(e) storage
  limitation, Art. 17 erasure). Add a test asserting the log record holds no
  e-mail address.
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
  Drop `email` from the event properties. The stated purpose, "Sign-ups are
  counted in product analytics", needs no e-mail address, yet every sign-up
  sends it to the third party at https://beacon.example.com/v1/events
  (app/analytics.py:5), where it is kept outside the erasure path
  (GDPR Art. 5(1)(c) data minimisation, Art. 28 processors). Counting needs at
  most the opaque `user_id`, or no identifier at all.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Beacon not identified as a processor, nor its location

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1-5"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""
recommendation: >-
  Question for the team: the change introduces a new third-party recipient of
  member data (Beacon), but the proposal names neither the vendor, a
  processing agreement (GDPR Art. 28), nor whether data leaves the EEA
  (Chapter V). State them in the proposal, or confirm they are already covered
  by the records of processing (Art. 30).
history:
  - status: open
    by: privacy
```

#### R-privacy-4 No lawful basis or retention stated for the collected e-mail address and display name

```yaml
severity: medium
confidence: low
location: "proposal.md#What Changes"
evidence: |
  - `signup(email, display_name)` creates a member.
recommendation: >-
  Question for the team: self sign-up now collects e-mail address and display
  name from the public, but the proposal states neither the lawful basis
  (GDPR Art. 6) nor how long the member row is kept and how it is deleted
  (Art. 5(1)(e), Art. 17). Only the controller can say; record the answer in
  the proposal.
history:
  - status: open
    by: privacy
```
