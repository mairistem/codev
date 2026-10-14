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
  Two columns, `birthday` (month and day, no year) and `phone`, are added to `profiles`, filled only for members who opt in. They are
  kept as long as the profile exists, and copied with the rest of the
recommendation: >-
  Tie retention to the purpose (GDPR Art. 5(1)(e), storage limitation; Art. 7(3), withdrawal of consent):
  when a member turns the option off, clear `birthday` and `phone` in the same update, and add a spec
  scenario "A member opts out → both fields are erased" with a matching task. Today a member who opts out
  keeps both values in `profiles` until the whole profile is deleted, although no feature uses them any more;
  the spec's "No phone" scenario even assumes the birthday stays once the phone is removed.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 No retention or erasure rule for the copies in the nightly backups

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >-
  State how long the nightly backups holding `birthday` and `phone` are kept and how an opt-out or a profile
  deletion reaches them (backup expiry period, or re-applying erasures on restore). As written, a birthday and
  phone number erased from `profiles` survive in every backup for an undefined time (GDPR Art. 5(1)(e), Art. 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Transfer of phone numbers to Textly in the United States has no stated safeguard or processor terms

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  A question for the team: identify Textly as a processor in the plan, with its data processing agreement
  (GDPR Art. 28), the transfer mechanism for the United States (Chapter V — adequacy decision under the
  EU-US Data Privacy Framework if Textly is certified, otherwise standard contractual clauses), and how long
  Textly keeps the phone numbers and display names it receives. Only the controller can tell what was decided;
  add it to the design and to the records of processing (Art. 30).
history:
  - status: open
    by: privacy
```

#### R-privacy-4 The display name is sent to Textly without the purpose requiring it

```yaml
severity: low
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  A daily job selects the profiles whose birthday is today and posts each
  phone number and display name to Textly's API (hosted in the United
recommendation: >-
  Neither the proposal nor the spec says the message is personalised by name ("SHALL send them a text message").
  Send only the phone number and a fixed text, or state in the spec that the greeting carries the display name,
  so that what goes to the processor matches the stated purpose (GDPR Art. 5(1)(c), data minimisation).
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Lawful basis for the new data is not stated

```yaml
severity: low
confidence: low
location: "proposal.md#What Changes"
evidence: |
  - Members can opt in to birthday greetings; they then give their birthday (day and month) and a mobile phone number.
recommendation: >-
  A question for the team: the opt-in suggests consent (GDPR Art. 6(1)(a)), but the plan never names the lawful
  basis, while the proposal's "Why" cites marketing. State the basis in the proposal, and if it is consent, the
  wording shown on the profile form and how withdrawal works (see R-privacy-1).
history:
  - status: open
    by: privacy
```
