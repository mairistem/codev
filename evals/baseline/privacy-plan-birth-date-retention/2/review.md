# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### privacy

#### R-privacy-1 Birthday and phone kept after opt-out, backups with no expiry

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >-
  Tie retention to the purpose (GDPR Art. 5(1)(e), storage limitation): a
  member who opts out of birthday greetings still has `birthday` and `phone`
  kept on `profiles` for the whole life of the profile, with no use left for
  them. Add a requirement and a scenario "A member opts out → `birthday` and
  `phone` are erased", clear both columns on opt-out and on account deletion,
  and state how long the nightly backups that hold them are kept and when
  erased values leave them. Add the matching task in tasks.md.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Display name sent to Textly beyond what the SMS needs

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
recommendation: >-
  Data minimisation (GDPR Art. 5(1)(c)): delivering a birthday text needs
  only the phone number and the message; posting `display_name` with it
  hands an identifying detail to a US processor and lets it link the number
  to a named person. Send only the phone number and a generic message, or,
  if the name must appear, render the message text on our side and record
  why in the design.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Transfer to Textly in the United States has no stated safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Question for the team: Textly becomes a processor (GDPR Art. 28) receiving
  phone numbers in a third country (Chapter V). State in the design the
  processing agreement and the transfer mechanism (adequacy decision / Data
  Privacy Framework certification, or standard contractual clauses), and how
  long Textly keeps phone numbers and message logs. Only the controller can
  say what was decided.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Lawful basis of the birthday processing not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Question for the team: the purpose is marketing messages by SMS, which
  under ePrivacy rules for electronic marketing usually requires consent.
  State in the proposal that the opt-in is the consent (GDPR Art. 6(1)(a)),
  that it is freely withdrawable from the profile form, and that withdrawal
  stops the job and triggers the erasure of R-privacy-1.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Access and portability exports not said to include the new columns

```yaml
severity: low
confidence: low
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  Two columns, `birthday` (month and day, no year) and `phone`, are added to `profiles`, filled only for members who opt in. They are
recommendation: >-
  If the project has a data export or subject-access process (GDPR Art.
  15 and 20), state that it covers `birthday`, `phone` and the opt-in flag;
  the reviewer could not see whether such an export exists.
history:
  - status: open
    by: privacy
```

### skeptic

Disabled by configuration.
