"""Sign-up of new members."""

import logging

from app.analytics import track
from app.db import insert_user

log = logging.getLogger(__name__)


def signup(email: str, display_name: str) -> int:
    """Creates the member and returns their id."""
    user_id = insert_user(email=email, display_name=display_name)
    log.info("new signup email=%s name=%s id=%s", email, display_name, user_id)
    track("signup", {"user_id": user_id, "email": email})
    return user_id
