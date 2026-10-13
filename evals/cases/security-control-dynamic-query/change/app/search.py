"""Ticket search for the support console."""

from app.db import _conn

# The filters the console offers, each with its SQL condition.
FILTERS = {
    "status": "status = ?",
    "assignee": "assignee_id = ?",
    "priority": "priority = ?",
}

# The sort orders the console offers.
ORDERS = {
    "newest": "created_at DESC",
    "oldest": "created_at ASC",
    "priority": "priority DESC, created_at ASC",
}

MAX_RESULTS = 200


def search_tickets(filters: dict[str, str], order: str = "newest", limit: int = 50) -> list[tuple]:
    """Tickets matching every filter; an unknown filter or order is refused."""
    clauses, values = [], []
    for name, value in sorted(filters.items()):
        if name not in FILTERS:
            raise ValueError(f"unknown filter: {name!r}")
        clauses.append(FILTERS[name])
        values.append(value)
    if order not in ORDERS:
        raise ValueError(f"unknown order: {order!r}")
    where = " AND ".join(clauses) or "1 = 1"
    sql = f"SELECT id, subject, status FROM tickets WHERE {where} ORDER BY {ORDERS[order]} LIMIT ?"
    values.append(max(1, min(limit, MAX_RESULTS)))
    return _conn.execute(sql, values).fetchall()
