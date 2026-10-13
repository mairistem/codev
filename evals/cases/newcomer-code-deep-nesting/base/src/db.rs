//! Database access. Every query binds its values as parameters.

pub struct Db;

#[derive(Debug)]
pub struct DbError(pub String);

pub struct Account {
    pub id: i64,
    pub plan: String,
    pub seats: Vec<String>,
}

impl Db {
    pub fn find_account(&self, _email: &str) -> Result<Option<Account>, DbError> {
        unimplemented!("database")
    }

    pub fn create_account(&self, _email: &str, _plan: &str) -> Result<i64, DbError> {
        unimplemented!("database")
    }

    pub fn add_seat(&self, _account: i64, _seat: &str) -> Result<(), DbError> {
        unimplemented!("database")
    }
}
