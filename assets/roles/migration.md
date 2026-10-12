---
id: migration
name: { en: Migration, fr: Migration }
at: [plan.challenge, code.review]
domains: [migration]
question: Can existing data, configuration and consumers move to this change safely?
done_when:
  - Every breaking change is marked **BREAKING** and announced in the
    release note, with what users must do
  - Existing data, configuration and API consumers have a migration path,
    tested
  - The rollout order keeps old and new versions working side by side
    when both can run at once
  - Schema migrations are reversible, or the plan says why not and how to
    recover
  - What is removed is deprecated first when consumers need time
---

## Scope

Look at what already exists outside the change: stored data, database
schemas, configuration files, file formats, public APIs, CLI flags,
environment variables and the consumers of each. Leave aside internal code
that no consumer sees, and greenfield features with no predecessor.

## Signals

- **Unmarked breaking change**: a renamed field, flag or endpoint, a
  changed default or error code, with no **BREAKING** item. Example: a JSON
  field `user_id` renamed `userId`.
- **No path for existing data**: a new required column with no default or
  backfill. Example: `ALTER TABLE … ADD COLUMN … NOT NULL` on a full
  table.
- **Big-bang rollout**: a schema change and the code that needs it shipped
  as one step, while old instances still run. Prefer expand, migrate,
  then contract.
- **Irreversible migration without a plan**: a `DROP COLUMN` or a data
  rewrite with no down migration and no backup step.
- **Configuration silently ignored**: an old key no longer read, without a
  warning. Example: `timeout` renamed `timeout_ms`, old files keep the old
  key.
- **Removal without deprecation**: a public function or flag deleted in
  the same release that introduces its replacement.
- **Long lock**: a migration that rewrites or locks a large table during
  traffic.

## By stack

- **Rust**: `serde` with `deny_unknown_fields` turns an added field into a
  breaking change for older readers; `#[serde(alias = "old")]` and
  `#[serde(default)]` keep old files readable; `#[deprecated]` before
  removal of a public item; public enums need `#[non_exhaustive]` to grow.
- **TypeScript/JavaScript**: a changed exported type or a removed export in
  a published package is a major version under semantic versioning; ORM
  migrations (Prisma, TypeORM, Knex) need a reviewed down step.
- **Python**: `DeprecationWarning` via `warnings.warn` before removal;
  Django and Alembic migrations: separate schema from data migrations, and
  make data migrations reversible or say why not.
- **Go**: a changed exported signature breaks importers — a new major
  version needs a `/v2` module path; `// Deprecated:` comments are
  recognized by tooling.
- **Java**: `@Deprecated(since = …, forRemoval = true)`; Flyway or
  Liquibase migrations are applied once — never edit an applied one, add a
  new one.

## References

- Semantic Versioning 2.0.0 — what makes a change major.
- Keep a Changelog — how to announce removed and changed behavior.
- The expand and contract pattern (also called parallel change) for
  zero-downtime schema and API changes.

## Reporting

One line per finding: what existing thing breaks (data, configuration,
consumer), who notices and when, the severity (high when data can be lost
or consumers break without warning; medium when a manual step is needed and
documented; low otherwise), and the fix — a migration step, a
compatibility shim, a deprecation, or a release note item.
