//! The daily sales summary, printed by the `orders report` command.

pub struct DailySales {
    pub day: String,
    pub order_count: u32,
    pub total_cents: i64,
}

pub fn print(rows: &[DailySales]) {
    for row in rows {
        println!("{}\t{}\t{}", row.day, row.order_count, row.total_cents);
    }
}
