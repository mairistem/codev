export interface Invoice {
  id: number;
  customer: string;
  amountCents: number;
}

// In memory: the accounting team exports the list every evening.
export class InvoiceStore {
  private invoices: Invoice[] = [];

  list(): Invoice[] {
    return [...this.invoices];
  }

  add(input: Omit<Invoice, "id">): Invoice {
    const invoice = { id: this.invoices.length + 1, ...input };
    this.invoices.push(invoice);
    return invoice;
  }
}
