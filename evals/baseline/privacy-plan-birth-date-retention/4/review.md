# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### privacy

#### R-privacy-1 Opting out leaves the birthday and phone number stored

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  Two columns, `birthday` (month and day, no year) and `phone`, are added to `profiles`, filled only for members who opt in. They are
  kept as long as the profile exists, and copied with the rest of the
recommendation: >-
  Tie retention to the purpose, not to the profile (GDPR Art. 5(1)(c) and
  5(1)(e)): when a member turns the option off, clear `birthday` and `phone`
  in the same update. Add a requirement and scenario for it to the spec
  ("WHEN a member opts out THEN their birthday and phone number are erased")
  and a test to task 1.2. Today a member who opts out keeps both values on
  their profile for as long as the account lives, with no use left for them;
  the spec's "No phone" scenario even assumes the phone can be removed while
  the birthday stays.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 No retention or purge for the copies in nightly backups

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >-
  State how long the nightly backups holding `birthday` and `phone` are kept
  and how an erased or opted-out member's values leave them (backup expiry
  period, or re-application of deletions on restore). Without it, a member's
  erasure request or opt-out removes the live row while the backups keep the
  data indefinitely (GDPR Art. 5(1)(e), Art. 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Display name sent to Textly beyond what delivery needs

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  A daily job selects the profiles whose birthday is today and posts each
  phone number and display name to Textly's API (hosted in the United
recommendation: >-
  Send Textly only what sending the SMS needs — the phone number and the
  message text — or state why the name must reach the processor (data
  minimisation, GDPR Art. 5(1)(c)). If the greeting is personalised, compose
  the text on our side so Textly does not receive the name as a separate
  field tied to the number. Confidence low: whether the greeting is meant to
  carry the name is not stated in the plan.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Transfer to a US processor without a stated safeguard or processor agreement

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Question for the team: name Textly as a processor in the proposal's Impact
  (it lists only "Textly account") and state the processor agreement
  (GDPR Art. 28) and the transfer mechanism for sending phone numbers and
  names to the United States (Chapter V — e.g. EU-US Data Privacy Framework
  certification or standard contractual clauses), plus Textly's own
  retention of the numbers. Only the controller can confirm what is in
  place.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Purpose and lawful basis not stated; marketing purpose mixed in

```yaml
severity: medium
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Question for the team: state the purpose of the new birthday and phone
  data and its lawful basis (presumably consent, given the opt-in — GDPR
  Art. 6(1)(a), Art. 7), and state that the phone number is used only for
  the birthday greeting, not for other marketing messages. If marketing
  intends further use, that is a separate purpose needing its own basis and,
  for SMS marketing, ePrivacy consent.
history:
  - status: open
    by: privacy
```
