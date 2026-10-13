"""Sign-in: a deleted account cannot sign in."""

from app.db import _conn


def can_sign_in(email: str) -> bool:
    row = _conn.execute(
        "SELECT 1 FROM users WHERE email = ? AND deleted_at IS NULL AND password_hash IS NOT NULL",
        (email,),
    ).fetchone()
    return row is not None
