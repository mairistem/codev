# Design: English-first product, artifacts in the team's language, bilingual documentation

## Context

Everything codev produced was French, from CLI messages to template
headings, and parsers recognized French headings. No external project
depends on those forms yet, which makes this the cheapest moment to switch.
The constraint that shapes the design: artifacts are read and approved by a
team that may not work in English, while codev must keep parsing them.

## Goals / Non-Goals

**Goals:**

- One product language, English, with no bilingual parsing to maintain.
- Artifact prose in the team's language, chosen once per repository.
- Documentation in English and French that cannot drift structurally.

**Non-Goals:**

- Localizing the CLI output or the structural keywords of artifacts.
- Translating accepted ADRs: they are sealed records (see below).

## Decisions

### Decision: Structural keywords stay English, prose follows `language:`

Headings, delta sections and `**WHEN**` / `SHALL` markers are what the
parsers read; keeping a single English set keeps the parsers simple and the
files portable between teams. Only prose is localized, like Gherkin keywords
in many projects. The language lives in `_codev/config.yaml` rather than
following the conversation: it is a team choice, versioned with the
repository, and two developers get the same result. Like `mcp`, it is not
inherited from sources — it belongs to the team writing in this repository.

**Alternatives considered**: fully localized templates with per-language
parser keywords (every added language becomes code and tests); following
the conversation language (two developers can produce two languages in the
same change).

### Decision: `language` is detected at init, never guessed later

`codev init` writes the value once, with its provenance (`--language`, the
POSIX locale precedence `LC_ALL` → `LC_MESSAGES` → `LANG`, or `en`). The
region is dropped except where it changes the written language (`pt-BR`,
`zh-Hant`, `zh-Hans`). No extra init question is asked, to keep the
"at most two questions" contract.

### Decision: One table of contents per language drives both the site and the binary

`build.rs` reads each `docs/<lang>/src/SUMMARY.md` and embeds the listed
chapters with `include_str!`, so `codev docs` and the mdBook site always
agree. A test enforces identical chapter paths across languages, which is
what keeps the two editions from drifting. Links between chapters are
rewritten to in-page anchors and headings get mdBook-style ids, so an
anchor that works on the site works on the single page. This follows the
functional-core rule of `_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md`:
the docs module only transforms strings; reading files happens at build time.

### Decision: Accepted ADRs and the project context stay as they are

Accepted decisions are sealed; rewriting their bodies to translate them
would contradict the immutability codev enforces, so they are kept in French
as dated records. `_codev/config.yaml` is left to the maintainers, since its
`context:` and `rules:` are injected into every agent prompt.

## Risks / Trade-offs

- [Breaking change for early adopters relying on French output] → No known
  external user; the CHANGELOG marks it **BREAKING**.
- [Two documentation editions to maintain] → The chapter-parity test and the
  pull request checklist require both to change together.
- [Detected locale may not be the team's writing language] → The provenance
  comment makes the source visible, and the key is a one-line edit.

## Migration Plan

None required: no project depends on the French forms. A project created
with an earlier version keeps working; without `language:`, artifacts are
written in English.

## Open Questions

- Should accepted ADRs get English translations as new, superseding
  decisions?
