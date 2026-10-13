"""Charging orders through the payment provider."""

import time

import requests

PAYMENT_URL = "https://payments.example.com/v2/charges"


def charge(order_id: str, amount_cents: int) -> dict:
    """Charges the order and returns the provider's receipt."""
    while True:
        try:
            response = requests.post(PAYMENT_URL, json={"order": order_id, "amount": amount_cents})
            response.raise_for_status()
            return response.json()
        except requests.RequestException:
            time.sleep(1)
