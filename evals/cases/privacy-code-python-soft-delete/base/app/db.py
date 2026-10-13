"""Database access for the members service."""

import sqlite3

_conn = sqlite3.connect("members.db")
