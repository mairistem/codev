# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

Disabled by configuration.

### privacy

#### R-privacy-1 Phone numbers and names go to a US processor with no transfer safeguard identified

```yaml
severity: high
confidence: high
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  A daily job selects the profiles whose birthday is today and posts each
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: Identify Textly as a processor (Art. 28 data processing agreement) and state the Chapter V transfer mechanism (adequacy decision / EU-US Data Privacy Framework certification, or standard contractual clauses), or choose an EEA-hosted SMS provider; record the processing in the Art. 30 register.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Birthday and phone are kept for the profile's lifetime, including after opt-out, and in backups with no expiry

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: Tie retention to the opt-in — clear `birthday` and `phone` when the member opts out or the profile is deleted — and state the backup retention period after which erased values are gone from backups (Art. 5(1)(e) storage limitation).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 No requirement for opting out or erasing the new data

```yaml
severity: medium
confidence: high
location: "specs/birthday-greetings/spec.md#Requirement: Members opt in to birthday greetings"
evidence: |
  The profile form SHALL offer birthday greetings as an option, off by default; a member who opts in SHALL give their birthday (day and month) and a mobile phone number.
recommendation: Add a requirement and scenarios for withdrawing consent (opting out stops greetings and erases `birthday` and `phone`, Art. 7(3) and Art. 17), and state that access and export of the profile include the two new fields (Art. 15, Art. 20); add matching tasks.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Purpose and lawful basis of the new data are not stated

```yaml
severity: medium
confidence: high
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: Question for the team — state the purpose (sending the birthday greeting only, or also marketing) and the lawful basis (consent via the opt-in, Art. 6(1)(a)); if marketing messages are intended, the SMS needs ePrivacy-compliant consent for direct marketing and the purpose must be disclosed at opt-in.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Display name is sent to Textly where the phone number alone would do

```yaml
severity: low
confidence: high
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
recommendation: Send only the phone number and a generic message, or state why the personalised name is needed (Art. 5(1)(c) data minimisation).
history:
  - status: open
    by: privacy
```

#### R-privacy-6 No rule keeps phone numbers out of the job's logs and retry state

```yaml
severity: low
confidence: low
location: "design.md#Risks / Trade-offs"
evidence: |
  - [Textly is down] → the job retries the next day.
recommendation: State that the job and its retry logic log only the profile id, never the phone number, birthday or name, nor Textly's request body.
history:
  - status: open
    by: privacy
```
