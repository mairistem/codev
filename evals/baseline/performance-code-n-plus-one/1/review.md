# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One order-lines query per order (N+1) on the customer page

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
  for order in orders {
      let lines = db.lines_of_order(order.id)?;
recommendation: >-
  Fetch every order's lines in one query: collect the order ids, call the
  existing `db.lines_of_orders(&ids)` (`WHERE order_id = ANY($1)`, api/src/db.rs:31),
  group the lines by `order_id` (e.g. a HashMap of totals), then build the
  summaries in the order `orders_of_customer` returned them, so results and
  ordering stay unchanged. Today a customer with 100 orders costs 101 queries
  (1 + one per order) on every customer page view, a user-facing path the
  proposal marks as hot; with the batch it is 2 queries whatever the count.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
