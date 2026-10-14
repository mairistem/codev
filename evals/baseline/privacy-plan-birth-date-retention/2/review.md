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
  kept as long as the profile exists, and copied with the rest of the
recommendation: >-
  Tie retention to the consent, not to the profile: when a member turns
  birthday greetings off, clear `birthday` and `phone` on their profile
  (Art. 5(1)(e) storage limitation, Art. 7(3) withdrawal of consent). Add
  an opt-out requirement and scenario to the spec ("WHEN a member opts out
  THEN their birthday and phone are erased") and a task with its test.
  Today a member who opts out keeps both fields stored for as long as the
  account lives, with no purpose left for them.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Retention of birthday and phone in the nightly backups is undefined

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  database into the nightly backups.
recommendation: >-
  State how long the nightly backups keep these columns and how an erasure
  (opt-out or account deletion) reaches them — for example a bounded
  backup rotation after which erased values are gone, or a re-applied
  erasure on restore. The backup retention policy may exist outside this
  plan; the design should cite it.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Textly, a US-hosted processor, has no stated processing agreement or transfer safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
recommendation: >-
  Identify Textly as a processor and the transfer outside the EEA: name
  the data processing agreement (Art. 28) and the Chapter V mechanism
  (adequacy decision / EU-US Data Privacy Framework certification, or
  standard contractual clauses), and Textly's own retention of the phone
  numbers and messages it receives. Question for the team: only the
  controller can tell whether these are in place.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Lawful basis and purpose of the new data are not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  State the purpose (birthday greetings only, no other marketing use of the
  phone number) and the lawful basis — presumably consent through the
  opt-in, Art. 6(1)(a) — in the proposal, so the birthday and phone are not
  reused for other marketing campaigns. Question for the team: only the
  controller can confirm the basis.
history:
  - status: open
    by: privacy
```
