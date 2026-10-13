//! Database access. Each method runs one SQL query and binds its values.

#[derive(Debug)]
pub struct DbError(pub String);

pub struct Invoice {
    pub id: i64,
    pub customer_id: i64,
    pub number: String,
    pub total_cents: i64,
    pub notes: String,
}

pub struct Customer {
    pub id: i64,
    pub name: String,
    pub country: String,
}

pub trait Db {
    /// `SELECT id, customer_id, number, total_cents, notes FROM invoices WHERE month = $1 ORDER BY number`
    fn invoices_of_month(&self, month: &str) -> Result<Vec<Invoice>, DbError>;
    /// `SELECT id, name, country FROM customers WHERE id = $1`
    fn customer(&self, id: i64) -> Result<Customer, DbError>;
    /// `SELECT id, name, country FROM customers WHERE id = ANY($1)`
    fn customers(&self, ids: &[i64]) -> Result<Vec<Customer>, DbError>;
    /// `SELECT coalesce(sum(total_cents), 0) FROM invoices WHERE month = $1`
    fn month_total(&self, month: &str) -> Result<i64, DbError>;
}
