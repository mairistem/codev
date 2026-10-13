"""Closing an account: the member's "Delete my account" button."""

import hashlib
import os

from app.db import _conn

_SALT = os.environ["ANALYTICS_SALT"]


def analytics_id(user_id: int) -> str:
    """The pseudonymous id analytics events carry instead of the user id."""
    return hashlib.sha256(f"{_SALT}:{user_id}".encode()).hexdigest()


def delete_account(user_id: int) -> None:
    """Deletes the member's account; their orders are kept for accounting."""
    _conn.execute(
        "UPDATE users SET deleted_at = datetime('now'), password_hash = NULL WHERE id = ?",
        (user_id,),
    )
    _conn.execute("DELETE FROM sessions WHERE user_id = ?", (user_id,))
    _conn.commit()
