# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Looked-up e-mail address written to the error log

```yaml
severity: high
confidence: high
location: "src/support.ts:20"
evidence: |
  throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Keep the e-mail address out of the error message (e.g. `no customer for this
  e-mail`): `errorHandler` in src/server.ts logs `error: err.message` for every
  failed request, so each unsuccessful lookup on POST /support/customers/lookup
  writes the typed e-mail address to the application log, where it has no
  retention rule and escapes erasure (GDPR Art. 5(1)(c), 5(1)(e)). The same
  applies, to a lesser degree, to the customer id in `no customer ${id}`
  (src/support.ts:32). Add a test asserting the logged message holds no e-mail.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor receives phone, birth date and address to greet by name

```yaml
severity: high
confidence: high
location: "src/support.ts:41-48"
evidence: |
      email: customer.email,
      phone: customer.phone,
      birthDate: customer.birth_date,
      address: customer.address,
recommendation: >-
  The proposal's purpose is that the Helply widget greets the signed-in
  customer by name; send only `id` and `name` (and the e-mail only if the
  agent's identification needs it, stated in the plan). As written, every
  signed-in customer who loads /chat/settings hands their phone number, birth
  date and postal address to the third-party chat vendor, beyond what the
  purpose needs (GDPR Art. 5(1)(c) data minimisation, Art. 25).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Helply as processor and its transfer location are not identified

```yaml
severity: medium
confidence: low
location: "src/support.ts:40"
evidence: |
    appId: process.env.HELPLY_APP_ID,
recommendation: >-
  Question for the team: record Helply as a processor (Art. 28 contract,
  Art. 30 record of processing), state whether customer data leaves the EEA
  (Chapter V), and state the lawful basis of sharing the chatting customer's
  identity with it. Neither the proposal nor the code says so.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Agent lookup returns birth date and address with no stated need

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
    "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The stated purpose is that agents stop asking customers who they are; the
  lookup returns the whole record, birth date and postal address included, to
  every support agent. Select only the fields agents need, or state in the
  proposal why the birth date and address are needed (e.g. identity
  verification) (GDPR Art. 5(1)(c)).
history:
  - status: open
    by: privacy
```
