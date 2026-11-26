from recipes import store


def test_recipes_are_listed_by_title(tmp_path):
    db = store.connect(str(tmp_path / "t.db"))
    db.execute("INSERT INTO recipes (title, ingredients) VALUES ('Tarte tatin', 'pommes'), ('Crêpes', 'oeufs, lait')")
    assert [r.title for r in store.by_title(db)] == ["Crêpes", "Tarte tatin"]
