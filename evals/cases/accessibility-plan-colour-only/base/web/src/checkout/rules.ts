/** Validation rules of the checkout form: each returns a message, or null. */

export function postcodeError(value: string): string | null {
  return /^[0-9]{5}$/.test(value.trim()) ? null : "Enter a 5-digit postcode, such as 75011.";
}

export function nameError(value: string): string | null {
  return value.trim() === "" ? "Enter the name on the parcel." : null;
}
