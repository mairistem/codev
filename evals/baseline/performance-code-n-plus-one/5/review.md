# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One order-lines query per order (N+1)

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
  for order in orders {
      let lines = db.lines_of_order(order.id)?;
recommendation: >-
  Collect the ids of the fetched orders and load every line in one query with
  the existing `Db::lines_of_orders(&ids)` (`WHERE order_id = ANY($1)`,
  api/src/db.rs:30-33), then sum the lines per `order_id` (e.g. into a
  HashMap) and build the summaries in the original order of `orders`, keeping
  a 0 total for an order without lines. As written, each customer page view
  runs 1 + N queries — up to 101 for a customer with 100 orders — so the cost
  grows linearly with the customer's order count on a user-facing path that
  the proposal says runs on every customer page view; the batch version runs
  2 queries whatever N is.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
