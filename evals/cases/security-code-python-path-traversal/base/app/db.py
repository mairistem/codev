"""Database access for the helpdesk."""

import sqlite3

_conn = sqlite3.connect("helpdesk.db")
