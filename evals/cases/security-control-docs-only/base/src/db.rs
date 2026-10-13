//! Database access. Every query binds its values as parameters.

pub struct Db;

#[derive(Debug)]
pub struct DbError(pub String);
