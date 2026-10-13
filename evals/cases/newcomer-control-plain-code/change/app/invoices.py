"""Due dates of invoices."""

from datetime import date, timedelta

PAYMENT_TERMS_DAYS = 30


def due_date(issued_on: date) -> date:
    """An invoice is due thirty days after it is issued."""
    return issued_on + timedelta(days=PAYMENT_TERMS_DAYS)


def is_overdue(issued_on: date, today: date) -> bool:
    """Overdue from the day after the due date."""
    return today > due_date(issued_on)
