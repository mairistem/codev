# Review record: rename-order-status

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### migration

#### R-migration-1 Public field removed with no deprecation or transition period

```yaml
severity: high
confidence: high
location: "design.md#Decision: The field is renamed in place"
evidence: |
  `OrderResponse.status` becomes `OrderResponse.state`; the old name is not
  kept. The OpenAPI file is regenerated.
recommendation: >-
  Follow expand then contract. In this change, `GET /orders/{id}` returns
  both `state` and `status` with the same value, and `status` is marked
  deprecated in `openapi.yaml` with a stated removal date or version.
  Remove `status` in a later change, after partners have had time to
  migrate. Add a spec scenario and a task test showing that a paid order
  still holds `"status": "paid"` next to `"state": "paid"` during the
  deprecation window. The design's rejection of this option ("keeping both
  fields (duplication)") should weigh the cost to partners, not only the
  duplication.
history:
  - status: open
    by: migration
```

The design's own context says partners' integrations read `status` to
know when to ship. On the day this ships, a partner integration doing
`GET /orders/{id}` on a paid order gets no `status` field: its check
against `"paid"` fails (the field is undefined), so it never ships the
order, with no error raised on either side. Partners cannot deploy their
update in step with the API, so there is no window in which old and new
consumers both work.

#### R-migration-2 The rename is not marked BREAKING and the release note gives partners no action

```yaml
severity: high
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The order JSON's `status` field is renamed `state`.
recommendation: >-
  Mark the item **BREAKING** in What Changes, and turn the release note's
  `- Changed: The order's `status` field is now called `state`.` into a
  **BREAKING** item that tells API consumers what to do: read `state`
  instead of `status`, from which version or date, and until when `status`
  remains available (see R-migration-1).
history:
  - status: open
    by: migration
```

`GET /orders/{id}` is public and partners read `status`, yet neither What
Changes nor the release note (`- Changed: The order's `status` field is
now called `state`.`) flags the change as breaking or says what consumers
must change. A partner skimming the release note has no signal that their
integration will stop seeing the field.
