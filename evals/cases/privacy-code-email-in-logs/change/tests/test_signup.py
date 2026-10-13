from unittest import mock

from app import signup


def test_signup_returns_the_new_id():
    with mock.patch.object(signup, "insert_user", return_value=7), mock.patch.object(signup, "track"):
        assert signup.signup("ada@example.org", "Ada") == 7
