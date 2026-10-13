// Fails when a field of openapi.yaml is missing from docs/api.md.
import { readFileSync } from "node:fs";

const spec = readFileSync("openapi.yaml", "utf8");
const docs = readFileSync("docs/api.md", "utf8");
const fields = [...spec.matchAll(/^ {8}(\w+):$/gm)].map((m) => m[1]);
const missing = fields.filter((f) => !docs.includes(`\`${f}\``));
if (missing.length > 0) {
  console.error(`docs/api.md does not document: ${missing.join(", ")}`);
  process.exit(1);
}
