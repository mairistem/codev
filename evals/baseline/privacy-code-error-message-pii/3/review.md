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
  Keep the e-mail address out of the error message, e.g.
  `throw new NotFoundError("no customer with this e-mail")`. As written, every
  lookup that matches no customer (a typo, or an address that is not a
  customer's) reaches `errorHandler` in src/server.ts, which runs
  `logger.error("request failed", { path: req.path, error: err.message })`,
  so the typed address lands in the application logs and wherever they are
  shipped (GDPR Art. 5(1)(c) and 5(1)(f), Art. 32). The 404 body itself stays
  generic; only the log leaks.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor receives phone, birth date and address it does not need

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
  The proposal's stated purpose is that the Helply widget "greets the
  signed-in customer by name, so that the agent knows who they talk to";
  the opaque id and the name serve it. Drop `phone`, `birthDate` and
  `address` (and `email` unless the team states why Helply needs it) from
  the settings, and select only the columns needed in `customerById`.
  Today every signed-in customer who opens the chat sends their full
  record through GET /chat/settings to the browser and on to Helply, a
  third-party processor (GDPR Art. 5(1)(c) data minimisation, Art. 25,
  Art. 28).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Helply as processor: contract, location and lawful basis not identified

```yaml
severity: medium
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — customers' records are read and the chat vendor receives who is chatting
recommendation: >-
  Question for the team: name Helply as a processor in the plan, confirm a
  data processing agreement (GDPR Art. 28), where it stores the data
  (Chapter V if outside the EEA), the lawful basis for sharing (Art. 6), and
  how Helply's copy is retained and erased when a customer is deleted or
  exercises their rights (Art. 15–17). The code cannot show any of this;
  only the controller can.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Support lookup returns birth date and address to agents by default

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The lookup exists so that agents stop asking customers who they are; the
  route returns the whole record, birth date and address included, to every
  `support` user. Unless the team states that agents need those fields,
  return only id, name and e-mail (GDPR Art. 5(1)(c), Art. 25). Recorded with
  low confidence: what agents actually need is not stated in the plan.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
