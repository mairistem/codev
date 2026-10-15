# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### privacy

#### R-privacy-1 Birthday and phone kept after the member opts out

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
recommendation: >-
  The purpose of `birthday` and `phone` is the greeting, and the opt-in is
  their only reason to exist. Define what opting out does: when a member
  turns greetings off, clear both columns on `profiles` (GDPR Art. 5(1)(e)
  storage limitation, Art. 7(3) withdrawal of consent). Add a requirement
  and a scenario ("WHEN a member opts out THEN their birthday and phone are
  erased") to the spec, and a task to implement and test it. Today a member
  who opts out keeps their birth date and phone number in `profiles` until
  the whole profile is deleted.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 No retention period for the copies in the nightly backups

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  database into the nightly backups.
recommendation: >-
  The design copies `birthday` and `phone` into the nightly backups but
  states no expiry for those copies, so a member's birth date and phone
  number erased from `profiles` (opt-out or account deletion) survive in
  every backup indefinitely. State the backup retention period that bounds
  how long erased data lingers, and how an erasure is honoured after a
  restore (GDPR Art. 5(1)(e), Art. 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Display name sent to Textly when the greeting only needs the phone

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
recommendation: >-
  The spec only requires that "the service SHALL send them a text message";
  nothing needs the member's name. Each day the job hands Textly, a US
  processor, the display name together with the phone number, beyond what
  the purpose needs (GDPR Art. 5(1)(c) data minimisation). Send only the
  phone number and a message body that does not carry the name, or state in
  the spec why the name is needed in the message.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Transfer to Textly in the United States has no stated safeguard or processor agreement

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Question for the team: Textly is identified as a new processor receiving
  phone numbers outside the EEA. Which transfer mechanism covers it (EU-US
  Data Privacy Framework certification or standard contractual clauses,
  GDPR Chapter V), and is a processor agreement in place (Art. 28)? Record
  the answer in the design; only the controller can confirm it.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Lawful basis for the birthday data is not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Question for the team: the purpose is stated (birthday messages wanted by
  marketing) but not the lawful basis (GDPR Art. 6). The opt-in suggests
  consent; if so, say so in the proposal, and make sure the opt-in wording
  states the purpose and that it can be withdrawn.
history:
  - status: open
    by: privacy
```

### skeptic

Disabled by configuration.
