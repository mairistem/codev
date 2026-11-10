# invoice-api

A small REST API that issues and lists invoices for the accounting team.

- `npm run build` compiles to `dist/`.
- `npm test` runs the Jest suite (`__tests__/`), including HTTP tests with
  supertest.
- Request bodies are validated with zod; a validation failure answers 400
  with the list of issues.
