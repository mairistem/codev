# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### migration

#### R-migration-1 Public `status` field removed in the same release, with no compatibility window

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  The design's own context says "partners' integrations read `status` to know
  when to ship": on deploy, every partner calling `GET /orders/{id}` gets no
  `status` field (undefined) and stops shipping, with no warning. Use expand
  and contract: return both `status` and `state` with the same value, mark
  `status` as `deprecated: true` in `openapi.yaml` and keep `state` optional
  or both required during the window, announce a removal date, and remove
  `status` in a later change. Add to tasks.md a test that the response still
  holds `status` equal to `state`, and amend the spec's requirement so the
  deprecated field is part of the contract until its removal. The rejected
  alternative "keeping both fields (duplication)" should be revisited on these
  grounds.
history:
  - status: open
    by: migration
```

#### R-migration-2 Release note does not mark the rename as BREAKING nor say what partners must do

```yaml
severity: medium
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: The order's `status` field is now called `state`.
recommendation: >-
  A partner reading this release note sees an ordinary "Changed" item and
  gets no instruction, while their integration reading `status` breaks. Mark
  the item **BREAKING** (or, with R-migration-1's fix, "Deprecated"), state
  what consumers must do — read `state` instead of `status` — and by when
  `status` will be removed.
history:
  - status: open
    by: migration
```

### skeptic

Disabled by configuration.
