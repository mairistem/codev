/** Structured logs, one JSON object per line, shipped to the log platform. */
type Fields = Record<string, string | number | boolean>;

function write(level: string, message: string, fields: Fields): void {
  console.log(JSON.stringify({ level, message, time: new Date().toISOString(), ...fields }));
}

export const logger = {
  info: (message: string, fields: Fields = {}) => write("info", message, fields),
  warn: (message: string, fields: Fields = {}) => write("warn", message, fields),
  error: (message: string, fields: Fields = {}) => write("error", message, fields),
};
