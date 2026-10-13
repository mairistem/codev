# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

### performance

#### R-performance-1 One query per order to load its lines (N+1)

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
      for order in orders {
          let lines = db.lines_of_order(order.id)?;
recommendation: >
  Each customer page view runs 1 query for the orders plus 1 query per order
  (`SELECT ... FROM order_lines WHERE order_id = $1`), up to 101 round trips
  per request, growing with the customer's order count and with traffic on a
  user-facing path. `Db::lines_of_orders(&[i64])` (api/src/db.rs:31, one
  `WHERE order_id = ANY($1)` query) already exists: collect the order ids,
  fetch all lines in one call, group them by `order_id` in a HashMap, and sum
  per order while keeping the orders' `ORDER BY id DESC` sequence. Two
  queries per request instead of up to 101.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
