/** Structured logger that never writes a full e-mail address. */

const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;

/** `ada.lovelace@example.org` becomes `a***@example.org`. */
export function maskEmails(text: string): string {
  return text.replace(EMAIL, "$1***@$2");
}

export function logInfo(message: string, fields: Record<string, string | number> = {}): void {
  const safe: Record<string, string | number> = {};
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
  }
  console.log(JSON.stringify({ level: "info", message: maskEmails(message), ...safe }));
}
