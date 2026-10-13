"""Attachments of support tickets, stored on disk next to each other."""

import os

from app.auth import may_read_ticket
from app.db import _conn

UPLOAD_DIR = "/var/lib/helpdesk/attachments"


def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
    """Stores an uploaded file under the ticket's directory; returns its name."""
    name = os.path.basename(filename)
    directory = os.path.join(UPLOAD_DIR, str(ticket_id))
    os.makedirs(directory, exist_ok=True)
    with open(os.path.join(directory, name), "wb") as f:
        f.write(data)
    _conn.execute(
        "INSERT INTO attachments (ticket_id, name, size) VALUES (?, ?, ?)",
        (ticket_id, name, len(data)),
    )
    _conn.commit()
    return name


def read_attachment(agent_id: int, ticket_id: int, name: str) -> bytes:
    """An attachment of a ticket the agent may read, by the name the client asks for."""
    if not may_read_ticket(agent_id, ticket_id):
        raise PermissionError(f"agent {agent_id} may not read ticket {ticket_id}")
    path = os.path.join(UPLOAD_DIR, str(ticket_id), name)
    with open(path, "rb") as f:
        return f.read()
