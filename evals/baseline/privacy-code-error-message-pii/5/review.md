# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Looked-up e-mail address written to the error log

```yaml
severity: high
confidence: high
location: "src/support.ts:20-21"
evidence: |
  if (rows.length === 0) {
    throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Keep the e-mail address out of the error message (e.g. `no customer with this e-mail`).
  As written, every lookup of an unknown address on POST /support/customers/lookup reaches
  `errorHandler` (src/server.ts), which runs `logger.error("request failed", { path: req.path,
  error: err.message })`, so the typed e-mail address lands in the application logs, with no
  retention rule and out of reach of erasure (GDPR Art. 5(1)(c), 5(1)(e), 32).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor receives phone, birth date and address it does not need

```yaml
severity: high
confidence: high
location: "src/support.ts:43-49"
evidence: |
      id: String(customer.id),
      name: customer.name,
      email: customer.email,
      phone: customer.phone,
      birthDate: customer.birth_date,
      address: customer.address,
recommendation: >-
  The proposal's purpose is to greet the signed-in customer by name so the agent knows who
  they talk to; an opaque id and the name serve it. Drop `phone`, `birthDate` and `address`
  (and `email`, unless the plan states why Helply needs it) from the settings sent to the
  Helply widget: every page load of GET /chat/settings shares them with a third-party
  processor and exposes them to any script on the page (GDPR Art. 5(1)(c), 25, 28). Also
  select only the needed columns in `customerById`.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Agent lookup returns the full customer record

```yaml
severity: medium
confidence: low
location: "src/support.ts:15-17"
evidence: |
  const rows = await query<Customer>(
    "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The plan says agents look a customer up so they need not ask who they are; whether they also
  need the birth date, phone and postal address is not stated. Return only the fields the
  support purpose needs, or have the plan state why each is needed (GDPR Art. 5(1)(c)).
  Confidence low: the agents' actual need is not visible in the change.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Helply not identified as a processor, transfer location unknown

```yaml
severity: medium
confidence: low
location: "src/support.ts:38-41"
evidence: |
  /** What the Helply chat widget is started with, for the signed-in customer. */
  export function chatWidgetSettings(customer: Customer) {
    return {
      appId: process.env.HELPLY_APP_ID,
recommendation: >-
  Question for the team: is Helply covered by a processor agreement (GDPR Art. 28), where does it
  host data (Chapter V if outside the EEA), and what is the lawful basis and retention of what
  it receives? Record it in the plan; only the controller can answer.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
