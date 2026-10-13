import { logger } from "./logger";

export interface PaymentEvent {
  id: string;
  type: "payment.succeeded" | "payment.refunded";
  data: { paymentId: string; amountCents: number };
}

type Listener = (event: PaymentEvent) => Promise<void>;
const listeners: Listener[] = [];

export function onPaymentEvent(listener: Listener): void {
  listeners.push(listener);
}

/** Called by the payment service once a payment is settled. */
export async function publish(event: PaymentEvent): Promise<void> {
  for (const listener of listeners) {
    try {
      await listener(event);
    } catch (error) {
      logger.error("payment event listener failed", { eventId: event.id, error: String(error) });
    }
  }
}
