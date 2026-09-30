# Proposal: English-first product, artifacts in the team's language, bilingual documentation

## Why

codev is going public as an open-source project, but everything it produced
was in French: CLI output, skills, templates, code comments and the manual.
That limits its audience to French speakers, and a single-file manual in
French does not meet the bar of a public project. Now is the time to switch,
before anyone depends on the French output.

At the same time, the people who read and approve the artifacts — proposal,
design, tasks, specs — are the team using codev, whose language may not be
English. The product speaks English; the artifacts must speak the team's
language.

## What Changes

- **BREAKING** — The tool speaks English only: CLI messages and help,
  skills, templates, generated `config.yaml`. Template headings are English
  (`## Why`, `## Decisions`, `### Decision: <title>`…) and parsers only
  recognize them. `codev init --preset` takes `full|minimal|custom`, and the
  decision origin qualifier is `project` (`project/NNNN`). There is no
  compatibility with the French forms: no project depends on them yet.
- New `language:` key in `_codev/config.yaml`: the language of the prose
  skills write in artifacts. `codev init` writes it from `--language`, the
  locale, or `en`; `codev instructions` exposes it; propose, update and
  configure write in it while structural keywords stay English.
- The "full" init preset installs all eight workflows, `configure` included.
- The documentation becomes an mdBook in English and French with identical
  structure, published as a website; `codev docs --lang en|fr` renders either
  as a single self-contained page. The release archives ship `README.md`,
  `README.fr.md`, `LICENSE` and `CHANGELOG.md`.
- Fixes found while documenting every command for real: `codev sync`
  validates like `archive`; both refuse with the code `validation_failed`;
  RENAMED accepts the template's backticked form and an unknown rename source
  is an error; unexpected headings and empty sections in deltas are
  reported.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `docs` — two languages, `--lang`, chapters from the book's table of
  contents.
- `distribution` — archive contents; installation paths now documented in
  the Installation chapter of each language.
- `init` — the `language:` key and `--language`.
- `skills` — skills write artifact prose in the configured language.
- `spec-merge` — sync runs the same validation pre-flight as archive.
- `spec-parsing` — backticked RENAMED form; unexpected headings and empty
  delta sections.
- `validation` — a delta is checked against its main spec.

### Removed Capabilities

None.

## Impact

- All four crates, the eight skills, the templates and the schema.
- JSON contract v1: `language` added to instructions (additive); sync and
  archive refusals carry the code `validation_failed`.
- Documentation: `docs/codev.md` is replaced by `docs/en/` and `docs/fr/`;
  README, CONTRIBUTING and CHANGELOG are rewritten, with French editions of
  README and CONTRIBUTING.
- CI: new workflows for tests, lint, MSRV, dogfood validation, audit and the
  documentation site.
