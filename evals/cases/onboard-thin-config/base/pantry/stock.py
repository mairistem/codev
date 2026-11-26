"""What is in the pantry, item by item."""


class Stock:
    def __init__(self) -> None:
        self.items: dict[str, int] = {}

    def add(self, item: str, count: int) -> None:
        self.items[item] = self.items.get(item, 0) + count

    def take(self, item: str) -> None:
        if self.items.get(item, 0) == 0:
            raise ValueError(f"no {item} left")
        self.items[item] -= 1

    def low(self, threshold: int) -> list[tuple[str, int]]:
        return sorted((i, c) for i, c in self.items.items() if c < threshold)
