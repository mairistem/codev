# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

Disabled by configuration.

### privacy

#### R-privacy-1 Phone and display name sent to a US processor with no transfer basis

```yaml
severity: high
confidence: high
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: Identify Textly as a processor (GDPR Art. 28, data processing agreement) and name the Chapter V transfer mechanism (adequacy decision / EU-US Data Privacy Framework certification or standard contractual clauses) in the design, or choose an SMS provider hosted in the EEA; also state whether Textly keeps the numbers and for how long.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 No retention period or purge for birth date and phone, backups included

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  Two columns, `birth_date` and `phone`, are added to `profiles`. They are
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: Define the retention period (e.g. cleared when the member withdraws from greetings or after a period of inactivity) and how the values leave the nightly backups (backup rotation period, or no restore of erased data), per GDPR Art. 5(1)(e) storage limitation.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Birth date and phone made mandatory for a marketing purpose, no lawful basis stated

```yaml
severity: medium
confidence: high
location: "specs/birthday-greetings/spec.md#Requirement: Members give their birth date and phone number"
evidence: |
  The profile form SHALL ask for the member's birth date and mobile phone number, both required.
recommendation: Make both fields optional and tied to an explicit opt-in to birthday greetings (consent, GDPR Art. 6(1)(a) and ePrivacy Directive for marketing text messages), and state the purpose and lawful basis in the proposal; the "A missing birth date" scenario then accepts the form. The lawful basis is for the team to decide.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Full birth date collected when only day and month are needed

```yaml
severity: low
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The profile asks for a birth date and a mobile phone number, both required.
recommendation: Collect only the day and month of birth (the year has no use in the change), per GDPR Art. 5(1)(c) data minimisation.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Data subject rights not addressed for the new data and its copies

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
recommendation: State that access and portability exports include `birth_date` and `phone`, that members can rectify or remove them, and that erasure reaches the backups and the copy held by Textly (GDPR Art. 15-20); add a task and a scenario for it.
history:
  - status: open
    by: privacy
```

#### R-privacy-6 No rule keeping phone numbers out of the job's logs and retries

```yaml
severity: low
confidence: low
location: "design.md#Risks / Trade-offs"
evidence: |
  - [Textly is down] → the job retries the next day.
recommendation: State that the daily job logs and error reports carry the profile's opaque `user_id`, never the phone number, birth date or display name, including on failed or retried sends.
history:
  - status: open
    by: privacy
```
