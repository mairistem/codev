"""Loans: recording them and finding the late ones."""

import sqlite3
from dataclasses import dataclass
from datetime import date, timedelta

LOAN_DAYS = 21
DB_PATH = "library.db"


@dataclass
class OverdueLoan:
    title: str
    member: str
    due: date
    days_late: int


def connect(path: str = DB_PATH) -> sqlite3.Connection:
    db = sqlite3.connect(path)
    db.execute(
        "CREATE TABLE IF NOT EXISTS loans ("
        " id INTEGER PRIMARY KEY, title TEXT, member TEXT,"
        " started TEXT, due TEXT, returned TEXT)"
    )
    return db


def record(db: sqlite3.Connection, title: str, member: str, started: date) -> date:
    due = started + timedelta(days=LOAN_DAYS)
    db.execute(
        "INSERT INTO loans (title, member, started, due) VALUES (?, ?, ?, ?)",
        (title, member, started.isoformat(), due.isoformat()),
    )
    return due


def overdue(today: date, db: sqlite3.Connection | None = None) -> list[OverdueLoan]:
    db = db or connect()
    rows = db.execute(
        "SELECT title, member, due FROM loans"
        " WHERE returned IS NULL AND due < ? ORDER BY due",
        (today.isoformat(),),
    ).fetchall()
    return [
        OverdueLoan(title, member, date.fromisoformat(due), (today - date.fromisoformat(due)).days)
        for title, member, due in rows
    ]
