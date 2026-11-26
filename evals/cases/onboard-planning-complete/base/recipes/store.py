"""The recipes, in a SQLite file."""

import sqlite3
from dataclasses import dataclass


@dataclass
class Recipe:
    id: int
    title: str
    ingredients: str


def connect(path: str = "cookbook.db") -> sqlite3.Connection:
    db = sqlite3.connect(path)
    db.execute("CREATE TABLE IF NOT EXISTS recipes (id INTEGER PRIMARY KEY, title TEXT, ingredients TEXT)")
    return db


def by_title(db: sqlite3.Connection) -> list[Recipe]:
    rows = db.execute("SELECT id, title, ingredients FROM recipes ORDER BY title").fetchall()
    return [Recipe(*row) for row in rows]
