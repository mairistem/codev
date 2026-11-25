# Design: Skills suggest the next step in plain words

## Context

The skills are rendered from `assets/workflows/<id>.md`, with shared text
in `assets/workflows/fragments/` included through `{{fragment:<name>}}`
(`crates/compagnon-agents/src/workflows.rs`), and a description per
workflow in the `CATALOG` of the same file, written into each skill's
frontmatter. Today the six skills in scope name `/compagnon-…` commands in
about twenty places: next steps (apply → archive, sync → archive, update →
apply or archive), refusals (planning incomplete → propose, validation
failed → `compagnon validate`, review blocked → `compagnon review`), and
boundary prose ("that is `/compagnon-apply`'s job"). `propose` already ends
on a plain sentence ("ask me to apply this change").

The evaluation suite scores some of these replies by their text: an
implementation refusal must name `/compagnon-propose`
(`evals/src/implement.rs`), an archive or sync refusal must name
`compagnon validate` (case `archive-validation-failed`), and a configure
case expects "No changes made". Decision 0008 makes `compagnon-evals
compare` on the touched steps a condition of merging; decision 0004 keeps a
single identity for a skill and its slash command, which this change does
not alter.

## Goals / Non-Goals

**Goals**

- One rule, stated once, that the six skills follow for every next step
  they suggest.
- Descriptions that let Claude Code pick a skill from a plain request in
  English or French.
- Checks that run without a model, on the rendered text.

**Non-Goals**

- The bodies of `explore` and `onboard`: not covered by the suite yet; a
  following change covers them, then converts them.
- The `fix:` lines of `compagnon doctor` and the CLI's other messages:
  they are read in a terminal, where the commands are the vocabulary; only
  the closing hint of `compagnon init`, the first thing a beginner reads,
  changes.
- Translating the skills: they stay in English and already tell Claude to
  answer in the user's language; the plain request is spoken in that
  language.

## Decisions

### Decision: A shared fragment states the rule; each skill writes its next steps as request, then shortcut

A new fragment, `fragments/next-steps.md`, says: when you suggest what the
user can do next, give first the words to say to you, in plain language
and in the language the user writes in, then the command in parentheses as
a shortcut — `say “archive it” (shortcut: /compagnon-archive)`; never
present a command as the only way. The six bodies include it near their
output section, and each place that suggests a step is rewritten in that
form, with the plain request written in English in the body for Claude to
say in the user's language. Boundary prose that only names another skill
("that is `/compagnon-apply`'s job") names the step instead ("that is the
apply step's job"), so that a slash command left in a body is always a
shortcut.

Every command named today stays in the reply, as the shortcut: the
evaluation expectations that look for `/compagnon-propose` or
`compagnon validate` stay valid, and the quality comparison of decision
0008 measures the same things as the baseline.

**Alternatives considered**: removing the commands from the replies — it
breaks the suite's expectations and takes the shortcut away from users who
know it; writing the rule once in each skill instead of a fragment — six
copies drift, which is why the fragments exist; generating the plain
phrases in Rust — the phrase depends on the user's language, which only
the model knows.

### Decision: Descriptions carry quoted example requests, English in straight quotes, French in guillemets

Each description keeps what the skill does and ends with
`Use for requests like "…", "…", « … »`: at least one English request in
straight double quotes and one French request in « » guillemets. The
check is mechanical: a description must hold a `"…"` pair and a `« … »`
pair, and stay within 1024 characters, Claude Code's limit for a
description. The YAML rendering already quotes and escapes the description
(`yaml_scalar`), and the existing test that parses the frontmatter keeps
guarding it.

**Alternatives considered**: a separate `triggers:` field — Claude Code
matches on the description only; listing requests in every language —
the description would outgrow its limit, and Claude matches across
languages from a few examples.

### Decision: The rule is checked on the rendered text, without a model

`compagnon-agents` tests, on the bodies as rendered: each of the six
includes the `next-steps` fragment; every line naming `/compagnon-` holds
the word `shortcut`, the configure marker `# written by
/compagnon-configure` excepted. A pure function returns the offending
lines, tested on synthetic bodies. The descriptions are checked the same
way. Whether the model then follows the rule is what the evaluation run
measures, on the same cases as the baseline.

**Alternatives considered**: new evaluation expectations on the plain
phrase — a changed expectation makes the rescored baseline fail by
construction (`evals/src/checked.rs`), and the phrase varies with the
language; leaving the check to review — it would drift with the next edit.

### Decision: The evaluation runs on the six touched steps, with the user's agreement on the cost

Per decision 0008, the tasks end with `compagnon-evals run` then `compare`
on the steps `plan.write`, `plan.update`, `code.implement`, `change.sync`,
`change.archive` and `configure.enrich`, against the committed baseline, and the pull request carries the output of
`compare`. The run calls a model and costs money — a planning case about
1.65 USD, repeated — so it starts only once the user has agreed to its
estimated cost; descriptions, which only steer the choice of a skill, are
not measured by the suite, which runs each skill by name.

**Alternatives considered**: skipping the evaluation as for the rename —
this change alters what the agents read and say, the case decision 0008
was written for; running it on every case — the untouched review roles
would cost the most and measure nothing new.

## Risks / Trade-offs

- [The model drops the shortcut and an expectation such as
  `compagnon validate` fails] → the fragment says to keep the command, the
  refusal texts the evals check keep it verbatim after the plain sentence,
  and `compare` reports any case under its baseline share.
- [A longer body costs tokens on every call] → the fragment stays a few
  lines; `compare` shows the cost per step, and a rise beyond the
  baseline's spread is reported in the pull request (decision 0010 does
  not seek savings in this text, but the cost is still shown).
- [Example requests in a description make Claude start a skill on a loose
  match] → the examples are specific to each step ("archive it", not
  "continue"); the description still says what the skill does and what it
  refuses.
- [Installed skills keep the old text] → `compagnon update` and
  `compagnon self-update` refresh them; a skill edited by hand stays
  preserved, as today.
