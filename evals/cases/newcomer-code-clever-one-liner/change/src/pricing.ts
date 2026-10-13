/** Prices of the catalogue, in cents. */

export interface Price {
  sku: string;
  cents: number;
}

/** Splits prices into the cheap and the premium ones. */
export const bucket = (ps: Price[]) =>
  ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});

/** The catalogue's prices, in display order. */
export function sortedBySku(prices: Price[]): Price[] {
  return [...prices].sort((a, b) => a.sku.localeCompare(b.sku));
}
