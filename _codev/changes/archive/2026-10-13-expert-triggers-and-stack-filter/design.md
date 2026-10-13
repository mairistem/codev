# Design: Expert activation that cannot silently miss, and knowledge that fits the stack

## Context

Experts are `on_domain` roles (`codev-core::roles`): active when the
proposal's `## Domains` ticks their domain (`codev-core::domains`). Role
files are embedded at build time (`assets/roles/*.md`, strict frontmatter
with `deny_unknown_fields`). `codev roles` resolves them in
`codev-engine::roles::resolve` and serves them through `RolesV1`
(`codev-cli::contract`), knowledge whole.

Git is reached only through the `ProcessRunner` port, for local read-only
queries (`codev changelog` does `rev-parse`, `tag`, `ls-tree`), with
`MockProcessRunner` in tests. `codev validate` today reads files only:
`engine::validate::validate_change` takes no runner, and is also the
pre-flight of `sync` and `archive` and the count behind `codev doctor`'s
`validate` check. Stack detection exists for `codev init`
(`codev-engine::detect`): root manifests only, first found wins.

Decisions in force: 0001 (the core decides on strings, the engine
reads), 0002 (crate graph; effects through ports), 0003 (`_codev/` is the
planning root), 0005 (inherited sources are read-only; their `roles:` is
already ignored), 0006 (`serde_norway` for YAML).

## Goals / Non-Goals

**Goals:**

- A trigger match the proposal ignores is visible in `codev roles` and
  `codev validate`, with the trigger and the file that caused it.
- Everything here is deterministic and checkable: no model, no network.
- The `By stack` knowledge an expert receives fits the project.
- Where the check bites: before the code exists, the diff holds mostly
  planning files, left out, so `/codev-propose` usually sees no
  suggestion; `/codev-apply`'s review and `codev validate` after the
  code is written are where an unticked domain is caught.

**Non-Goals:**

- Ticking domains automatically: a suggestion is evidence for a human or
  an agent to judge, never a decision.
- A diff per change: codev cannot attribute lines to one of several
  active changes; every active change shares the working tree's diff.
- Triggers for non-expert roles, or triggers that deactivate an expert.
- Detecting languages from file extensions, or frameworks.
- Suggestions in `codev doctor`, `codev status`, or the archive gate.

## Decisions

### Decision: Trigger groups in the frontmatter, the same shape in the configuration

```yaml
triggers:
  - domain: security
    paths: ["**/auth/**", "**/.env", "**/.env.*"]
    manifests: [Cargo.toml, package.json, go.mod]
    patterns: ['(?i)\b(password|passwd|api[_-]?key|private[_-]?key|access[_-]?token)\b']
```

A group names its domain, so a role file and the project's `triggers:`
read alike, and a reader sees what each trigger suggests without knowing
which role it belongs to. `codev-core::triggers` parses a group from a
`serde_norway::Value` field by field: unknown keys are collected rather
than refused, so an older codev reading a newer project's `triggers:`
warns instead of failing (the configuration's rule for unknown keys). The
built-in files stay strict: `RoleFrontmatter` gains a `triggers` field
read by the same parser, and a unit test fails on any problem, at build
time.

The built-in triggers favour precision over recall: `password`,
`api_key`, `access_token` rather than `token` (a lexer has tokens);
`street_address`, `email`, `phone_number`, `iban` rather than `address`
(a socket has one). What they miss, the project adds.

| Expert | `paths` | `manifests` | `patterns` |
|---|---|---|---|
| security | `auth`, `authn`, `authz`, `security`, `crypto` directories; `.env` files; `*.pem`, `*.key` | the dependency manifests of the spec | secrets and credentials; `eval`/`exec`; `innerHTML`; `pickle`; `shell=True`; `unsafe {` |
| privacy | `user(s)`, `customer(s)`, `profile(s)`, `account(s)` directories and files | — | e-mail, phone, birth date, street address, postcode, SSN, IBAN, passport, national id, IP address, geolocation |
| accessibility | `*.html`, `*.css`, `*.scss`, `*.tsx`, `*.jsx`, `*.vue`, `*.svelte` and server templates; `templates`, `components` directories | — | `aria-*`, `role=`, `tabindex`, `alt=`; form and media elements |
| performance | `*.sql`; `queries`, `repositories`, `benches` directories | — | SQL statements; `Promise.all`, `asyncio.gather`, `tokio::spawn`, thread pools; caching; pagination |
| ops | Dockerfiles, compose files, CI workflows, Kubernetes, Helm, Terraform, Ansible, deploy and infra directories, systemd units, logging configuration | — | environment variables; timeouts, retries, health checks; log levels; metrics |
| migration | `migrations`, `migrate` directories; `*.proto`, `*.graphql`, `*.prisma`, `*.avsc`, OpenAPI and Swagger files, `*.d.ts`, `schemas` directories | — | `ALTER`/`DROP`/`RENAME` of tables and columns; deprecations; `serde` renames and aliases |

**Alternatives considered**: triggers keyed by kind with the domain
implied by the role (`triggers: { paths: [...] }`) — shorter, but a
project trigger would need a role to hang from, and a role bound to
several domains could not say which one a trigger suggests; triggers
under `roles.<id>` in the configuration — each role entry is strict, so
an older codev would refuse the whole configuration.

### Decision: Globs through `globset`, expressions through `regex-automata`

Globs compile with `globset` (already a dependency), case-insensitive,
`*` not crossing `/`. Expressions compile with `regex-automata`'s
`meta::Regex`, which `globset` already pulls in with the features we
need: no new crate enters `Cargo.lock`. Its matching is linear in the
input whatever the expression, so a project pattern cannot hang codev
(no catastrophic backtracking); compilation is bounded by a size limit
(1 MiB of compiled program), and a pattern over it is `invalid_trigger`.

**Alternatives considered**: the `regex` crate (the same engine behind a
friendlier API, but four lines of convenience for a new direct
dependency and a new crate in the tree); substring lists instead of
expressions (no word boundaries: `password` would need ten spellings).

### Decision: The diff in `codev-engine::diff`, through `ProcessRunner`

All commands run in the project root, with
`git -c core.quotePath=false -c core.fsmonitor=false`: paths come out
unescaped, and a repository unpacked from an archive with its own
`.git/config` cannot make codev run its file-system monitor program. The
patch also passes `--no-textconv` and `--no-ext-diff`: no configured
program runs on codev's behalf.

1. `rev-parse --is-inside-work-tree` — otherwise *not a git repository*
   (a missing `git` answers 127: same outcome).
2. `rev-parse --verify --quiet HEAD` — otherwise *no commit yet*.
3. The base: `symbolic-ref --quiet --short refs/remotes/origin/HEAD`
   (`origin/main`), else `rev-parse --verify --quiet refs/heads/main`,
   else `master` — otherwise *no default branch (looked for origin/HEAD,
   main, master)*. Then `merge-base HEAD <base>`.
4. `diff --no-ext-diff --no-renames --ignore-submodules --relative
   --numstat -z <merge-base>` — every tracked change against the
   merge-base, committed, staged and unstaged alike, limited to the
   project directory (`--relative`), with added-line counts; binary files
   show `-`.
5. `ls-files --others --exclude-standard -z` — untracked files, relative
   to the project directory.
6. For the files the patterns must read: `diff --no-ext-diff
   --no-textconv --no-renames --relative --unified=0 --no-color
   <merge-base> -- :(literal)<path>…`
   in batches of at most 50 paths and 20 000 announced added lines (the
   Windows command line stays short, one batch's output stays bounded);
   added lines are the `+` lines, numbered from the `@@ +start` hunk
   headers. Untracked files are read through `FileSystem` after
   `is_regular_file` and `len`.

The engine returns a `Diff` value — files with their path, binary flag,
added count, and added lines when read — and a summary; the core decides
everything else. A git failure at any step becomes *unavailable*, with
the step's reason: the diff never fails a command.

`FileSystem` gains `is_regular_file` (`symlink_metadata`, no
dereference): an untracked symbolic link — possibly to `/dev/zero` or a
file outside the project — is never read.

**Alternatives considered**: three separate diffs (`base..HEAD`,
`--cached`, worktree) — the same lines, three times the parsing;
`git diff <base>` without `--numstat` first — the whole patch in memory
before any cap applies; reading tracked files from the working tree
instead of the patch — it cannot tell added lines from existing ones.

### Decision: The core classifies, caps and matches

`codev-core::triggers` holds the parsing of groups, the noise list
(`classify(path) -> Option<Skip>`), the caps, the matching and the
suggestions — pure functions over the `Diff` value and the proposal's
text. Classification goes by path only, deterministic and explainable;
content sniffing (`@generated` headers) is left out. Prose extensions
skip the patterns: a documentation page explaining passwords is not a
security change, and its code, if any, is matched on its own.

Each line is first matched against all the patterns at once — one
multi-pattern expression (`meta::Regex::new_many`) answering whether any
matches — and only a line that matches is tried pattern by pattern to
name its triggers: most lines cost one pass, not one per pattern.

Caps — 2000 files (in path order, so the cut is reproducible), 5000
added lines per file, 1 MiB per untracked file, 2 MiB of added text in
total, 2000 characters per line — are constants of the core, documented
in the Roles chapter. A capped scan sets `truncated`; it never fails.

**Alternatives considered**: a configurable noise list (a second place to
explain why a file was skipped; `.gitignore` already removes most build
output, and project triggers cover the rest); sniffing generated headers
(non-deterministic across generators, and a hand-written file can say
"DO NOT EDIT").

### Decision: A justification is text on the unticked line

`- [ ] security — only renames a variable`: the text after the domain
name, with leading dashes, colons and spaces removed, is the
justification. `codev-core::domains` already treats it as a free comment;
it now returns, per listed domain, whether it is ticked, its comment and
its line. `None` justifies nothing: it is one answer for six domains.

**Alternatives considered**: a separate `## Domain justifications`
section (two places to keep in sync); an HTML comment (invisible in a
rendered proposal, the place a reviewer reads).

### Decision: Suggestions are computed in the CLI's orchestration, not in `validate_change`

`codev-engine::suggest::for_change(fs, layout, config, ctx, roles,
&diff)` combines the triggers of the experts active-or-`on_domain` for
the change (not `never`) and the project's valid groups, and returns the
suggestions with their states. The diff is read once per command by
`codev-engine::diff::read(fs, runner, layout)`. `codev roles --change`
and `codev validate` (the CLI's `commands::validate`, once for all
active changes) call them; `engine::validate::validate_change` keeps its
signature, so the `sync` and `archive` pre-flights and `codev doctor`
never run git. The warning is located as the spec says.

**Alternatives considered**: a runner parameter on `validate_change`
(every pre-flight would run git, and an archive would depend on a
branch's diff); an error instead of a warning (a suggestion is evidence,
not proof; `--strict` turns it into a gate for teams that want one).

### Decision: Stacks from manifest names over a bounded walk; the filter in the core

`codev-engine::detect::stacks(fs, root)` walks the project breadth-first
with `list_dir`, three levels deep, skipping hidden directories and
`node_modules`, `vendor`, `third_party`, `target`, `dist`, `build`, at
most 2000 directories; `codev-core::detect::stack::languages(names)` maps
manifest names to languages. Presence is enough: a `Cargo.toml` that
does not parse still says Rust. `codev-core::roles::filter_by_stack(
knowledge, languages)` splits the `By stack` section into its `- **Label**:`
items (each item runs to the next one) and keeps the items as the spec
says. The CLI applies it to each served role unless `--all-stacks`.
`codev init`'s detection is unchanged.

**Alternatives considered**: `git ls-files` for manifests (no answer
outside git); `walk_files` over the whole tree (a `node_modules` walk
costs seconds); filtering in the skills, as today (a judgement codev can
make exactly, and the source of the noise this change removes).

### Decision: The contract grows additively

`RolesV1` gains `stacks`, `diff` and `suggestions`; `RoleV1` is
unchanged. A failure keeps the shape with `stacks: null`, `diff: null`,
`suggestions: []`. `domain_suggested` and `invalid_trigger` join the
codes of the uniqueness test. Skills generated by an older codev read
`codev roles --json` and ignore the new fields.

**Alternatives considered**: a separate `codev suggest` command (one more
call per step in each skill, and the roles a suggestion would activate
are served by `codev roles` anyway).

### Every case, planned

| Case | Behavior |
|---|---|
| No git repository | `diff.available: false`, reason *not a git repository*; no suggestion; `codev validate` reports none and does not fail (scenarios "Not a git repository", "No diff"). |
| A change with no diff yet | `diff.files: 0`, no suggestion; at `/codev-propose` time the diff usually holds only the planning files, which are `planning` noise: the product owner ticks by judgement (the skill says so when `files` is 0). Scenario "Vendored and planning files suggest nothing". |
| A docs-only change | Prose files skip the patterns; only `paths` triggers naming docs paths could match, and none of the built-in ones does (scenario "A docs-only change"). |
| Generated, vendored, binary or very large files | The noise list and the `large` and `binary` categories; binary and large files still meet `paths` and `manifests` (scenarios "A lock file suggests nothing", "Vendored and planning files suggest nothing", "A binary file is matched by its path only"). |
| Huge diffs | 2000 files in path order, 5000 added lines per file, 2 MiB of added text, 2000 characters per line; `truncated: true` beyond them (scenario "A huge diff is truncated"). Patches are fetched in bounded batches. |
| Monorepos with several stacks | Every stack found within three levels is served (scenario "A monorepo receives each of its stacks"); a project in a subdirectory of the repository sees only its own files (`--relative`, scenario "A project in a subdirectory of the repository"). |
| Windows paths and CRLF | git prints `/`-separated paths; `\r` is stripped from added lines (scenario "Windows line endings"); pathspecs are `:(literal)` so a `*` in a file name is not a glob; batches keep the command line under Windows' limit. |
| Offline | Local git commands only; `origin/HEAD` used as last fetched. |
| A role disabled by configuration | Its triggers are not evaluated (scenario "A disabled expert suggests nothing"); project triggers still are. |
| A step re-run (idempotence) | Read-only; the same tree gives the same document, sorted (scenario "Run twice, the same answer"). |
| An interrupted run resumed | Nothing is stored: a new session recomputes the suggestions; justifications live in the proposal. |
| Inherited sources | Their `triggers:` is ignored, like their `roles:` (scenario "Triggers of an inherited source are ignored"); built-in triggers come with the binary. Out of scope: sharing triggers through an inherited source. |
| Older binaries reading newer files | A codev without this change warns `unknown_config_key` on `triggers:` and ignores it; it reads `- [ ] security — reason` as a free comment, as today; skills it generated ignore the new JSON fields. A newer codev's unknown key inside a group is a warning here, not a failure. |
| Shallow clones (CI) | Without the merge-base in history, `merge-base` fails: no diff, no warning. Out of scope: deepening the clone; the docs say to use `fetch-depth: 0` for suggestions in CI. |
| Submodules | `--ignore-submodules`: a submodule's changes are its own repository's. |
| Hostile input | Project patterns run in linear time with a compile-size cap; untracked symbolic links and devices are never read; a matched line's content is never printed, only its path and number. |

## Risks / Trade-offs

- [Built-in patterns are noisy and teams learn to justify by reflex] →
  precision first in the built-in list, each match shown with its file
  and line so a justification is quick to check, and the skeptic checks
  each justification against its matches.
- [A suggestion is missed because the diff is unavailable (shallow CI
  clone, detached `HEAD` without a default branch)] → `diff.available` and
  `diff.reason` are in the document and the human output; the skills say
  when there is no diff; the docs give `fetch-depth: 0`.
- [Two active changes share a diff and get each other's suggestions] →
  a stated non-goal; each justifies what is not its own in one line.
- [`codev roles --change` gets slower on large repositories] → five git
  calls plus bounded batches, capped scan; measured on this repository in
  the verification task.
- [The stack filter drops an item a reviewer needed (a TypeScript
  front-end in a repository whose `package.json` is four levels deep)] →
  `--all-stacks`; three levels cover `apps/<name>/` and
  `packages/<scope>/<name>/`.

## Migration Plan

Nothing to migrate. Active changes whose diff suggests a domain they
neither tick nor justify get a `domain_suggested` warning, which fails
`codev validate --strict` until a line is ticked or justified. A team
mixing codev versions sees `unknown_config_key` on `triggers:` from the
older binary; raising `codev_version` makes everyone upgrade together.
Rollback: revert the commit; justifications stay as free comments.

## Open Questions

None.
