//! Imports accounts from the partner's CSV export.

use crate::db::{Db, DbError};

pub struct Row {
    pub email: String,
    pub plan: String,
    pub seats: Vec<String>,
}

/// Imports the rows and returns how many seats were created.
pub fn import(db: &Db, rows: &[Row]) -> Result<usize, DbError> {
    let mut created = 0;
    for row in rows {
        if !row.email.is_empty() {
            match db.find_account(&row.email)? {
                Some(account) => {
                    if account.plan != row.plan {
                        for seat in &row.seats {
                            if !account.seats.contains(seat) {
                                if seat.contains('@') {
                                    db.add_seat(account.id, seat)?;
                                    created += 1;
                                }
                            }
                        }
                    }
                }
                None => {
                    let id = db.create_account(&row.email, &row.plan)?;
                    for seat in &row.seats {
                        if seat.contains('@') {
                            db.add_seat(id, seat)?;
                            created += 1;
                        }
                    }
                }
            }
        }
    }
    Ok(created)
}
