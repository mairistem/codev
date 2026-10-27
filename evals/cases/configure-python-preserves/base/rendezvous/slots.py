"""Les créneaux d'un cabinet et leur réservation."""

from dataclasses import dataclass, field

from .errors import SlotTaken, UnknownSlot


@dataclass
class Agenda:
    free: set[str] = field(default_factory=set)
    booked: dict[str, str] = field(default_factory=dict)

    def book(self, slot: str, patient: str) -> None:
        if slot in self.booked:
            raise SlotTaken(slot)
        if slot not in self.free:
            raise UnknownSlot(slot)
        self.free.remove(slot)
        self.booked[slot] = patient
