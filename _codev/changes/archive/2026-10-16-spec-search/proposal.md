# Proposal: Search the specs and read one requirement, so that planning finds what to modify for fewer tokens

## Why

When `/codev-propose` plans a change, it finds the requirements to modify by
`grep`-ing and reading whole `spec.md` files: in the 25 planning cases of the
evaluation suite, exploration makes about 60 % of the tokens written to the
cache, half of it on the specs (171 tool calls), and the plans still miss
about one primary requirement in five (primary recall 0.78) and add 9
probable duplicates of requirements they should have modified. A study of
this repository measured that a ranked search (BM25 with stemming) finds the
expected requirement twice as often as a ranked `grep` (recall@5 0.54
against 0.29; decisions 0.80 against 0.40), but that search alone did not
shorten the exploration, because the agent still read whole specs. Now that
planning is measured (decision 0008), the change can be shipped with the
proof it cuts tokens without planning worse.

## What Changes

- A new command, `codev search <query>`, ranks the requirements of the
  project's main specs (title, description and scenarios) and the
  decisions, inherited decisions included, against the query and prints the best matches, each with its
  capability or decision id and a short excerpt; `--json` prints the same
  under the stable JSON contract. Words are stemmed in the language the
  specs are written in.
- `codev search --archive` also lists matching archived changes, in a
  section of their own, never ranked together with the requirements.
- A new command, `codev show <capability>::<requirement>`, prints a single
  requirement — its heading, description and scenarios, as the spec holds
  them — and `codev show <capability>` lists the names of the capability's
  requirements, so that an agent reads what it needs instead of a whole
  `spec.md`; both take `--json`.
- Once the proposal is written, `codev instructions` gives, for the other
  artifacts, the existing requirements the change likely touches — ranked
  against the proposal's text, at most 10, each with an excerpt — and says
  that a requirement the change alters is modified rather than added
  again. The agent gets the information in the instructions it already
  reads, without being told to run any search; as the study and the first
  measure showed, an agent told to search keeps searching its own way.
- The `/codev-propose` skill reads the requirements its instructions name
  and modifies those the change alters, the list being a starting point,
  not a complete one. `/codev-explore` is left as it is: the evaluation suite does not cover it
  yet, and decision 0008 asks for a step to be covered before it changes.
- The CLI reference, the JSON reference and the skills chapter, in English
  and French, document both commands and the instructions' new list.

## Capabilities

### New Capabilities

- search

### Modified Capabilities

- instructions
- skills

### Removed Capabilities

## Impact

- `codev-core`: a pure ranking module (tokens, stemming, BM25) over
  documents already read, and the extraction of one requirement from a
  parsed spec; `codev-engine`: loading the specs, the decision index and
  the archive through the existing ports; `codev-cli`: the two commands
  and their JSON documents in `contract::v1`, and the `related` field of
  `codev instructions` (an addition to its JSON document).
- A new dependency for stemming, chosen in the design.
- `assets/workflows/propose.md`: the skill users install with
  `codev update`.
- `docs/{en,fr}/src/reference/cli.md`, `docs/{en,fr}/src/reference/json-output.md`,
  `docs/{en,fr}/src/skills.md`.
- Measured with the evaluation suite against the baseline of
  `evaluate-planning`, on `plan.write` and on the review steps: that
  baseline, with its planning runs (`evaluate-planning` task 6.2), must be
  recorded before this change is measured.

## Domains

- [x] security
- [ ] personal-data
- [ ] user-interface — suggested by the reliability page of `evaluate-planning`, which this branch builds on; this change only prints terminal text in the CLI's existing style
- [x] performance
- [ ] operations — suggested by the workflows of `evaluate-planning`, which this branch builds on; this change adds no workflow, log or timeout
- [ ] migration — the instructions document of the JSON contract gains a field, `related`, an addition that installed skills ignore; the new skill treats an absent list as empty, so a new skill with an older binary still works; no data or configuration changes

## Release note

- Added: `codev search <query>` finds the requirements and decisions that match a query, ranked by relevance, and `codev show <capability>::<requirement>` prints a single requirement.
- Changed: once the proposal is written, `codev instructions` names the existing requirements the change likely touches, and `/codev-propose` reads and modifies them rather than adding new ones beside them.
