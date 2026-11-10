import pytest

from rendezvous.errors import SlotTaken, UnknownSlot
from rendezvous.slots import Agenda


@pytest.fixture
def agenda() -> Agenda:
    return Agenda(free={"09:00", "09:30"})


def test_booking_takes_the_slot(agenda: Agenda) -> None:
    agenda.book("09:00", "p1")
    assert agenda.booked == {"09:00": "p1"}


@pytest.mark.parametrize("slot, error", [("09:00", SlotTaken), ("10:00", UnknownSlot)])
def test_booking_refuses(agenda: Agenda, slot: str, error: type) -> None:
    agenda.book("09:00", "p1")
    with pytest.raises(error):
        agenda.book(slot, "p2")
