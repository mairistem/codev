# Design: Rename an order's status to state

## Context

`GET /orders/{id}` is public: partners' integrations read `status` to know
when to ship. The domain model calls it `state` everywhere else.

## Decisions

### Decision: The field is renamed in place

`OrderResponse.status` becomes `OrderResponse.state`; the old name is not
kept. The OpenAPI file is regenerated.

**Alternatives considered**: keeping both fields (duplication).
