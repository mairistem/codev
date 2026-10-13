"""The carrier's pickup API: a pickup is booked for the warehouse, by order reference."""

import os

import urllib.request

_URL = os.environ.get("CARRIER_URL", "https://api.carrier.example/pickups")


def book_pickup(order_id: int, parcels: int) -> str:
    """Books a pickup of `parcels` parcels at the warehouse; returns the carrier's booking id.

    The carrier reads the delivery address from the label the warehouse prints;
    only the order's reference and parcel count are sent here.
    """
    body = f'{{"reference": "{order_id}", "parcels": {parcels}}}'.encode()
    request = urllib.request.Request(_URL, data=body, headers={"content-type": "application/json"})
    with urllib.request.urlopen(request, timeout=10) as response:
        return response.read().decode()
