import { useState } from "react";
import { nameError, postcodeError } from "./rules";

/** The checkout form: validated on submit, errors listed above the fields. */
export function CheckoutForm({ onSubmit }: { onSubmit: (data: FormData) => void }) {
  const [errors, setErrors] = useState<string[]>([]);

  function submit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const data = new FormData(event.currentTarget);
    const found = [
      nameError(String(data.get("name") ?? "")),
      postcodeError(String(data.get("postcode") ?? "")),
    ].filter((e): e is string => e !== null);
    setErrors(found);
    if (found.length === 0) onSubmit(data);
  }

  return (
    <form onSubmit={submit} noValidate>
      {errors.length > 0 && (
        <div role="alert" className="error-summary">
          <h2>There is a problem</h2>
          <ul>
            {errors.map((e) => (
              <li key={e}>{e}</li>
            ))}
          </ul>
        </div>
      )}
      <label htmlFor="name">Name</label>
      <input id="name" name="name" autoComplete="name" />
      <label htmlFor="postcode">Postcode</label>
      <input id="postcode" name="postcode" autoComplete="postal-code" />
      <button type="submit">Pay</button>
    </form>
  );
}
