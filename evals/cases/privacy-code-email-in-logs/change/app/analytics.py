"""Product analytics, sent to the Beacon SaaS (beacon.example.com)."""

import requests

BEACON_URL = "https://beacon.example.com/v1/events"


def track(event: str, properties: dict) -> None:
    requests.post(BEACON_URL, json={"event": event, "properties": properties}, timeout=2)
