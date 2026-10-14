# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### privacy

#### R-privacy-1 Birthday and phone are kept after the member opts out

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  filled only for members who opt in. They are
  kept as long as the profile exists, and copied with the rest of the
recommendation: >-
  The only purpose of `profiles.birthday` and `profiles.phone` is the
  greeting, but the design ties their retention to the profile, not to the
  opt-in. A member who turns the option off keeps both values in `profiles`
  for as long as the account lives, with no purpose left (GDPR Art. 5(1)(c)
  and (e), Art. 7(3) withdrawal of consent). The spec only covers opting in
  ("Members opt in to birthday greetings"). Add a requirement and a scenario
  "WHEN a member opts out THEN their birthday and phone are erased", state
  in the design that opting out clears both columns, and add the matching
  task and test.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 No retention or purge rule for the copies in nightly backups

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  copied with the rest of the
  database into the nightly backups.
recommendation: >-
  Birth dates and phone numbers are copied into the nightly backups, but
  nothing says how long those backups are kept, so a value erased on opt-out
  or account deletion survives there for an undefined time (GDPR Art. 5(1)(e),
  Art. 17). State the backup retention period in the design (or point to the
  existing backup policy) and how an erasure reaches restored backups.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Transfer of phone numbers and names to Textly in the United States has no stated safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  The processor and the transfer outside the EEA are named, but the plan
  does not say whether a processing agreement (GDPR Art. 28) and a transfer
  mechanism (Chapter V: adequacy decision, e.g. the EU-US Data Privacy
  Framework, or standard contractual clauses) cover Textly, nor whether
  Textly keeps the numbers and messages it receives, and for how long.
  Question for the team: record the agreement, the transfer basis and
  Textly's retention in the design.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Lawful basis of the processing is not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  The purpose is stated (birthday messages, wanted by marketing), but not
  the lawful basis (GDPR Art. 6). The opt-in suggests consent; if so, say it
  in the proposal, together with how consent is recorded and withdrawn.
  Question for the team: only the controller can tell what was decided.
history:
  - status: open
    by: privacy
```
