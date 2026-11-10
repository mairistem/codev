# Design: Validate detects what merging parallel changes breaks

## Context

codev reads one tree. When two branches each carry a change, the files of
`_codev/` meet through git's textual merge:

- the parser detects a requirement name defined twice only inside a delta
  section (`parse_requirement_blocks` in `codev-core/src/parser/delta.rs`);
  `parse_spec` lets a main spec hold `Login` twice;
- the decision index reports a shared id only between the project and an
  inherited source (`detect_id_collisions` in
  `codev-engine/src/decisions.rs`); two local `0013` files pass, and the
  seal check keys its paths by id, so one of them is silently dropped;
  `seal::verify` uses the first entry of a duplicated id;
- the check of a delta against its main spec
  (`check_delta_against_spec` in `codev-core/src/validate/mod.rs`) covers
  `RENAMED` only. A `MODIFIED` whose target is gone is refused at sync
  with `modified_target_missing`, after `validate` said nothing; so is a
  `REMOVED` whose target is gone — even after the change's own sync, since
  the merge has no idempotent skip for it — and an
  `ADDED` whose name already exists is skipped as a no-op by the merge
  (`codev-core/src/merge/mod.rs`), whose comment says the validator should
  report the conflicting case — it does not;
- a conflict marker left in a markdown file is not a defect anyone
  reports; in a YAML file it surfaces as a parse error.

`codev validate --strict` already runs in CI (`codev init` writes it into
GitHub workflows), and on a GitHub `pull_request` event the checkout is
the merge of the branch into its base: whatever `validate` detects on a
tree, CI detects on the merge result, before the merge is made.

The proposal lists the defects; this design settles how, and how the rest
of the subject — collisions seen before the branches meet — is split into
later changes.

## Goals / Non-Goals

**Goals**

- Every collision that a text merge of two branches leaves in `_codev/`,
  and that the merged tree alone can reveal, is an error of
  `codev validate`, at its file and line, with what to do.
- No git, no network, no new port: the check works on any tree, in any
  CI, on Windows, macOS and Linux.

**Non-Goals**

- Seeing a collision before the branches meet — another branch's change
  name, decision id or delta — which needs git: change 2 below.
- A command that renumbers a decision: change 3 below. This change names
  the next free id and documents the manual steps.
- Creating worktrees or branches, orchestrating processes (decision 0007),
  or a lock file: out of the whole series.
- Two archived changes with the same name under `_codev/changes/archive/`:
  both dated folders survive the merge and nothing reads them as one;
  change 2 prevents the name clash earlier.

## Decisions

### Decision: Detect on the merged tree in `validate`, before anything that needs git

The first change is the one that needs nothing but the tree: it protects
every team as soon as it upgrades, through the `codev validate --strict`
its CI already runs, whatever the forge, and it is the base the later
changes point to ("validate will refuse this after the merge"). Every
check is decided in `codev-core` on text the engine already reads
(decision 0001); the engine adds no port and no process (decision 0002).

**Alternatives considered**:

- Starting with a cross-branch check in `doctor`: it warns earlier, but
  only on a machine that has the other branches locally, never in CI, and
  it still needs a verdict on the merged tree when a warning was ignored.
- A dedicated `codev check-merge` command: a second entry point CI would
  have to learn, for checks that are plain validation defects.

### Decision: One duplicate rule for delta sections and main specs

`parse_spec` reports `duplicate_requirement` with the same message and
both lines as the delta sections do, by reusing the duplicate detection of
`parse_requirement_blocks` rather than adding a second implementation.
Both requirements stay in the parsed result, so the other findings still
point at the right lines.

Today the change's validation reads its targeted main specs and drops the
parser's findings. It now reports, as errors of the change, the
`duplicate_requirement` and `merge_conflict_marker` findings of the main
specs its deltas target: with `Login` twice, the merge's name index keeps
one copy and a `MODIFIED` would rewrite it while the other diverges. The
pre-flight of sync and archive therefore refuses. A broken main spec the
change does not target is reported by `codev validate` as a whole, not by
that change's pre-flight.

**Alternatives considered**:

- A validator rule (`codev-core/src/validate/rules.rs`): rules see one
  requirement or one delta, while duplicate detection already belongs to
  the parser, which knows both lines.

### Decision: `ADDED` collisions compared on parsed content, language-blind

An `ADDED` whose name the main spec already holds is either an earlier
sync of this very change (re-syncing must stay possible) or another
change's addition. The two are told apart by content: same description and
same scenarios — names and bodies — compared on the parsed requirement,
with line endings normalized and trailing whitespace of each line and
trailing blank lines ignored. The parsed form drops the keyword lines, so a
French delta synced into an English spec, whose headings the merge
translated, compares equal.

`MODIFIED` needs no comparison: its target must exist, unless a `RENAMED`
of the same delta produces it. `REMOVED` of an absent requirement is not
reported: the outcome it asks for is already there, which is also the
state after its own sync.

The merge keeps its idempotent skip for an equal `ADDED`, and the
pre-flight now refuses the different one before the merge sees it. The
merge gains the same skip for a `REMOVED` whose target is gone, which it
refused with a misleading `modified_target_missing`: `validate` and the
merge then agree, and a change synced before being archived, or whose
removal another branch already merged, archives.

**Alternatives considered**:

- Comparing raw text: a French delta synced into an English spec, or a
  CRLF checkout on Windows, would raise a false error on every re-sync.
- Recording in `change.yaml` which requirements a sync already applied:
  new state to keep consistent across merges, which is the very problem.
- Turning the merge's skip into an error: the merge would refuse, but
  only at archive time; `validate` reports it in CI already.

### Decision: Conflict markers are a line-prefix scan outside code fences

A line is a marker when it starts with seven `<`, `|` or `>` followed by a
space or the end of the line — git's opener, diff3 base and closer. The
separator `=======` alone is not checked: it is a markdown heading
underline, and git never writes it without an opener and a closer. In
markdown, lines inside fenced code blocks are skipped with the fence
tracking the parser already has (`parser/fence.rs`).

The scan runs on the files `validate` reads: main specs, every file of an
active change — artifacts, `review.md`, `corrections.md`, `change.yaml` —
and local decisions. `_codev/config.yaml`, `seal.yaml` and each active
change's `change.yaml` are read before any check runs; when their YAML fails to parse and they hold a marker, the
reading reports a `merge_conflict_marker` error naming the file and line
instead of the YAML error. That error belongs to config and seal reading,
so every command that reads the file reports it, as the spec states.

**Alternatives considered**:

- `git diff --check` or `git ls-files -u`: needs git and an index in
  conflict, while a marker committed by mistake is the case to catch.
- Including `=======`: false positives on setext headings, no extra
  detection.
- Scanning inside fences too: a conflict inside a fenced example would be
  caught, but a decision or a guide showing git's markers as an example
  would fail validation; the limit is stated in the spec.

### Decision: Local duplicate ids are errors that name the next free id

`decision_id_duplicate` is an error, unlike the inherited
`decision_id_collision` warning: between a project and a source the
project wins by design (decision 0005), while two local decisions with one
id make `supersedes`, `decision show` and the seal ambiguous. The message
names every file and the next free id (largest numeric local id plus one,
the rule `decision new` uses). The seal check stops keying paths by id
alone, so both files are checked. `decision_seal_duplicate` reports a
duplicated `seal.yaml` entry, the trace of two branches each sealing their
`0013`. While an id is sealed several times, a decision matching any of
its entries is not a `decision_seal_mismatch`: the second branch's
decision would otherwise be reported as edited. `codev decision seal
<id> --force` leaves a single entry for the id, which gives the
regeneration the guide recommends a working command; today it rewrites
every entry and keeps the duplicate.

Which of the two keeps its id is not decided by codev: the tree does not
say which branch merged first. The guide recommends keeping the id of the
decision already on the default branch.

**Alternatives considered**:

- Ids that cannot collide (a date, a branch hash, a random suffix):
  evaluated for change 3. Ids are cited in prose, designs and other
  decisions, and the four-digit sequence is part of the accepted format
  (`decisions` spec); a collision is rare and now loud.
- A warning: CI with `--strict` would catch it, without `--strict` the
  ambiguous index would pass.

### Decision: One documented rule — archive inside the pull request

The guide states one rule: a change is archived inside its pull request,
as its last commit, after the branch was updated from the default branch;
CI validates the merge result. Archiving after the merge on the default
branch was rejected: it leaves the default branch with code whose specs
are not merged yet, needs a second commit — often a second pull request on
a protected branch — and the archive's spec diff is never reviewed.
Archiving in the pull request means the reviewer sees the spec diff, and
the textual merge of two archives is exactly what `validate` now checks on
the merge result. That result is only current if the forge reruns CI when
the base moves: the guide asks for "require branches to be up to date
before merging" or a merge queue on GitHub, merged results pipelines on
GitLab, and says that without them a collision between two pull requests
merged back to back is caught on the default branch, after the merge.

Generated files that conflict are regenerated, not hand-merged: the seal
with `codev decision seal`, the `CHANGELOG.md` managed section with
`codev changelog --write` (whose drift `doctor` already reports), the
skills with `codev update`. A clone whose skills are older than its binary
is already reported by `doctor` (`skills_outdated`) and its version
requirement by `codev_version`; the guide says so instead of adding a
mechanism.

**Alternatives considered**:

- Changing `/codev-archive` to check the rule: a change to a skill's text,
  which decision 0008 requires to be evaluated; it belongs to change 4.

### The series: what follows this change

1. **This change** — `validate-detects-merge-collisions`: the merged tree
   is checked; the rule is documented.
2. **`doctor-sees-other-branches`** — a read-only `branches` check in
   `codev doctor`: from the local refs only (local branches and the
   remote-tracking refs already fetched; never a fetch), it lists the
   branches not merged into `HEAD` and warns when one of them carries an
   active change of the same name, a decision id this branch also adds,
   or a delta touching a requirement this branch's deltas touch
   (`MODIFIED` against `MODIFIED`, `REMOVED` or `RENAMED`). Git runs
   behind `ProcessRunner` with the local, read-only commands `diff.rs`
   already uses (`for-each-ref`, `merge-base`, `ls-tree`, `show`), bounded
   in number of refs; the core decides the collisions from the listings
   (decisions 0001, 0002). `codev new change` may warn on a name found on
   another branch. In `doctor` rather than `validate`: the verdict of
   `validate` must not depend on which branches a machine happens to hold.
3. **`decision-renumber`** — `codev decision renumber <id> <file>` moves
   one of two duplicates to the next free id in one plan: the file name,
   its `id`, the references in other decisions (`supersedes`,
   `deviates_from`) and in active changes' designs, and the seal entry.
   Its design re-evaluates non-colliding ids against this renumbering.
4. **`archive-follows-the-merge-rule`** — `/codev-archive` states the rule
   and warns when the branch is behind the default branch as known
   locally; a skill change, evaluated as decision 0008 requires.

## Risks / Trade-offs

- [A project whose main specs already hold a duplicate, or whose decisions
  already share an id, fails `codev validate` after the upgrade] → the
  release note marks it **BREAKING**; the error names both lines or files
  and the guide gives the resolution; this repository's own specs and
  decisions are checked by a task before release.
- [`added_target_exists` on a delta edited after its sync, which the merge
  used to drop silently] → it is the right verdict: the edit was being
  lost; the message tells to turn it into a `MODIFIED`.
- [A conflict inside a fenced code block is not reported] → stated in the
  spec and the guide; a broken example inside a fence does not change the
  parsed requirements, so the merge stays unambiguous.
- [The failure code of every command reading a config, seal or
  `change.yaml` file holding a marker changes from `invalid`, `unreadable`
  or `seal_invalid` to `merge_conflict_marker`] → in the **BREAKING**
  release note item and in the JSON output reference's error table.
- [A marker-like line in a YAML file or outside a fence that is not a
  conflict] → seven identical characters then a space at the start of a
  line has no other use in codev's formats; the fence exemption covers
  documented examples.
- [The new errors change the validity measured by the planning evaluation,
  which records `codev validate` on each plan] → a task runs
  `codev-evals` on `plan.write` and compares with the baseline, as
  decision 0008 requires for an output the agents read.
- [A pipeline that validates the branch alone (GitLab without merged
  results pipelines) does not see the collisions] → the guide says so and
  names the setting.

## Migration Plan

No data migration. On upgrade, a project runs `codev validate`; each new
error names its fix. Rollback is a downgrade of the binary: no file format
changes.

## Open Questions

- Whether `doctor`'s cross-branch check (change 2) should also inspect
  remote-tracking refs of every remote or only `origin`: it does not change
  this change.
