from datetime import date

from app.invoices import due_date, is_overdue


def test_an_invoice_is_due_thirty_days_after_it_is_issued():
    assert due_date(date(2026, 1, 15)) == date(2026, 2, 14)


def test_an_invoice_is_not_overdue_on_its_due_date():
    assert not is_overdue(date(2026, 1, 15), today=date(2026, 2, 14))


def test_an_invoice_is_overdue_the_day_after():
    assert is_overdue(date(2026, 1, 15), today=date(2026, 2, 15))
