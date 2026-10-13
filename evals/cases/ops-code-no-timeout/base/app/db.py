"""Database access for the members service."""

import sqlite3

_conn = sqlite3.connect("members.db")


def insert_user(email: str, display_name: str) -> int:
    cur = _conn.execute(
        "INSERT INTO users (email, display_name) VALUES (?, ?)",
        (email, display_name),
    )
    _conn.commit()
    return cur.lastrowid
