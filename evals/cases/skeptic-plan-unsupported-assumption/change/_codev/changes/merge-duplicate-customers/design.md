# Design: Merge duplicate customers

## Context

Customers are created at checkout; a guest checkout creates a new record
each time. Business customers often share a purchasing mailbox
(`orders@acme.example`) between several buyers, each with their own
customer record and billing address.

## Decisions

### Decision: Customers are merged by e-mail

Since an e-mail address identifies exactly one person, the job groups
customers by lower-cased e-mail, keeps the oldest record of each group,
moves the others' orders to it and deletes the others.

**Alternatives considered**: matching on name and address (fuzzy).
