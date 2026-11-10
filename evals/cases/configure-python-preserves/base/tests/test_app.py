from fastapi.testclient import TestClient

from rendezvous.app import app


def test_booking_twice_answers_409() -> None:
    client = TestClient(app)
    assert client.post("/slots/2026-11-02T09:00/book?patient=p1").status_code == 200
    assert client.post("/slots/2026-11-02T09:00/book?patient=p2").status_code == 409
