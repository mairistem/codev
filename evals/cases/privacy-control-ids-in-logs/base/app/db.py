"""Database access for the warehouse service."""

import sqlite3

_conn = sqlite3.connect("warehouse.db")
