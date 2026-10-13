import { query } from "./db";

export interface UserRow {
  id: number;
  name: string;
}

/** Users whose name contains `term`, sorted by name. */
export async function searchUsers(term: string): Promise<UserRow[]> {
  const sql = "SELECT id, name FROM users WHERE name LIKE '%" + term + "%' ORDER BY name LIMIT 50";
  return query<UserRow>(sql);
}
