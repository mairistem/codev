export interface OrderSummary {
  id: number;
  totalCents: number;
}

export async function fetchOrderSummaries(customerId: number): Promise<OrderSummary[]> {
  const response = await fetch(`/api/customers/${customerId}/order-summaries`);
  return response.json();
}
