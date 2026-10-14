# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One order_lines query per order (N+1) on the customer page

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
  for order in orders {
      let lines = db.lines_of_order(order.id)?;
recommendation: >-
  Fetch every line in one query with the existing batch method
  `db.lines_of_orders(&ids)` (api/src/db.rs:31, `WHERE order_id = ANY($1)`),
  then sum the totals per order id in a HashMap and build the summaries in the
  order `orders_of_customer` returned, so ordering and results are unchanged.
  Today `order_summaries` runs 1 + N queries, N up to 100 per call: a customer
  with 100 orders costs 101 database round trips on every customer page view
  (the proposal's Domains line), instead of 2.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
