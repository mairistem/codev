# Proposal: Skills suggest the next step in plain words

## Why

When a skill finishes, refuses or recommends what to do next, it names a
slash command (`/compagnon-apply`, `/compagnon-archive`…): a beginner or a
reader who does not code has to learn the commands to go on, while Claude
Code would start the right skill from a plain request. The documentation
now leads with plain-words requests (change `accessible-docs`); the skills
still contradict it.

## What Changes

- The six skills the evaluation suite covers — propose, apply, update,
  sync, archive, configure — suggest every next step first as the words to
  say, in the user's language ("say “implement it”"), then give the slash
  command, or the `compagnon` command, as an optional shortcut. Their
  refusals and summaries follow the same rule. No pointer is removed: each
  command they named stays, as the shortcut.
- A shared fragment states that rule once, and each of the six skills
  includes it.
- The descriptions of the same six skills, which Claude Code matches
  against a request to pick a skill, name the kinds of plain requests that
  should start them, in English and in French.
- `compagnon init`'s closing hint invites the user to ask Claude Code in
  plain words ("ask Claude Code to configure Compagnon for this project"),
  the slash command given as the shortcut.
- `explore` and `onboard`, body and description, are left as they are: the
  evaluation suite does not cover them yet, so per decision 0008 a
  following change covers them first, then converts them.

## Capabilities

### New Capabilities

### Modified Capabilities

- skills
- init

### Removed Capabilities

## Impact

- `assets/workflows/{propose,apply,update,sync,archive,configure}.md` and a
  new fragment under `assets/workflows/fragments/`;
  `crates/compagnon-agents/src/workflows.rs` (fragment registration,
  descriptions, tests).
- `crates/compagnon-cli/src/render.rs` (the init hint) and its tests.
- The installed skills change: `compagnon update` refreshes them, and a
  skill edited by hand stays preserved as today.
- Evaluation, per decision 0008: `compagnon-evals compare` on the steps
  touched — planning, implementation, update, sync, archive, configure —
  which calls a model and costs money.
- Documentation: the init output shown in the quickstart and the
  "Archiving" step of the Roles chapter, in both editions; the "How to
  ask" sections already describe the plain-words requests.

## Domains

- [ ] security
- [ ] personal-data — the match is "téléphone" in the example scenario of the French home page (`docs/site/fr.html`), not personal data
- [x] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Changed: Compagnon's skills now suggest each next step in plain words — "say “implement it”" — with the slash command as an optional shortcut, so you can go through a change without learning the commands.
