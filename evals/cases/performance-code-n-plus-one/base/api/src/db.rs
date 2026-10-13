//! Database access. Each method runs one query.

pub struct Db;

#[derive(Debug)]
pub struct DbError(pub String);

pub struct Order {
    pub id: i64,
    pub customer_id: i64,
}

pub struct OrderLine {
    pub order_id: i64,
    pub quantity: i64,
    pub unit_price_cents: i64,
}

impl Db {
    /// `SELECT id, customer_id FROM orders WHERE customer_id = $1 ORDER BY id DESC LIMIT $2`
    pub fn orders_of_customer(&self, _customer_id: i64, _limit: i64) -> Result<Vec<Order>, DbError> {
        unimplemented!("database")
    }

    /// `SELECT order_id, quantity, unit_price_cents FROM order_lines WHERE order_id = $1`
    pub fn lines_of_order(&self, _order_id: i64) -> Result<Vec<OrderLine>, DbError> {
        unimplemented!("database")
    }

    /// `SELECT ... FROM order_lines WHERE order_id = ANY($1)`
    pub fn lines_of_orders(&self, _order_ids: &[i64]) -> Result<Vec<OrderLine>, DbError> {
        unimplemented!("database")
    }
}
