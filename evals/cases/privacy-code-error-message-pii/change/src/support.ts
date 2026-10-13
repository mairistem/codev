import { query } from "./db";
import { NotFoundError } from "./errors";

export interface Customer {
  id: number;
  email: string;
  name: string;
  phone: string;
  birth_date: string;
  address: string;
}

/** The customer whose e-mail address a support agent typed. */
export async function customerByEmail(email: string): Promise<Customer> {
  const rows = await query<Customer>(
    "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
    [email],
  );
  if (rows.length === 0) {
    throw new NotFoundError(`no customer with e-mail ${email}`);
  }
  return rows[0];
}

/** The signed-in customer's record. */
export async function customerById(id: number): Promise<Customer> {
  const rows = await query<Customer>(
    "SELECT id, email, name, phone, birth_date, address FROM customers WHERE id = $1",
    [id],
  );
  if (rows.length === 0) {
    throw new NotFoundError(`no customer ${id}`);
  }
  return rows[0];
}

/** What the Helply chat widget is started with, for the signed-in customer. */
export function chatWidgetSettings(customer: Customer) {
  return {
    appId: process.env.HELPLY_APP_ID,
    user: {
      id: String(customer.id),
      name: customer.name,
      email: customer.email,
      phone: customer.phone,
      birthDate: customer.birth_date,
      address: customer.address,
    },
  };
}
