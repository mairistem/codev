# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### privacy

#### R-privacy-1 Birthday and phone are kept after the member opts out

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  Two columns, `birthday` (month and day, no year) and `phone`, are added to `profiles`, filled only for members who opt in. They are
  kept as long as the profile exists, and copied with the rest of the
recommendation: >-
  Tie retention to the purpose, not to the profile (GDPR Art. 5(1)(e),
  storage limitation): add a requirement and a scenario for opting out —
  when a member turns birthday greetings off, `birthday` and `phone` are
  cleared from `profiles` — and a task that implements and tests it. Today
  a member who withdraws keeps both fields stored for the life of the
  account, with no use left for them; the spec only covers "the phone
  number was removed", not withdrawal of the opt-in.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 No retention or erasure rule for the copies in nightly backups

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  copied with the rest of the
  database into the nightly backups.
recommendation: >-
  State how long the nightly backups holding `birthday` and `phone` are
  kept and how an erased or withdrawn member's values leave them (backup
  expiry period, or re-application of erasures on restore). As written, a
  member's birthday and phone number cleared from `profiles` survive in
  every backup with no defined end (GDPR Art. 5(1)(e), Art. 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Display name sent to Textly though the greeting does not need it

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
recommendation: >-
  Send Textly only the phone number and the message text it must deliver
  (GDPR Art. 5(1)(c), data minimisation). Nothing in the spec says the
  greeting carries the member's name; if it must, say so in the
  requirement and send the name only inside the message body. Otherwise
  every opted-in member's display name goes to a third-party processor
  for no stated purpose.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Transfer of phone numbers to a US processor has no stated safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  A daily job selects the profiles whose birthday is today and posts each
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Question for the team: Textly is a new processor outside the EEA. Record
  the processing agreement (GDPR Art. 28) and the transfer mechanism
  (Chapter V — adequacy decision such as the EU-US Data Privacy Framework,
  or standard contractual clauses) in the design, or pick an EEA-hosted SMS
  provider. Only the controller can confirm what was decided.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Purpose and lawful basis of the new data are not stated

```yaml
severity: medium
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Question for the team: state the purpose (a member-requested greeting, or
  marketing) and the lawful basis (presumably consent through the opt-in,
  GDPR Art. 6(1)(a)) for `birthday` and `phone`. If the messages serve
  marketing, the SMS needs prior consent under the ePrivacy Directive and
  the phone number must not be reused for other campaigns; say so in the
  proposal.
history:
  - status: open
    by: privacy
```

#### R-privacy-6 Access, portability and erasure are not shown to reach the new columns

```yaml
severity: low
confidence: low
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  Two columns, `birthday` (month and day, no year) and `phone`, are added to `profiles`, filled only for members who opt in. They are
recommendation: >-
  Confirm in the design that the existing account export and account
  deletion cover the new `birthday` and `phone` columns (GDPR Art. 15,
  17, 20), and add a task to check it. The material does not show how
  these rights are served today, so this could not be verified.
history:
  - status: open
    by: privacy
```

### skeptic

Disabled by configuration.
