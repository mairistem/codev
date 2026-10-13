import { useState } from "react";

export interface Product {
  id: number;
  name: string;
  priceCents: number;
  addedOn: string;
}

/** The product list of a category page. */
export function ProductList({ products }: { products: Product[] }) {
  const [sort] = useState("Newest");
  return (
    <section aria-labelledby="products-title">
      <h2 id="products-title">Products</h2>
      <p>Sorted by {sort.toLowerCase()}</p>
      <ul>
        {products.map((p) => (
          <li key={p.id}>
            {p.name} — {(p.priceCents / 100).toFixed(2)} €
          </li>
        ))}
      </ul>
    </section>
  );
}
