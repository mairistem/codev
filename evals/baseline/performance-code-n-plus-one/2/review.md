# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One order_lines query per order (N+1) in order_summaries

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
  for order in orders {
      let lines = db.lines_of_order(order.id)?;
recommendation: >-
  Load every line in one query with the existing batch method
  `db.lines_of_orders(&ids)` (`api/src/db.rs:31`, `WHERE order_id = ANY($1)`),
  then group the lines by `order_id` (a `HashMap<i64, i64>` of totals) and build
  the summaries in the order `orders_of_customer` returned them, so ordering and
  results stay unchanged. This turns 1 + N queries (up to 101) into 2 per call.
history:
  - status: open
    by: performance
```

Scenario: `order_summaries` runs on every customer page view (proposal `## Domains`). For a customer with 100 orders, each view issues 1 query for the orders plus 100 `SELECT … FROM order_lines WHERE order_id = $1` round-trips (`api/src/db.rs:25`, "Each method runs one query"), so database load and page latency grow with the customer's order count, up to 101 queries per request, multiplied by traffic — while `lines_of_orders` can serve the whole set in one call.

### newcomer

Disabled by configuration.
