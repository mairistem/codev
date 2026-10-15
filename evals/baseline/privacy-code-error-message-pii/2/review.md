# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Customer e-mail address written to the error log on a failed lookup

```yaml
severity: high
confidence: high
location: "src/support.ts:20"
evidence: |
  throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Keep the e-mail address out of the error message — throw
  `new NotFoundError("no customer with that e-mail")` (or carry no detail at
  all). `errorHandler` (src/server.ts:14) logs `err.message` for every failed
  request, so each lookup that misses writes the typed address — a customer's
  or a mistyped one — to the application log, where it has no retention or
  erasure rule (GDPR Art. 5(1)(c) minimisation, Art. 5(1)(e) storage
  limitation). Scenario: an agent posts `{"email": "jane@example.com"}` to
  `/support/customers/lookup` for an unknown address; the log receives
  `request failed {"path": "/support/customers/lookup", "error": "no customer
  with e-mail jane@example.com"}`.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Phone, birth date, address and e-mail sent to the Helply chat vendor when the purpose is a greeting by name

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
recommendation: >-
  Send Helply only what the stated purpose needs — the proposal says the
  widget "greets the signed-in customer by name, so that the agent knows who
  they talk to": `id` and `name`, nothing more. Drop `email`, `phone`,
  `birthDate` and `address` from `chatWidgetSettings`, and select only the
  columns needed in `customerById` (src/support.ts:28). Today every signed-in
  customer who loads `/chat/settings` hands their phone number, date of birth
  and postal address to the third-party widget (GDPR Art. 5(1)(c)
  minimisation, Art. 25 data protection by default, Art. 28 processor).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Helply is a new processor that the change does not identify

```yaml
severity: medium
confidence: low
location: "src/support.ts:37-40"
evidence: |
  /** What the Helply chat widget is started with, for the signed-in customer. */
  export function chatWidgetSettings(customer: Customer) {
    return {
      appId: process.env.HELPLY_APP_ID,
recommendation: >-
  Question for the team: is Helply already listed as a processor with a
  data processing agreement (GDPR Art. 28, Art. 30 records), and does it
  store or process the data outside the EEA (Chapter V)? The proposal names
  "the chat vendor" but neither the processor's contract, its hosting
  region, nor the retention of the chat identities it receives; only the
  controller can tell what was decided.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Support lookup returns birth date and address with no stated need

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The proposal's purpose is that agents stop asking customers "who they
  are"; whether an agent needs the full birth date and postal address for
  that is not stated. Confirm with the team which fields the support
  console shows, and select only those (GDPR Art. 5(1)(c) minimisation).
  Low confidence: the console's needs are not visible in the change.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
