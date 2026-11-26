import pytest

from pantry.stock import Stock


def test_taking_the_last_one_empties_the_item():
    s = Stock()
    s.add("tea", 1)
    s.take("tea")
    with pytest.raises(ValueError):
        s.take("tea")
