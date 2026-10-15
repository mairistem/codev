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
  Fetch every order's lines in one query with the existing
  `Db::lines_of_orders(&[i64])` (`SELECT ... FROM order_lines WHERE
  order_id = ANY($1)`, api/src/db.rs:30-33), passing the ids of the orders
  just loaded, then sum the lines into a map keyed by `order_id` and build
  the summaries in the original order of `orders` (keep orders with no line
  at total 0). That turns up to 101 round trips per page view into 2,
  without changing results or ordering.
history:
  - status: open
    by: performance
```

Scenario: `order_summaries` runs on every customer page view (proposal
`## Domains`). For a customer with 100 orders, `db.orders_of_customer`
returns 100 rows and the loop then issues `lines_of_order` once per order —
101 queries per request (`api/src/db.rs:1`: "Each method runs one query."),
cost growing linearly with the customer's orders up to the limit and
multiplied by traffic, while a batch method serving the whole set already
exists.

### newcomer

Disabled by configuration.
