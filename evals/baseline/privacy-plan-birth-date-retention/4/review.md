# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### privacy

#### R-privacy-1 Birthday and phone kept after opt-out and in backups with no expiry

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >
  Tie retention to the purpose (GDPR Art. 5(1)(e)): when a member opts out
  or withdraws consent, clear `birthday` and `phone` on `profiles`; add a
  requirement and scenario for opting out (the spec only covers opting in),
  and a task for it. State the retention period of the nightly backups and
  how cleared values age out of them. Today a member who opts out, or whose
  phone is removed ("No phone" scenario), keeps their birthday stored for the
  life of the profile and in every backup, with no use left.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Access, portability and erasure not shown to reach the new columns

```yaml
severity: medium
confidence: low
location: "tasks.md#1. Greetings"
evidence: |
  - [ ] 1.1 Columns `birthday` and `phone`, verified by `npm test -- profiles`
recommendation: >
  Check that the existing data export (Art. 15, 20) and account deletion
  (Art. 17) include `birthday` and `phone`, and Textly's copy of the phone
  number; if they enumerate fields, add a task for each. The plan does not
  mention them and the export/deletion code could not be seen.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Transfer of phone numbers to Textly in the United States has no stated safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >
  The processor and the transfer outside the EEA are identified; question
  for the team: record the data processing agreement with Textly (Art. 28),
  the Chapter V transfer mechanism (adequacy decision or SCCs), how long
  Textly keeps the numbers and messages, and the lawful basis (presumably
  consent via the opt-in, Art. 6(1)(a)). Only the controller can confirm
  what was decided.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Display name sent to Textly may exceed what the greeting needs

```yaml
severity: low
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  A daily job selects the profiles whose birthday is today and posts each
  phone number and display name to Textly's API (hosted in the United
recommendation: >
  If the message text is built on our side, send Textly only the phone
  number and the final message body rather than the display name as a
  separate field (data minimisation, Art. 5(1)(c)); if personalising with
  the name is not required, send the phone number alone.
history:
  - status: open
    by: privacy
```
