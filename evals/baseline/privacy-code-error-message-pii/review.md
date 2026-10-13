# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

### privacy

#### R-privacy-1 Looked-up e-mail address written to the error log

```yaml
severity: high
confidence: high
location: "src/support.ts:20"
evidence: |
    throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Keep the e-mail address out of the error message (e.g. `throw new NotFoundError("no customer for this e-mail")`).
  Scenario: an agent looks up an address that matches no customer; `errorHandler` in src/server.ts
  runs `logger.error("request failed", { path: req.path, error: err.message })`, so the typed e-mail
  address (personal data, possibly of a non-customer) lands in the application logs with no retention
  rule (GDPR Art. 5(1)(c), 5(1)(e)). The same applies, to a lesser degree, to the customer id in
  `no customer ${id}` at src/support.ts:32.
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
    user: {
      id: String(customer.id),
      name: customer.name,
      email: customer.email,
      phone: customer.phone,
      birthDate: customer.birth_date,
      address: customer.address,
    },
recommendation: >-
  Send Helply only what the stated purpose needs — the id and the name ("greets the signed-in customer
  by name, so that the agent knows who they talk to"); drop `phone`, `birthDate` and `address` (and
  `email` unless the plan states why the vendor needs it). Scenario: any signed-in customer opening
  the chat causes GET /chat/settings to hand their phone number, date of birth and postal address to
  the browser-side widget and so to the third-party processor (GDPR Art. 5(1)(c), Art. 25, Art. 28).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Support lookup returns the full customer record

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
    "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The purpose is that agents no longer ask customers who they are; select and return only the fields an
  agent needs for that (likely id, name, e-mail) instead of the whole `Customer` with birth date and
  address, which `res.json(...)` at src/routes.ts:7 sends to the support console as is (GDPR Art. 5(1)(c)).
  Confidence is low: what the support console actually needs to display is not stated in the plan.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Helply as a new processor is not identified

```yaml
severity: low
confidence: low
location: "src/support.ts:40"
evidence: |
    appId: process.env.HELPLY_APP_ID,
recommendation: >-
  Question for the team: the change starts sharing customer data with Helply. Confirm a processor agreement
  exists (GDPR Art. 28), the record of processing is updated (Art. 30), and whether data is transferred
  outside the EEA (Chapter V). The proposal names the vendor but not its status.
history:
  - status: open
    by: privacy
```
