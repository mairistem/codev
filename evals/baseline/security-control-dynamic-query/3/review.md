# Review record: add-ticket-filters

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Search returns every ticket with no caller scope

```yaml
severity: medium
confidence: low
location: "app/search.py:22"
evidence: |
  def search_tickets(filters: dict[str, str], order: str = "newest", limit: int = 50) -> list[tuple]:
recommendation: >
  The new operation takes no caller identity and applies no restriction of its own: any console user who can
  call it lists every ticket, any assignee's included (`assignee` is a free filter, not the caller). If tickets
  are not meant to be visible to every agent, take the caller as a parameter and scope the query server-side
  (CWE-862). Confidence is low because no caller of `search_tickets` is in the change, so whether the console
  already restricts who reaches it could not be checked. The query itself is safe: column fragments and sort
  orders come only from the `FILTERS` and `ORDERS` allow-lists, and values and the limit are bound parameters.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
