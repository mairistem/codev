import { query } from "./db";

export interface Invoice {
  number: string;
  issuedOn: string;
  customer: string;
  totalCents: number;
}

/** The invoices issued in a month (`YYYY-MM`), oldest first. */
export function invoicesOfMonth(month: string): Promise<Invoice[]> {
  return query<Invoice>(
    `SELECT number, issued_on AS "issuedOn", customer, total_cents AS "totalCents"
       FROM invoices WHERE to_char(issued_on, 'YYYY-MM') = $1 ORDER BY issued_on`,
    [month],
  );
}
