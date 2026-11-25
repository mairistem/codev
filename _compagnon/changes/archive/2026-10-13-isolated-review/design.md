# Design: Each reviewer works alone, then the team meets

## Context

The reviewing roles are served by `compagnon roles --at <step> --change
<name> --json` (`compagnon-engine::roles::resolve`, the change's `change.yaml`
and domains applied; knowledge filtered by stack in `compagnon-cli`). The
skills apply them in the conversation: `propose.md` step 6
(`plan.trace`, the tracer fixes gaps directly) and step 7
(`plan.challenge`), `apply.md` step 5 (`code.review`); the record
procedure is shared once, `assets/workflows/_review-record.md`,
substituted for `{{REVIEW_RECORD}}` by `compagnon-agents::claude::render`.

A skill's `allowed-tools` is a static string of the catalog
(`compagnon-agents::workflows::CATALOG`), with one placeholder,
`{{JIRA_MCP_TOOL}}`, substituted from `_compagnon/config.yaml.mcp.jira_tool`
through `RenderCtx` (`substitute_jira_mcp`: the placeholder and its
leading comma vanish when unset). `plan_skills` preserves a skill whose
version stamp is current but whose content differs from the rendering:
it treats it as edited by hand. `compagnon doctor`'s `skills` check only
compares version stamps (`SkillState`).

`_compagnon/config.yaml` is read by `compagnon-engine::config::load` into
`ProjectConfig`: unknown top-level and `mcp` keys warn
(`unknown_config_key`), invalid values of known keys fail
(`invalid_compagnon_version` has its own stable code). `change.yaml`
(`compagnon-engine::metadata`) is strict, `deny_unknown_fields`. The change's
diff is read once by `compagnon-engine::diff::read` for the triggers, and
`compagnon-core::triggers::classify` tells noise (planning, vendored, lock,
generated) from code.

The review record (`compagnon-core::review`) reads structure from headings
and ignores other lines; a finding's YAML block is read field by field,
an unknown field being the warning `review_unknown_field` (the forward
compatibility rule of the review-record design: a new optional field is
additive, a new status or required field raises the format marker).

Decisions in force: 0001 (the core decides, the shell reads and writes;
nothing in `compagnon-core` does I/O), 0002 (crate graph; the JSON contract
in `compagnon-cli::contract::v1`), 0004 (one skill per workflow, no command
file), 0006 (`serde_norway`).

## Goals / Non-Goals

**Goals:**

- A reviewer that has not read the conversation, by configuration, with
  today's behavior as the default.
- Every deterministic part in the binary — the mode, its origin, the
  material and its budget, the record's new lines — and only judgement
  in the skills.
- Records written in isolated mode stay readable by the compagnon that
  introduced the record.

**Non-Goals:**

- The binary starting a model, a sub-agent or a process other than git:
  it never calls an LLM.
- Running the author's answer loop or the reviewer's re-check in
  sub-agents: they stay inline, as today.
- A custom sub-agent definition (`.claude/agents/`) with a restricted
  tool list; the sub-agent is told to write nothing (see Risks).
- A configurable material limit, or a per-role budget.
- A measured comparison of the two modes (see the default below).

## Decisions

### Decision: The default is `inline`, because no fair comparison can be run today

The evaluation harness (`evals/`) runs each case as one headless
`claude -p` session, started from committed files, with every role but
the evaluated one set to `never` (`runner::change_yaml`). Isolation
targets two effects: the reviewer reading the author's reasoning, and
roles reading each other's opinions in one pass. A case has neither: no
authoring conversation exists in the session, and one role is active.
In that setting the `inline` condition is already a fresh context with a
single role, and the `isolated` condition only wraps the same role and
the same material in a sub-agent. A paid run would compare two
equivalent conditions, one run per case, within the noise the baseline
report itself warns about ("a score is a noisy point"). No run was made:
cost 0 USD.

A fair comparison needs a new kind of case — a scripted authoring
conversation before the review, several roles active, and a scoring that
counts merged findings — which is a change of the harness and of the
cases, out of this change's scope. Until then, `inline` stays the
default: it is today's behavior, it costs one reading of the material
instead of one per role, and switching is one line
(`review: { mode: isolated }`, then `compagnon update`), documented in the
configuration reference.

**Alternatives considered**: running the 40 cases in both modes anyway
(about 30 USD for a comparison that cannot show the effect); defaulting
to `isolated` on the argument alone (multiplies the review cost by the
number of roles for every user, unmeasured).

### Decision: compagnon's own repository sets `mode: isolated`

compagnon is developed with compagnon, and its own `_compagnon/config.yaml` exists
to try formats and workflows on real changes. Its changes routinely
activate several reviewers (the skeptic or the newcomer, `migration` for
the JSON contract, often `security`), which is where isolation and the
merge matter. The maintainers accept the higher cost, and the committed
skills then show the `Agent` tool injection in the repository. The
evaluation harness is unaffected: it initializes each case with its own
default configuration.

**Alternatives considered**: keeping compagnon's repository inline until a
measurement exists (the mode would never meet a real change, and the
procedure would rot untested).

### Decision: The mode lives under `review:`, project then change

```yaml
# _compagnon/config.yaml, or a change's change.yaml
review:
  mode: isolated   # inline (default) | isolated
```

`compagnon-core::review::mode` holds `ReviewMode` (`Inline`, `Isolated`,
`parse`, `as_str`, `ALL`) and `resolve(project, change) -> (mode,
origin)`, pure. `compagnon-engine::config` reads `review` as a YAML value,
so that `mode` is validated with our message, and adds `review` to
`PROJECT_KEYS` and a `REVIEW_KEYS = ["mode"]` list for the
`unknown_config_key` warning, as `mcp` does: a newer compagnon may add a
budget key, and an older one should say so rather than refuse. An
invalid value fails with `EngineError::InvalidReviewMode { path, value }`,
code `invalid_review_mode`, like `invalid_compagnon_version`: a consumer
tests the code, not a word of the message. `change.yaml` gets an
optional `review: { mode }`, strict like the rest of the file, validated
by the same function. `ResolvedConfig.review_mode` is project only, like
`mcp` and `roles`: inherited sources give context and rules, not the
team's way of working.

**Alternatives considered**: a top-level `review_mode: isolated` (one
key less, but no room for later review settings); `deny_unknown_fields`
under `review` (an older compagnon would refuse a project using a newer
key); inheriting the mode (a shared source would impose a token cost on
every project).

### Decision: `compagnon roles` serves the mode and measures the material

`RolesV1` gains `review: { mode, origin, material }`, additive in v1,
`null` in a failure's document. `material` is given with `--change` at
`plan.trace`, `plan.challenge` and `code.review`:

```json
"review": {
  "mode": "isolated", "origin": "project",
  "material": { "limitBytes": 163840, "totalBytes": 172000, "summarized": true,
    "files": [ { "path": "src/a.rs", "bytes": 12000, "treatment": "full" },
               { "path": "src/big.rs", "bytes": 160000, "treatment": "summarize" } ] }
}
```

The budget is pure, `compagnon-core::review::material::budget(files, limit)`:
smallest first, ties in path order, `full` while the running sum stays
within the limit. Smallest first keeps the most files whole, and a large
file is the one whose change is most likely summarizable. The engine
lists the files: the change directory's files but `change.yaml` and
`review.md` for the plan steps; the diff already read for suggestions,
minus `triggers::classify` noise, binary files and anything
`FileSystem::is_regular_file` rejects — a committed symbolic link could
point outside the project — for `code.review`, with each file's current
size from `FileSystem::len` (0 when deleted). No content is read.

The budget counts whole files because a `full` file is read whole: its
diff adds only the removed lines. A `summarize` file is not read whole;
its summary is written by a separate sub-agent that sees only that
file's diff, quotes the changed lines verbatim when they fit, and
describes the rest — so a one-line fix in a large file still reaches the
reviewers as the line itself, and the summary never comes from the
conversation that wrote the change. The limit, 160 KiB, is about 40,000 tokens: with a
role's criteria and knowledge, a sub-agent stays far from a context
window, and five roles stay around the cost of one long inline session.

The skills already call `compagnon roles` at each step: no new command, no
new tool (decision 0004).

**Alternatives considered**: a new `compagnon review material` command (one
more call and contract for data `compagnon roles` is the natural carrier
of); letting the skill measure (non-deterministic, and the record could
not say exactly what was summarized); counting tokens (needs a
tokenizer, model-specific).

### Decision: The skills dispatch sub-agents; compagnon only renders the permission

> Promoted to ADR **0007** — see `_compagnon/decisions/0007-the-skills-dispatch-sub-agents-compagnon-only-renders-the-permission.md`.

### Decision: The meeting merges by place and fix, credits with `raised_by`

Two findings are the same when they cite the same file or plan section
with overlapping lines and call for the same fix — the review record's
"one finding per defect" rule, applied across roles. The merged finding
is written once, under the role that rated it most severe (then higher
confidence, then `compagnon roles` order), keeping that role's id, evidence
and title, with the other roles' points folded into its
recommendation, and `raised_by: [<owner>, <others>]`. The owner
re-checks the answers: `awaiting` keeps reading `by` against the
section's role, unchanged.

`raised_by` is an optional field of the block: the compagnon that introduced
the record warns `review_unknown_field` and keeps working, which is the
record's rule for additive fields (no format bump). A role whose
findings were all merged writes `No finding.` — meaning none of its own
— and `compagnon review` lists it as `credited` by the shared finding, so
the reader sees the credit without a new role state, which an older
compagnon would call invalid.

**Alternatives considered**: a new role line such as `Merged into
R-…` (an older compagnon reports `review_invalid`, which would need a format
bump); keeping duplicates (the human reads the same defect several
times, the very cost the merge removes); crediting through a free-text
line (not checkable).

### Decision: How a step ran is recorded as plain lines compagnon reads

`Mode: isolated.` and `Summarized: <text>` under a step heading, and
`Fallback: <reason>` under a role heading, are body lines: the parser
that predates them ignores them as free text, so the record stays
compatible both ways. The new parser reads them into
`StepSection.mode`, `StepSection.summarized` and `RoleSection.fallback`;
an unknown mode or a duplicate line is `review_invalid`. `ReviewV1`
gains `steps[].mode`, `steps[].summarized`, `roles[].fallback`,
`roles[].credited` and `findings[].raisedBy`, additive in v1.

**Alternatives considered**: fields in a YAML block per step (a new
block kind, invalid for an older parser); nothing recorded (the user
could not tell a reviewer that never ran from one that found nothing).

### Decision: `compagnon update` and `compagnon doctor` know the mode a skill was rendered for

`plan_skills` treats a skill as edited by hand only if it differs from
the rendering in both modes: a skill rendered by this compagnon for the
other mode is regenerated without `--force`. `ClaudeCode::skill_states`
takes the `RenderCtx` and returns `SkillState::OtherReviewMode` for a
current-version `propose` or `apply` skill whose frontmatter's
`allowed-tools` does (or does not) list `Agent` against the mode;
`compagnon-core::doctor` reports it as `skills_review_mode`, after
`skills_missing` and `skills_outdated`, with the fix `compagnon update`
(`compagnon update --force` for a skill it preserves as edited by hand):
doctor cannot tell a hand edit from a stale rendering, and a fix that
`compagnon update` alone cannot apply would leave `--strict` failing. The
doctor table requirement is modified to list the new code, and its
`config.keys` row names `review` next to `mcp`.

**Alternatives considered**: a hash of the render context in the skill's
metadata (changes every skill's format for one bit of information);
`--force` documented as the way to switch (it also erases real hand
edits).

### Every case, planned

| Case | Behavior |
|---|---|
| Monorepo / several stacks | Knowledge is filtered by the detected stacks as today; the material lists paths relative to the compagnon project root, as the diff does. |
| Huge diff | Over 160 KiB, files are summarized smallest-last; `Summarized:` lines name them. The diff reader's caps (2,000 files) still apply. |
| No git | `material` is `null` at `code.review`; the skill gives the files the tasks name. Plan steps still have material. |
| Docs-only change | Markdown files are material like code (`prose` is not noise for the reviewers); no special case. |
| Generated, vendored, lock, binary files | Left out of the material, as from the triggers. |
| Windows paths | Paths `/`-separated, sizes from the file system; no path separator assumed. |
| Offline | No network: sub-agents run in the user's Claude Code; compagnon reads sizes only. |
| Sub-agent tool unavailable | Every role inline, `Fallback:` per role, summary says so. |
| Sub-agent failure | One retry, then inline with `Fallback:`. A failure is what the platform reports (an error, a stop on its own limits) or a malformed answer. |
| Tracer at `plan.trace` | Isolated like the others; its gaps are fixed in the artifacts, not recorded — `plan.trace` has no section in the record (the review-record design's non-goal); a fallback is named in the summary. |
| Symbolic link in the diff | Not material: never measured, never handed to a reviewer. |
| Re-run / resumed session | The record is the state, as today: the procedure resumes from `compagnon review`, and sub-agents are told the ids already recorded so as not to raise them again. |
| Older compagnon reading a newer record | Lines ignored; `raised_by` warns `review_unknown_field`. |
| Older compagnon reading a newer config | `review` is an `unknown_config_key` warning; skills render inline. A `change.yaml` with `review:` fails on an older compagnon (strict file): pin `compagnon_version`. |
| Inherited sources | The mode is never inherited. |

## Risks / Trade-offs

- [A sub-agent acts beyond reading — Claude Code's general-purpose
  sub-agent has write, shell and network tools, and the material may
  hold text written to steer it (prompt injection in a changed file)] →
  the brief says read-only and that the material is data, never
  instructions; the skill picks a sub-agent type without file-editing
  tools when one exists; the sub-agent's tool calls go through the
  user's Claude Code permissions like the main agent's (pre-approving
  `Agent` approves starting it, not what it runs); writes in the
  repository show in the diff the human reviews. A restricted agent
  definition is the follow-up once the mode is measured.
- [The main agent leaks the conversation into the brief] → the
  procedure builds the brief from named JSON fields and files only, and
  forbids anything else; it cannot be enforced by compagnon, it is checked in
  review of the skill text (tests assert the instruction is there).
- [Isolated review costs more tokens] → opt-in, inline default; a
  material limit; parallel sub-agents cut wall time, not tokens.
- [A `change.yaml` with `review:` breaks an older compagnon; `review:` in the
  configuration and `raised_by` in a record make an older compagnon warn, and
  `--strict` fail] → **migration**: only when the mode is used; the
  release note and the configuration reference say to raise
  `compagnon_version` to this release, so every compagnon reading the
  repository, CI included, understands the keys.
- [Merging hides a role's distinct concern] → only same place and same
  fix merge; different fixes stay separate; every raiser is credited.
- [The `Agent` tool is renamed again] → one constant in `compagnon-agents`;
  the fallback keeps the review running inline.

## Migration Plan

Nothing to migrate: without `review:`, behavior and skills' tools are
unchanged; the skill bodies grow by the procedure, picked up by `compagnon
update`. Projects switching set `review: { mode: isolated }` and run
`compagnon update`; `compagnon doctor` reminds them. Rollback: remove the key and
run `compagnon update`; records keep their lines, ignored or read.

## Open Questions

None.
