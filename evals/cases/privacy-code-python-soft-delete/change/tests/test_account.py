from app.account import delete_account
from app.db import _conn
from app.signin import can_sign_in


def test_a_deleted_account_cannot_sign_in():
    _conn.execute(
        "INSERT INTO users (id, email, display_name, password_hash) VALUES (1, 'ada@example.org', 'Ada', 'x')"
    )
    delete_account(1)
    assert not can_sign_in("ada@example.org")


def test_a_deleted_account_has_no_session():
    _conn.execute("INSERT INTO sessions (id, user_id) VALUES ('s1', 1)")
    delete_account(1)
    assert _conn.execute("SELECT count(*) FROM sessions WHERE user_id = 1").fetchone()[0] == 0
