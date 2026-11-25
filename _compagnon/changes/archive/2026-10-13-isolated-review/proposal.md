# Proposal: Each reviewer works alone, then the team meets

## Why

At `plan.challenge`, `plan.trace` and `code.review`, every reviewing role
runs in the conversation that wrote the plan or the code: the reviewer
has read the author's reasoning and the other roles' opinions, and tends
to agree with them. A real team reviews separately first and meets
afterwards. Claude Code can now run sub-agents that see only what they
are given, so codev can offer that way of working, as an opt-in until
it is measured.

## What Changes

- A new `review:` key in `_codev/config.yaml`, `review: { mode: inline |
  isolated }`, and the same key in a change's `change.yaml` to override
  it for that change. The default is `inline`, today's behavior. An
  unknown value fails with the new stable code `invalid_review_mode`,
  naming the allowed values. An unknown key under `review:` warns with
  `unknown_config_key`, like `mcp`.
- `codev roles --json` tells the skills the effective mode and where it
  comes from (`default`, `project` or `change`). With `--change` at a
  reviewing step, it also measures the material the reviewers will read:
  each file with its size, a size limit, and which files must be
  summarized because the whole would exceed the limit. The binary
  measures; it never calls a model.
- In isolated mode, `/codev-propose` and `/codev-apply` run each active
  reviewing role — the skeptic, the tracer at its step, the newcomer and
  the active experts — as a separate sub-agent, in parallel when the
  platform allows. A sub-agent receives only its role, the project's
  context and rules, and the material, never the conversation, and
  returns its findings in the review record's format. A sub-agent that
  fails, times out or answers in another format is retried once; then
  that role runs inline and the record says so.
- "The meeting": the skill consolidates the findings — the same defect at
  the same place becomes one finding crediting every role that raised
  it — ranks them, writes the record, then runs the author's answer loop
  and the escalation as today.
- The review record gains, all optional and ignored by an older codev:
  a finding's `raised_by` list, a step's `Mode:` and `Summarized:`
  lines, and a role's `Fallback:` line. `codev review --json` exposes
  them.
- The `propose` and `apply` skills request the sub-agent tool only when
  the project's mode is `isolated`. `codev update` regenerates them when
  the mode changes, and `codev doctor` reports skills rendered for
  another mode with the new code `skills_review_mode`.
- codev's own configuration sets `mode: isolated`, to try the mode on
  its own changes.

## Capabilities

### New Capabilities

- `review-mode` — the `review.mode` setting, its per-change override,
  and what `codev roles` tells the skills about the mode and the
  material.

### Modified Capabilities

- `skills` — the isolated review, its failure handling and the meeting;
  the sub-agent tool in `allowed-tools`; `codev update` follows the
  mode.
- `review-record` — `raised_by`, the `Mode:`, `Summarized:` and
  `Fallback:` lines, and their place in `codev review`.
- `doctor` — the `skills` check compares the skills with the mode.
- `configuration` — `review` is a known key, its sub-keys are checked.
- `docs` — isolated review and the meeting are documented in both
  languages.

### Removed Capabilities

None.

## Impact

- `codev-core`: the review mode and its resolution, the material budget,
  the record's new field and lines.
- `codev-engine`: `config` and `metadata` read `review:`; a new error
  `invalid_review_mode`; the material is measured from the diff or the
  change's artifacts.
- `codev-agents`: the sub-agent tool placeholder, the isolated-review
  procedure shared by `propose` and `apply`, skill states per mode.
- `codev-cli`: `RolesV1.review`, `ReviewV1` additions (additive in v1),
  `codev doctor`'s new code, the human rendering.
- `assets/workflows/` and the regenerated `.claude/skills/`.
- `_codev/config.yaml` of this repository.
- Documentation, `docs/en` and `docs/fr`: `roles.md`, `skills.md`,
  `faq.md`, `reference/configuration.md`, `reference/file-formats.md`,
  `reference/json-output.md`, `guides/review-record.md`.

## Domains

- [x] security — a skill pre-approves a tool that starts sub-agents with tools of their own
- [ ] personal-data — no personal data is read, stored or sent; the material is the project's own files
- [ ] user-interface — codev has no user interface; the human output of `codev review` and `codev doctor` gains lines
- [ ] performance — the binary only adds a size lookup per changed file; the cost of sub-agents is a model-token cost, bounded by the material limit and the opt-in
- [x] operations — retries, timeouts and the fallback to inline when a sub-agent fails
- [x] migration — new configuration keys, a new field in `change.yaml` and `review.md` that an older codev reads differently, and additions to the JSON contract

## Release note

- Added: Isolated review. Set `review: { mode: isolated }` in
  `_codev/config.yaml`, or in a change's `change.yaml`, and
  `/codev-propose` and `/codev-apply` run each reviewing role as a
  separate sub-agent that sees only its role, the project's rules and
  the material — never the conversation — then merge their findings,
  crediting every role that raised one. The default stays `inline`. Run
  `codev update` after changing the mode.
- Added: `codev roles --json` gives the review mode and measures the
  material the reviewers read, naming the files to summarize beyond a
  size limit; `codev review` shows how each step was reviewed, what was
  summarized and which roles fell back to inline.
- Added: `codev doctor` reports skills generated for another review mode
  (`skills_review_mode`).
- Changed: A project or a change that sets `review:` needs this codev
  release or a later one everywhere it is read, CI included: an older
  codev warns about the unknown key (and fails with `--strict`), and
  refuses a `change.yaml` that sets it. Raise `codev_version` in
  `_codev/config.yaml` to this release when you set the mode.
