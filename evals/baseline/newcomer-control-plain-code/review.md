# Review record: add-invoice-due-dates

<!-- codev-review: 1 -->

## code.review

### newcomer

#### R-newcomer-1 Docstring restates the payment term instead of naming the constant

```yaml
severity: low
confidence: high
location: "app/invoices.py:5-10"
evidence: |
  PAYMENT_TERMS_DAYS = 30
  def due_date(issued_on: date) -> date:
      """An invoice is due thirty days after it is issued."""
      return issued_on + timedelta(days=PAYMENT_TERMS_DAYS)
recommendation: Have the docstring refer to PAYMENT_TERMS_DAYS (e.g. "due PAYMENT_TERMS_DAYS days after it is issued") so it cannot drift from the constant, or drop it as it only paraphrases the one-line body.
history:
  - status: open
    by: newcomer
```
