import { query } from "./db";
import { onPaymentEvent, type PaymentEvent } from "./events";

const MAX_ATTEMPTS = 5;

/** Delivers an event to a merchant's webhook URL; gives up after five attempts. */
export async function deliver(url: string, event: PaymentEvent): Promise<boolean> {
  const body = JSON.stringify(event);
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
    try {
      const response = await fetch(url, {
        method: "POST",
        headers: { "content-type": "application/json", "x-event-id": event.id },
        body,
        signal: AbortSignal.timeout(5000),
      });
      if (response.ok) {
        return true;
      }
    } catch (error) {
      console.debug(`webhook ${event.id} attempt ${attempt} failed`, error);
    }
  }
  console.debug(`webhook ${event.id} dropped after ${MAX_ATTEMPTS} attempts`);
  return false;
}

onPaymentEvent(async (event) => {
  const rows = await query<{ webhook_url: string }>(
    "SELECT m.webhook_url FROM merchants m JOIN payments p ON p.merchant_id = m.id WHERE p.id = $1",
    [event.data.paymentId],
  );
  if (rows.length > 0) {
    await deliver(rows[0].webhook_url, event);
  }
});
