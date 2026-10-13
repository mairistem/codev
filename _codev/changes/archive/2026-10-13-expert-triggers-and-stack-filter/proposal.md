# Proposal: Expert activation that cannot silently miss, and knowledge that fits the stack

## Why

An expert reviews a change only when its proposal ticks the expert's
domain, and that tick is a judgement made once, before the code exists: a
change that ends up adding a login form, a migration or a Dockerfile
without its domain ticked is never seen by the expert, and nothing says
so. And every active expert receives its pitfalls for five languages, four
of which a typical project does not use — noise the reviewer must sort.

## What Changes

- **Triggers** in the frontmatter of an expert's role file: groups of
  `paths` (globs), `manifests` (dependency manifest file names) and
  `patterns` (regular expressions over added lines), each group naming the
  domain it suggests. The six built-in experts get realistic triggers
  (authentication code, secrets and dependency manifests for security;
  personal-data fields for privacy; templates, components and style sheets
  for user interface; queries and concurrency for performance; deployment,
  CI and infrastructure files for operations; migrations, schemas and API
  definitions for migration).
- An optional project-level `triggers:` key in `_codev/config.yaml`, same
  shape, for the paths only the team knows to be sensitive. Project only;
  an inherited source's `triggers:` is not read.
- **The change's diff**, defined deterministically through git: the
  changes since the merge-base with the default branch (`origin/HEAD`,
  else `main`, else `master`), staged and unstaged changes, and untracked
  files, limited to the project's directory. Planning files, vendored,
  generated, lock and binary files are left out by a documented list;
  the scan is capped in files and bytes and says when a cap was reached.
  Outside git, or without a base, there is no diff — reported, never a
  failure.
- **Suggested domains**: `codev roles --change <name>` returns the diff's
  summary and, per domain a trigger matched, its state — `ticked`,
  `justified` (left unticked with a reason on its line,
  `- [ ] security — reason`) or `unticked` — with the triggers and files
  that matched. `codev validate` reports each `unticked` suggestion as
  the warning `domain_suggested`, naming the trigger and the file.
  `codev validate` also checks the project's `triggers:` (`invalid_trigger`).
- **Stack filter**: the `By stack` part of each role's knowledge served by
  `codev roles` keeps only the languages detected in the project — every
  stack of a monorepo; when none is detected, everything is kept and
  `codev roles` says so. `codev roles --all-stacks` serves every stack.
- `/codev-propose`: the product owner reads the suggestions before
  writing `## Domains` and ticks or justifies each; the skeptic records a
  finding for every suggestion left `unticked`. `/codev-apply`: a domain
  still `unticked` at the review goes under "To decide". Before the code
  exists the diff mostly holds planning files, which are left out: the
  review, once the code is written, is where a missed domain is caught.

## Capabilities

### New Capabilities

- `domain-suggestions` — triggers, the change's diff, its exclusions and
  caps, the suggestions and their states, the project's `triggers:`.

### Modified Capabilities

- `roles` — `codev roles` serves the stacks, the diff and the
  suggestions, filters `By stack`, and gains `--all-stacks`; `triggers`
  is a known configuration key.
- `validation` — `domain_suggested` and `invalid_trigger`; a justification
  on an unticked domain.
- `skills` — the product owner and the skeptic use the suggestions;
  `/codev-apply` lists an unticked suggestion under "To decide".
- `docs` — the Roles chapter documents triggers, suggestions and the stack
  filter.

### Removed Capabilities

None.

## Impact

- `assets/roles/*.md` (the six experts' `triggers`),
  `assets/workflows/propose.md`, `apply.md`, `_review-record.md`; the
  regenerated skills under `.claude/skills/`.
- `codev-core`: `roles` (triggers in the frontmatter, `By stack` filter),
  new `triggers` module (trigger parsing, noise list, matching,
  suggestions, codes), `domains` (justifications), `detect::stack`
  (languages from manifest names).
- `codev-engine`: new `diff` module (git through `ProcessRunner`), stack
  detection over the project's directories, suggestions for a change,
  `config` (`triggers:`), `ports` (`FileSystem::is_regular_file`).
- `codev-cli`: `codev roles --all-stacks`, `RolesV1` (`stacks`, `diff`,
  `suggestions`, additive), `codev validate` (suggestions per active
  change, `invalid_trigger` on the configuration), human rendering.
- Dependencies: `regex-automata`, already in the tree through `globset`,
  becomes a direct dependency of `codev-core`; no new crate.
- Documentation, `docs/en` and `docs/fr`: `roles.md`,
  `reference/configuration.md`, `reference/cli.md`,
  `reference/json-output.md`, `reference/file-formats.md`, `skills.md`.
- `codev validate --strict` fails on a branch whose diff suggests a domain
  the proposal neither ticks nor justifies.

## Domains

- [x] security — runs git, reads untracked files, compiles project regexes
- [ ] personal-data — triggers name personal-data fields, the change stores none
- [ ] user-interface — no interface, the human output gains lines
- [x] performance — every `codev roles --change` scans the diff
- [ ] operations — no deployment, log or CI change; git calls are local
- [x] migration — a new configuration key and new JSON fields

## Release note

- Added: Experts carry triggers — file paths, dependency manifests and
  code patterns — and `codev roles --change` reports the domains the
  change's diff suggests; `codev validate` warns (`domain_suggested`)
  when a suggested domain is neither ticked nor kept unticked with a
  reason on its line (`- [ ] security — reason`).
- Added: A `triggers:` key in `_codev/config.yaml` adds the project's own
  sensitive paths and patterns. An earlier codev ignores it with an
  `unknown_config_key` warning, which fails its `codev validate
  --strict`: raise `codev_version` to this release when you add it.
- Changed: The `By stack` part of a role's knowledge keeps only the
  languages detected in the project; `codev roles --all-stacks` serves
  every language.
- Changed: `/codev-propose` ticks or justifies each suggested domain, and
  `/codev-apply` asks you to decide a suggested domain still unticked.
  Since `domain_suggested` is a warning, `codev validate --strict` fails
  until each suggestion is ticked or justified.
