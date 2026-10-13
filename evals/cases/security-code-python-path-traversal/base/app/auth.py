"""API keys of the agents' desktop client."""

import hashlib
import secrets

from app.db import _conn


def agent_for_key(api_key: str) -> int | None:
    """The agent an API key belongs to, compared in constant time."""
    digest = hashlib.sha256(api_key.encode()).hexdigest()
    row = _conn.execute(
        "SELECT agent_id, key_sha256 FROM api_keys WHERE key_prefix = ?",
        (api_key[:8],),
    ).fetchone()
    if row is None or not secrets.compare_digest(row[1], digest):
        return None
    return row[0]


def may_read_ticket(agent_id: int, ticket_id: int) -> bool:
    """Whether the agent's team owns the ticket."""
    row = _conn.execute(
        "SELECT 1 FROM tickets t JOIN agents a ON a.team_id = t.team_id WHERE t.id = ? AND a.id = ?",
        (ticket_id, agent_id),
    ).fetchone()
    return row is not None
