//! The daily sales summary, printed by the `orders report` command.

pub struct SalesRow {
    pub day: String,
    pub count: u32,
    pub total_cents: i64,
}

pub fn print(rows: &[SalesRow]) {
    for row in rows {
        println!("{}\t{}\t{}", row.day, row.count, row.total_cents);
    }
}
