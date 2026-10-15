# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One order-lines query per order on every customer page view

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
  for order in orders {
      let lines = db.lines_of_order(order.id)?;
recommendation: >-
  N+1 queries: `order_summaries` runs one query for the orders, then one
  `lines_of_order` query per order — up to 101 database round trips per call,
  growing with the customer's order count, on a path the proposal says runs on
  every customer page view. `Db` already offers the batch query
  `lines_of_orders(&[i64])` (`SELECT ... FROM order_lines WHERE order_id = ANY($1)`,
  api/src/db.rs:31): collect the order ids, fetch all their lines in one call,
  sum the totals per `order_id` (for example in a `HashMap<i64, i64>`), and build
  the summaries in the orders' original order, keeping orders without lines at a
  total of 0 — two queries per call whatever the number of orders.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
