# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

### performance

#### R-performance-1 N+1 query: one order_lines query per order

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
    for order in orders {
        let lines = db.lines_of_order(order.id)?;
recommendation: Collect the order ids and fetch every line in one query with the existing `Db::lines_of_orders(&ids)` (`WHERE order_id = ANY($1)`), then sum the totals per order id (e.g. a HashMap<i64, i64>), keeping the order of `orders` and a 0 total for orders without lines. This turns up to 101 queries per customer page view into 2.
history:
  - status: open
    by: performance
```

Cost: one database round trip per order, up to 100 extra queries on every customer page view (a user-facing path, per the proposal's `## Domains`), growing with the customer's order count up to the limit. `api/src/db.rs:31` already offers the batch query `lines_of_orders`.

### newcomer

Disabled by configuration.
