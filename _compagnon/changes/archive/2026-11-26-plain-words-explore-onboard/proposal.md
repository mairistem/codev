# Proposal: Explore and onboard suggest the next step in plain words

## Why

`plain-words-in-skills` made six skills suggest each next step as the
words to say, the command as a shortcut, and left `explore` and `onboard`
as they were until the evaluation suite covered them (decision 0008). The
change `explore-onboard-evaluation` recorded their baseline. These two
are the skills a newcomer meets first, and they still end on bare
commands: "ask me for `/compagnon-propose`", "type `/compagnon-propose
add-user-auth`", a table of `/compagnon-…` recommendations.

## What Changes

- `explore` and `onboard` include the shared `next-steps` rule and give
  every next step — explore's offer to plan the change, its guardrail
  refusal to implement, onboard's recommended action for each state of its
  table and its "if you want the details" pointers — first as the words to
  say, in the user's language, then the command as a shortcut; each
  command they name today stays, as the shortcut. Onboard's reply closes
  on its one recommended request; with several active changes, it lists
  them, each with its request and shortcut, and its last line asks which
  one, without a shortcut.
- `compagnon doctor`'s fix lines, which onboard relays verbatim, stay as
  doctor prints them, each after a few plain words saying what is
  missing; onboard's opening description says the user moves through the
  steps by asking Claude in their own words, the commands being
  shortcuts.
- Their descriptions name plain requests that start them, in English and
  in French (« fais-moi visiter Compagnon », « explorons une idée »…),
  within 1024 characters.
- Measured with `compagnon-evals` on `idea.explore` and `project.onboard`
  against the committed baseline; the nine cases' marks are the commands,
  which stay as shortcuts.
- The evaluation's case loading is simplified, as deferred by the review
  of `explore-onboard-evaluation` (R-newcomer-3): one helper parses the
  explore outcome and the onboard state.

## Capabilities

### New Capabilities

### Modified Capabilities

- skills

### Removed Capabilities

## Impact

- `assets/workflows/explore.md`, `assets/workflows/onboard.md`,
  `crates/compagnon-agents/src/workflows.rs` (descriptions, the list of
  skills the plain-words tests cover, the exemption of relayed doctor
  fixes).
- `evals/src/case.rs` (`load_read_only`), no behaviour change.
- The installed skills change: `compagnon update` refreshes them; a skill
  edited by hand stays preserved.
- Evaluation (decision 0008): `compagnon-evals run --step idea.explore
  --step project.onboard` and `compare` against the baseline, about 8 USD
  for five repetitions, with the user's agreement.
- Documentation: the "How to ask" sections already list plain requests
  for both skills; checked, not rewritten.

## Domains

- [ ] security — the matches are the manifests (`pyproject.toml`, `package.json`, `go.mod`) of the `explore-*` and `onboard-*` evaluation fixtures, from `explore-onboard-evaluation` on the branch; not touched here
- [ ] personal-data — the match is the word "phone" in the home page's example (`docs/site/index.html`), from another change on the branch
- [x] user-interface
- [ ] performance — the matches are the SQL of the `explore-*` and `onboard-*` evaluation fixtures and « cache » in French text of `evals/src/reliability.rs`, all from other changes on the branch; not touched here
- [ ] operations — the matches are words of the `explore-not-worth-gateway-retry` fixture, from another change on the branch
- [ ] migration — the match is `#[serde(deny_unknown_fields)]` on the evaluation case format (`evals/src/case.rs`), from another change on the branch; the simplification here keeps the format

## Release note

- Changed: The explore and onboard skills now suggest each next step in plain words — "say “plan this change”" — with the slash command as an optional shortcut.
