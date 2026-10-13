"""Fulfilment: books a pickup for each paid order, and says what it did."""

import logging
import time

from app.carrier import book_pickup
from app.db import _conn

log = logging.getLogger(__name__)


def ship_paid_orders(batch_size: int = 100) -> int:
    """Books a pickup for each paid order of the batch; returns how many were shipped."""
    started = time.monotonic()
    rows = _conn.execute(
        "SELECT id, parcel_count FROM orders WHERE status = 'paid' ORDER BY id LIMIT ?",
        (batch_size,),
    ).fetchall()
    shipped = 0
    for order_id, parcels in rows:
        booking = book_pickup(order_id=order_id, parcels=parcels)
        _conn.execute(
            "UPDATE orders SET status = 'shipped', pickup_booking = ? WHERE id = ?",
            (booking, order_id),
        )
        log.info("order shipped order_id=%s parcels=%s", order_id, parcels)
        shipped += 1
    _conn.commit()
    log.info(
        "fulfilment batch done orders=%s shipped=%s duration_ms=%d",
        len(rows),
        shipped,
        (time.monotonic() - started) * 1000,
    )
    return shipped
