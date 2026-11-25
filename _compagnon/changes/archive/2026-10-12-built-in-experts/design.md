# Design: Built-in experts with their own knowledge, activated by the domains a change touches

## Context

The `declarative-roles` change (archived 2026-10-12) made roles data:
`assets/roles.yaml` holds the six step roles, `codev-core::roles` parses
it, merges the schema's artifact roles, checks settings and resolves the
activation (`always` / `never`); `codev-engine::roles` reads the files and
calls the core; `codev-cli` serves `RolesV1` / `RoleV1`. That design
deferred experts and `on_domain` to this change. A role serves exactly one
step (`Role.at: String`), and the proposal is only read for its release
note (`codev-core::changelog::release_note`, checked in
`codev-engine::validate::check_release_note`).

## Goals / Non-Goals

**Goals:**

- One file per built-in role, readable as a document: data on top,
  knowledge below.
- Experts that run only where their domain is touched, with knowledge
  concrete enough to find real defects.
- Additive JSON contract.

**Non-Goals:**

- Knowledge for artifact roles, or for roles of a custom schema: a schema
  `role:` keeps its current shape.
- Custom experts or custom domains: the six domains are a closed list.
- Filtering the knowledge by detected stack in the binary: the skill reads
  the part for the project's languages.

## Decisions

### Decision: One markdown file per role, parsed by the core

`assets/roles/<id>.md` starts with a YAML frontmatter between `---` lines
(`id`, `name`, `at`, `question`, `required`, `done_when`, `domains`), then
the knowledge as markdown. `codev-core::roles` embeds the files with
`include_str!` in a fixed list, which gives the catalog order, and parses
them with a pure `parse_role_file(&str)`: split the frontmatter, parse it
with `serde_norway` (decision 0006, `deny_unknown_fields`), trim the body.
Nothing reads the disk (decision 0001); the files are embedded, so a parse
failure is a build bug caught by a unit test, as before. A test checks
that each file's `id` matches its position in the list and that every
body uses only the five headings, in order, and that each expert has all
five.

**Alternatives considered**: a `knowledge:` block scalar inside
`roles.yaml` (long markdown inside YAML loses its editor support and is
painful to review); a `build.rs` that discovers the directory (sorted by
file name, so the workflow order would need another key; one more moving
part for a list of sixteen lines).

### Decision: A role serves one or several steps; `at` stays a string in JSON

`at` in a role file takes a step or a list; `Role.at` becomes
`Role.steps: Vec<String>`. `active_at` filters on `steps.contains(step)`;
the catalog sorts by the first step, so experts follow the skeptic. In
`RoleV1`, `at` keeps its type — the step asked with `--at`, else the first
step — and `steps` is added: consumers of v1 keep working.

**Alternatives considered**: duplicating each expert per step (two ids for
one role, settings would need both); turning `at` into an array (breaks
the v1 contract).

### Decision: Domains are a closed list parsed from the proposal by the core

A new `codev-core::domains` module holds `DOMAINS` and
`parse(proposal) -> Parsed<Domains>` (`Absent`, or the ticked domains; an
explicit `None` and an all-unticked list both give an empty list), with
the codes `unknown_domain`, `domains_invalid` (both errors) and
`domains_missing` (warning, emitted by the engine as for
`release_note_missing`). The section reader of `release_note` — level-2
section, literal zones masked — moves to `parser::section` and is shared.
`codev-engine::validate` reads `proposal.md` once for both checks; archived
changes are never validated, as today. The codes join the uniqueness test
of `codev-core::validate`.

`domains_invalid` is not in the request: it separates a malformed line or
`None` next to a ticked domain from a misspelled domain, so the message can
say what to fix rather than silently picking one reading.

**Alternatives considered**: a `domains:` key in `change.yaml`
(machine-friendly but invisible to the reviewer reading the proposal, and
the product owner owns the scope); free-text domains (an expert could never
know it was meant).

### Decision: `on_domain` is an activation; experts default to it

`Activation` gains `OnDomain`. A role's default is `on_domain` when it has
domains, `always` otherwise — no extra key in the role file.
`check_settings` reports `on_domain` on a role without domains as
`invalid_role_activation`, and `required: true` on `analyst` makes
`never` a `role_not_disableable`. `resolve` takes the change's domains
(`Option<&[String]>`, `None` without a change or a proposal) and computes
`activated_by: Option<ActivatedBy>` (`Always`, `Project`, `Change`,
`Domain(name)`); `is_active` is `activated_by.is_some()`. The engine
reads the proposal of the change, if any, and ignores its findings
(`codev validate` reports them). The order of precedence stays: role
default, then project, then change (from the `declarative-roles` design).

**Alternatives considered**: a separate `domains` filter next to
`always` (two settings that can disagree); activating experts on keywords
of the Impact section (guesswork, not reviewable).

### Decision: Knowledge goes to active roles only, and to humans on request

`RoleV1` gains `steps`, `domains`, `activatedBy` and `knowledge` (the body
when the role is active and has one, else `null`). A role listed without
`--at` is served the same way, so the shape never depends on the flag. The
human output prints the knowledge only with `--knowledge`: it runs to
dozens of lines per expert. The skills keep `Bash(codev:*)` and get no new
command (decision 0004); `codev-cli` maps the domain types to the contract
(decision 0002: the core does not derive `Serialize`).

**Alternatives considered**: knowledge in a separate `codev roles
knowledge <id>` command (one more call per role, per step).

### Decision: The skills keep the procedure, the caps and the severity rule

`propose.md` step 7 applies every role returned at `plan.challenge` with
its knowledge, caps the Points to challenge at five in total, ranked by
impact, tagged `[role-id]`. `apply.md` step 5 does the same at
`code.review`; its block becomes **Review notes**, capped at five, a
high-severity `security` or `privacy` finding never dropped, and an expert
reports a behavior-changing fix unless it is a clear defect within the
change's scope (fixed with a test, named in the summary). Criteria and
knowledge stay out of the skill texts.

**Alternatives considered**: a separate cap per role (up to thirty-five
items; the user stops reading); dropping the severity exception (a cap
must never hide an injection).

### Decision: The docs test follows the catalog

`roles_chapters_list_exactly_the_catalog` already iterates over the
catalog: the experts join the table automatically. It gains a check that
an expert's row names its domain.

## Risks / Trade-offs

- [Wrong or invented references in the knowledge] → only standards whose
  identifiers are certain are cited (OWASP Top 10, ASVS, CWE without
  numbers except well-known ones, GDPR articles 5, 6, 17, 25, 28, 32, 44,
  WCAG 2.2 success criteria by name and number, RGAA 4); reviewed in the
  newcomer pass.
- [Knowledge inflates the agent's context] → served to active roles only;
  experts are inactive unless a domain is ticked.
- [`analyst: never` in an existing configuration now fails] → marked
  **BREAKING** in the release note; the message says to remove `never`.
- [Active changes without `## Domains` fail `codev validate --strict`] →
  a warning only; adding the section is one line (`None`).
- [`when: "on_domain"` surprises a consumer that only knew two values] →
  the skills only read active roles; the JSON reference documents it.

## Migration Plan

Existing active changes get `domains_missing` until their proposal gains a
`## Domains` section. Projects remove `analyst: never` if they set it.
Rollback: revert the commit; no data is migrated.

## Open Questions

None.
