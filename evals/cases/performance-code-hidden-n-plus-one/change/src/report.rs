//! The monthly invoice report: one line per invoice, with its customer.

use regex::Regex;

use crate::db::{Db, DbError, Invoice};

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

/// The report of `month` (`YYYY-MM`), in invoice-number order.
pub fn monthly_report(db: &dyn Db, month: &str) -> Result<Report, DbError> {
    let invoices = db.invoices_of_month(month)?;
    let mut lines = Vec::with_capacity(invoices.len());
    for invoice in &invoices {
        lines.push(ReportLine {
            number: invoice.number.clone(),
            customer: customer_label(db, invoice)?,
            total_cents: invoice.total_cents,
            purchase_order: purchase_order(&invoice.notes),
        });
    }
    Ok(Report {
        lines,
        total_cents: db.month_total(month)?,
    })
}

/// "Name (country)" of the invoice's customer.
fn customer_label(db: &dyn Db, invoice: &Invoice) -> Result<String, DbError> {
    let customer = db.customer(invoice.customer_id)?;
    Ok(format!("{} ({})", customer.name, customer.country))
}

/// The purchase-order number a customer wrote in the invoice's notes, such as `PO-2026-0042`.
fn purchase_order(notes: &str) -> Option<String> {
    let pattern = Regex::new(r"PO-\d{4}-\d{4}").expect("a valid pattern");
    pattern.find(notes).map(|m| m.as_str().to_string())
}
