# Library loans

A small web app for the municipal library of Saint-Aubin: the librarians
record the books members borrow and see which loans are late.

    pip install -e .[dev]
    flask --app library.app run
    pytest

The data lives in a SQLite file, `library.db`, created on first start.
