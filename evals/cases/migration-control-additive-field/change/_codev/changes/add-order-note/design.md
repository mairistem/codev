# Design: Order notes

## Context

`GET /orders/{id}` is public; partners' clients are generated from
`openapi.yaml`, and our API guidelines (`docs/api-guidelines.md`) require
clients to ignore unknown fields.

## Decisions

### Decision: A nullable column and an optional field

Migration `0012_order_note.sql` adds `orders.note TEXT NULL` — no default,
no rewrite of existing rows — and `0012_order_note.down.sql` drops it. The
field is optional in `openapi.yaml` and omitted when null, so the
responses of existing orders are byte-for-byte unchanged.

**Alternatives considered**: an empty string by default (changes every
existing response).

### Decision: Deployed in two steps

The migration ships in one release, the code that writes and returns the
note in the next, so old and new versions of the service run side by side
during the blue/green switch.

**Alternatives considered**: one release (the old version would run against
the new schema, which it tolerates, but the order is safer).
