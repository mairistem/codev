import { useState } from "react";
import { SortMenu } from "./SortMenu";

export interface Product {
  id: number;
  name: string;
  priceCents: number;
  addedOn: string;
}

const ORDER: Record<string, (a: Product, b: Product) => number> = {
  Newest: (a, b) => b.addedOn.localeCompare(a.addedOn),
  Oldest: (a, b) => a.addedOn.localeCompare(b.addedOn),
  "Price: low to high": (a, b) => a.priceCents - b.priceCents,
  "Price: high to low": (a, b) => b.priceCents - a.priceCents,
};

/** The product list of a category page. */
export function ProductList({ products }: { products: Product[] }) {
  const [sort, setSort] = useState("Newest");
  const sorted = [...products].sort(ORDER[sort]);
  return (
    <section aria-labelledby="products-title">
      <h2 id="products-title">Products</h2>
      <SortMenu value={sort} onChange={setSort} />
      <ul>
        {sorted.map((p) => (
          <li key={p.id}>
            {p.name} — {(p.priceCents / 100).toFixed(2)} €
          </li>
        ))}
      </ul>
    </section>
  );
}
