# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One order-lines query per order (N+1) on every customer page view

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
  for order in orders {
      let lines = db.lines_of_order(order.id)?;
recommendation: Collect the order ids and fetch every line in one query with the existing `Db::lines_of_orders(&[i64])` (`WHERE order_id = ANY($1)`, api/src/db.rs:30-31), then group the lines by `order_id` (e.g. a `HashMap<i64, i64>` of totals) and build the summaries in the original order of `orders`, keeping an order with no line at a total of 0. This turns up to 101 round trips per request into 2.
history:
  - status: open
    by: performance
```

`order_summaries` runs on every customer page view (proposal, `## Domains`): for a customer with 100 orders, each request issues 1 query for the orders plus 100 sequential `lines_of_order` queries, so the database round trips grow linearly with the customer's orders on a user-facing path, while the batch query already exists in `api/src/db.rs`.

### newcomer

Disabled by configuration.
