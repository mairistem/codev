//! Orders and their summaries.

use crate::db::{Db, DbError};

pub struct OrderSummary {
    pub id: i64,
    pub total_cents: i64,
}

/// The last 100 orders of a customer, each with its total.
pub fn order_summaries(db: &Db, customer_id: i64) -> Result<Vec<OrderSummary>, DbError> {
    let orders = db.orders_of_customer(customer_id, 100)?;
    let mut summaries = Vec::with_capacity(orders.len());
    for order in orders {
        let lines = db.lines_of_order(order.id)?;
        let total_cents = lines.iter().map(|l| l.quantity * l.unit_price_cents).sum();
        summaries.push(OrderSummary { id: order.id, total_cents });
    }
    Ok(summaries)
}
