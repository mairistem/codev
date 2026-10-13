"""Checkout: a guest checkout creates a customer record for each order."""

from app.db import _conn


def guest_checkout(email: str, name: str, billing_address: str, total_cents: int) -> int:
    cur = _conn.execute(
        "INSERT INTO customers (email, name, billing_address, created_at) "
        "VALUES (?, ?, ?, datetime('now'))",
        (email, name, billing_address),
    )
    _conn.execute(
        "INSERT INTO orders (customer_id, total_cents, placed_at) VALUES (?, ?, datetime('now'))",
        (cur.lastrowid, total_cents),
    )
    _conn.commit()
    return cur.lastrowid
