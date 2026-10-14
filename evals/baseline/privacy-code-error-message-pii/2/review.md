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
  Keep the e-mail address out of the error message (for example `throw new NotFoundError("no customer for this e-mail")`). `errorHandler` in src/server.ts logs `err.message` (`logger.error("request failed", { path: req.path, error: err.message })`), so every lookup that finds no customer writes the typed e-mail address to the application log, which has no purpose for it (GDPR Art. 5(1)(c) data minimisation; criterion "No personal data goes to logs").
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
recommendation: Send the Helply widget only what the stated purpose needs — the proposal says the widget "greets the signed-in customer by name, so that the agent knows who they talk to", which needs `id` and `name` (at most `email`). Drop `phone`, `birthDate` and `address` from `chatWidgetSettings`, and stop selecting them in `customerById` if nothing else uses them. Today `GET /chat/settings` hands the full record to the third-party widget, so every signed-in customer's phone number, birth date and postal address goes to the vendor (GDPR Art. 5(1)(c), Art. 25; Art. 28 for the processor).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Helply processor and possible transfer outside the EEA not identified

```yaml
severity: medium
confidence: low
location: "src/support.ts:40"
evidence: |
  appId: process.env.HELPLY_APP_ID,
recommendation: Question for the team — the change starts sending customer data to Helply, but neither the proposal nor the code identifies Helply as a processor (Art. 28 agreement), states the lawful basis for sharing, or says whether the data leaves the EEA (Chapter V). Only the controller can confirm what was decided; record it in the proposal or a decision.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Lookup returns birth date, phone and address to agents

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: The stated purpose is that agents stop asking customers to repeat who they are; confirm which fields agents actually need and return only those from `POST /support/customers/lookup` (birth date and postal address may exceed that purpose, GDPR Art. 5(1)(c)). Depends on support workflows the reviewer could not see.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
