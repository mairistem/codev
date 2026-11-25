## Purpose

Lets customers leave a note for the shop on an order.

## ADDED Requirements

### Requirement: Orders carry an optional note

`GET /orders/{id}` SHALL return an optional `note` field, a string of at most 500 characters, absent when the customer left none. Existing fields are unchanged.

#### Scenario: An order with a note

- **WHEN** a client reads an order whose customer left a note
- **THEN** the response holds `note`

#### Scenario: An order without note

- **WHEN** a client reads an order without note
- **THEN** the response has no `note` field and is otherwise unchanged
