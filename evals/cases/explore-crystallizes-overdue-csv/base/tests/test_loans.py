from datetime import date

from library import loans


def test_a_loan_is_due_three_weeks_later(tmp_path):
    db = loans.connect(str(tmp_path / "t.db"))
    assert loans.record(db, "Dune", "Ana", date(2026, 3, 1)) == date(2026, 3, 22)


def test_the_most_overdue_loan_comes_first(tmp_path):
    db = loans.connect(str(tmp_path / "t.db"))
    loans.record(db, "Dune", "Ana", date(2026, 3, 1))
    loans.record(db, "Emma", "Léo", date(2026, 2, 20))
    late = loans.overdue(date(2026, 4, 1), db)
    assert [l.title for l in late] == ["Emma", "Dune"]
