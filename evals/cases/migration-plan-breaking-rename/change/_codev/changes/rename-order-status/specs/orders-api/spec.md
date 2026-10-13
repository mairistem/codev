## Purpose

Names an order's lifecycle consistently.

## ADDED Requirements

### Requirement: Orders expose their state

`GET /orders/{id}` SHALL return the order's lifecycle in the field `state`, one of `open`, `paid`, `shipped`, `cancelled`.

#### Scenario: A paid order

- **WHEN** a client reads a paid order
- **THEN** the response holds `"state": "paid"`

#### Scenario: An unknown order

- **WHEN** a client reads an order that does not exist
- **THEN** the service answers 404
