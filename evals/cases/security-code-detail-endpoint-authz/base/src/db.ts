import pg from "pg";

const pool = new pg.Pool({ connectionString: process.env.DATABASE_URL });

/** Runs a parameterized query: values go in `params`, never in `sql`. */
export async function query<T>(sql: string, params: unknown[] = []): Promise<T[]> {
  const result = await pool.query(sql, params);
  return result.rows as T[];
}
