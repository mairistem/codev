# Design: Explore and onboard suggest the next step in plain words

## Context

`plain-words-in-skills` (archived 2026-11-26) gave six skills the
`next-steps` fragment (`assets/workflows/fragments/next-steps.md`): plain
request first, in the user's language and the French lexicon, the
command in parentheses as a shortcut, the reply closing on one quoted
request. Its tests in `crates/compagnon-agents/src/workflows.rs` run over
`PLAIN_WORDS_SKILLS` (six ids): the fragment is included, every line
naming `/compagnon-…` is a shortcut (`commands_not_given_as_shortcuts`,
the configure marker excepted), and each description names English and
French requests (`description_problems`), within 1024 characters.

`assets/workflows/explore.md` names `/compagnon-propose` in its outcome
("I can create the change — ask me for `/compagnon-propose`") and in its
guardrail. `assets/workflows/onboard.md` names `compagnon init`,
`/compagnon-configure`, `/compagnon-propose`, `/compagnon-apply` and
`/compagnon-explore` in its recommendation table, its example replies and
its guardrails, and quotes doctor's fix lines, e.g.
"`config.rules` — fix: `/compagnon-configure`", which it must relay
verbatim (requirement "The `onboard` skill relays what `compagnon doctor`
reports missing").

The nine evaluation cases recorded by `explore-onboard-evaluation` check
the final message for commands (`/compagnon-propose`,
`/compagnon-configure`, `/compagnon-apply <name>`, `compagnon init`) and
outcome words ("crystallizes", "information is missing", "not worth
pursuing"); the baseline passes them all.

## Goals / Non-Goals

**Goals:**

- The same rule, the same tests and the same closing line for all eight
  skills.
- The baseline's checks still meaningful: the commands they look for stay
  in the replies, as shortcuts.

**Non-Goals:**

- Changing what either skill recommends, or onboard's table of states.
- Rewording doctor's fix lines: they are the CLI's output.

## Decisions

### Decision: The two skills join the six, with their tests

`explore` and `onboard` include `{{fragment:next-steps}}` and join
`PLAIN_WORDS_SKILLS` (renamed by its count in the tests' names: "eight").
Every suggestion in their bodies is rewritten as "say “…” (shortcut:
`/compagnon-…`)", keeping the argument when the command had one
(`/compagnon-propose <that-name>`, `/compagnon-apply <that-name>`), so
that the cases' marks stay in the replies. Onboard's "what's next" block
closes on its one recommendation, quoted; for a thin configuration, the
later `/compagnon-propose` step is mentioned before that last line, which
is the request to configure. For several active changes it lists them,
each with its request and shortcut, and its last line asks which one,
without a shortcut — the closing rule's case of a choice.

Every other line of the two bodies naming `/compagnon-` is reworded so
that it is a shortcut or no longer names a command: onboard's opening
description (the user asks Claude in their own words; the commands are
shortcuts), the thin-config explanation, the non-case of a user who
already asked for the tour, and the guardrail against suggesting a skill
that is not installed. The paragraph "Make the suggestion actionable:
quote the exact command to type" becomes "give the request to say, then
the exact command as the shortcut", so that no instruction contradicts
the fragment.

Each missing check that onboard relays from `compagnon doctor` gets a few
plain words of its own before the verbatim `fix`
("your project's rules are not written yet — fix: `/compagnon-configure`").

### Decision: Doctor's fix lines are exempt as relayed output

`commands_not_given_as_shortcuts` skips a line holding `— fix:`, the form
in which onboard quotes doctor's output, as it skips the configure
marker; a unit test shows such a line is not reported. It is the only
new exemption: `— fix:` appears in onboard's body on the doctor lines
alone.

**Alternatives considered**: rewording doctor's `fix` texts in the CLI —
rejected, out of scope, and the CLI's output is English for scripts.

### Decision: Measured on the two steps, against the baseline

`run --step idea.explore --step project.onboard --repeat 5`, then
`compare` against `evals/baseline`: 45 runs at the baseline's 0.11 to 0.17
USD median, about 8 USD. A case whose mark the conversion legitimately
changes would be a defect of the conversion, not of the case: the marks
are commands, which stay.

### Decision: The case loading helper

`load_read_only` gets one helper `named<T>(field, value, all, as_str)`
that parses `outcome` or `state` from its list of names, giving the same
errors; the six loading tests stay as they are.

## Risks / Trade-offs

- [Onboard's longer replies cost more] → `compare` reports the cost; a
  rise within the baseline's spread is reported, not blocking (decision
  0008).
- [The closing request hides the doctor's list] → the list stays in the
  "here you have" block, before the last line.
