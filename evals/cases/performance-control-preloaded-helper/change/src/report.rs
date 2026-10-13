//! The monthly invoice report: one line per invoice, with its customer.

use std::collections::HashMap;
use std::sync::OnceLock;

use regex::Regex;

use crate::db::{Customer, Db, DbError, Invoice};

pub struct ReportLine {
    pub number: String,
    pub customer: String,
    pub total_cents: i64,
    pub purchase_order: Option<String>,
}

pub struct Report {
    pub lines: Vec<ReportLine>,
    pub total_cents: i64,
}

/// The report of `month` (`YYYY-MM`), in invoice-number order: three
/// queries whatever the number of invoices.
pub fn monthly_report(db: &dyn Db, month: &str) -> Result<Report, DbError> {
    let invoices = db.invoices_of_month(month)?;
    let mut ids: Vec<i64> = invoices.iter().map(|i| i.customer_id).collect();
    ids.sort_unstable();
    ids.dedup();
    let customers: HashMap<i64, Customer> = db
        .customers(&ids)?
        .into_iter()
        .map(|c| (c.id, c))
        .collect();
    let mut lines = Vec::with_capacity(invoices.len());
    for invoice in &invoices {
        lines.push(ReportLine {
            number: invoice.number.clone(),
            customer: customer_label(&customers, invoice),
            total_cents: invoice.total_cents,
            purchase_order: purchase_order(&invoice.notes),
        });
    }
    Ok(Report {
        lines,
        total_cents: db.month_total(month)?,
    })
}

/// "Name (country)" of the invoice's customer, from the customers loaded
/// for the month.
fn customer_label(customers: &HashMap<i64, Customer>, invoice: &Invoice) -> String {
    match customers.get(&invoice.customer_id) {
        Some(c) => format!("{} ({})", c.name, c.country),
        None => format!("unknown customer {}", invoice.customer_id),
    }
}

/// The purchase-order number a customer wrote in the invoice's notes, such as `PO-2026-0042`.
fn purchase_order(notes: &str) -> Option<String> {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    let pattern = PATTERN.get_or_init(|| Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern"));
    pattern.find(notes).map(|m| m.as_str().to_string())
}
