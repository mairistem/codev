# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Typed e-mail address written to the log platform on a failed lookup

```yaml
severity: high
confidence: high
location: "src/support.ts:20"
evidence: |
  throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Keep the e-mail address out of the error message (e.g. `no customer for
  this e-mail`), or carry it in a field the error handler never logs. The
  `errorHandler` registered at src/routes.ts:21 logs `err.message`
  (`logger.error("request failed", { path: req.path, error: err.message })`
  in src/server.ts), and src/logger.ts ships every line to the log
  platform: every lookup with no match puts the typed e-mail address — a
  customer's or a mistyped third party's — into the logs, with no retention
  rule of its own (GDPR Art. 5(1)(c) and 5(1)(e), Art. 32). The same holds,
  to a lesser degree, for the customer id at src/support.ts:32.
history:
  - status: open
    by: privacy
```

Failing scenario: a support agent POSTs `{"email": "jane@example.com"}` to
`/support/customers/lookup` for an address not in `customers`;
`customerByEmail` throws `NotFoundError("no customer with e-mail
jane@example.com")`, `errorHandler` logs that message as `error`, and the
address lands in the log platform.

#### R-privacy-2 Chat vendor Helply receives phone, birth date, address and e-mail to greet by name

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
  Send Helply only what the stated purpose needs — the proposal says the
  widget "greets the signed-in customer by name, so that the agent knows
  who they talk to": the customer id and name. Drop `email`, `phone`,
  `birthDate` and `address` from `chatWidgetSettings` (and stop selecting
  them in `customerById` if nothing else uses them). As written, every
  page load of `/chat/settings` (src/routes.ts:13-19) hands the full record
  to the browser and to the Helply widget script, i.e. to a third-party
  processor, beyond the purpose (GDPR Art. 5(1)(b)-(c), Art. 25, Art. 28).
history:
  - status: open
    by: privacy
```

Failing scenario: a signed-in customer opens any page with the chat widget;
`GET /chat/settings` returns their phone number, birth date and postal
address, and the widget initialises Helply with them although only the name
is used to greet.

#### R-privacy-3 Support lookup returns birth date, address and phone with no stated need

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
    "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  State which fields an agent needs to recognise the customer and return
  only those from `/support/customers/lookup`; the proposal's purpose is
  that agents stop asking customers "who they are", which the id, name and
  e-mail may already serve. Birth date and postal address are kept only if
  the team confirms a need (e.g. identity verification) — a question for
  the controller (GDPR Art. 5(1)(c)).
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Helply not identified as a processor, nor its data location

```yaml
severity: medium
confidence: low
location: "src/support.ts:37"
evidence: |
  /** What the Helply chat widget is started with, for the signed-in customer. */
recommendation: >-
  Ask the team to confirm that Helply is covered by a processing agreement
  (GDPR Art. 28), listed in the record of processing (Art. 30), and where
  it stores the data (Chapter V if outside the EEA), and how erasure
  requests reach the copy Helply keeps. The plan only says "the chat vendor
  receives who is chatting".
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
