# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

### privacy

#### R-privacy-1 Looked-up e-mail address written to the logs through the error message

```yaml
severity: high
confidence: high
location: "src/support.ts:20"
evidence: |
    throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Keep the e-mail address out of the error message (e.g. `new NotFoundError("no customer for this e-mail")`);
  `errorHandler` in src/server.ts:14 logs `err.message` verbatim
  (`logger.error("request failed", { path: req.path, error: err.message });`), so every
  unsuccessful lookup ships the typed address to the log platform — GDPR Art. 5(1)(c) and 32.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor receives phone, birth date and address when only the name is needed

```yaml
severity: high
confidence: high
location: "src/support.ts:42-47"
evidence: |
      id: String(customer.id),
      name: customer.name,
      email: customer.email,
      phone: customer.phone,
      birthDate: customer.birth_date,
      address: customer.address,
recommendation: Send Helply only what the stated purpose needs — the proposal says the widget "greets the signed-in customer by name", so an opaque id and the name; drop phone, birthDate and address (and email unless a stated use requires it). Data minimisation, GDPR Art. 5(1)(c) and 25; the full record is also exposed to every script on the page through the `/chat/settings` response.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Helply is a new processor with no transfer, retention or erasure stated

```yaml
severity: medium
confidence: low
location: "src/support.ts:37-40"
evidence: |
  /** What the Helply chat widget is started with, for the signed-in customer. */
  export function chatWidgetSettings(customer: Customer) {
    return {
      appId: process.env.HELPLY_APP_ID,
recommendation: Question for the team — record Helply as a processor (Art. 28, Art. 30), state where it hosts the data (Chapter V transfer if outside the EEA), how long it keeps the customer profile, and how account erasure and access requests (Art. 15–17) reach the copy held by Helply.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Support lookup returns birth date, phone and address with no stated purpose

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
    "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: State why the support agent needs each field (e.g. birth date for identity checks) or select only id and name for the lookup; the proposal's purpose is that agents know who the customer is. GDPR Art. 5(1)(b)-(c).
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
