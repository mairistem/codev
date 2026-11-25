# Design: Plain language on the everyday pages of the documentation

## Context

The change `accessible-docs` gave the documentation a "Start here" path,
a glossary and tests on its structure (`crates/compagnon-cli/src/docs_checks.rs`:
word caps, glossary terms, folded "Under the hood" blocks), but left the
wording of the everyday chapters as it was: technical terms on first use
("delta", "main specs", "artifact", "schema") and, in French, the
English vocabulary of the tool ("change", "skill", "spec"). The chapters
are embedded in the binary in the order of each `SUMMARY.md`
(`crates/compagnon-cli/build.rs`), so the offline manual follows the
website. Commands, paths and configuration keys are English identifiers
and stay so.

## Goals / Non-Goals

**Goals**

- Ten everyday chapters, in both editions, a reader who does not code can
  follow without the glossary open beside them.
- A French edition in French, with a lexicon fixed once.
- Checks that keep the listed terms and the anglicisms from coming back.

**Non-Goals**

- The "Going further", "Reference" and "Contributing" parts: their readers
  configure or change Compagnon and need the exact terms.
- The text of the skills: their French example requests belong to the
  active change `plain-words-in-skills`, whose plan is revised to follow
  this lexicon, with a check, because a change to a skill's text goes
  through the evaluation that change already runs (decision 0008).
- Renaming commands, folders or configuration keys (`/compagnon-apply`,
  `_compagnon/changes/`, `workflows:`): they are identifiers.
- Measuring readability with a score: the review roles judge it, the
  checks guard only what a machine can tell.

## Decisions

### Decision: Replace first, explain otherwise

Each chapter is rewritten for its reader: everyday words, one idea per
sentence, an example where a definition would be abstract ("the plan
lists what will be true afterwards: when the system is set to dark mode,
the application shows the dark theme"). A listed term is kept only where
the reader will meet it again — on screen, in a file — and its first use
in the chapter then links to the glossary. "Main specs" has an everyday
equivalent, "the project's specs", and so is never needed.

**Alternatives considered**: a glossary tooltip on every term — mdBook has
none, and the offline manual would lose it; a separate beginners' edition —
two texts to keep in step, which `accessible-docs` already rejected.

### Decision: A fixed lexicon for the French edition, English kept in parentheses

The French edition follows one lexicon: change → modification, skill →
compétence, spec → spécification, workflow → déroulé, review → relecture,
commit → enregistrement, pull request → demande de fusion. The English word
may follow its French equivalent in parentheses where the reader will see
it on screen — Claude Code lists "skills", GitHub says "pull request" —,
and stays in commands, paths and code. The French glossary entries take
the French names (Modification, Compétence, Spécification, Déroulé,
Relecture), each saying the English word it stands for.

**Alternatives considered**: keeping the English terms as the tool's
vocabulary — what the author asked to stop; translating identifiers too —
the reader would type `/compagnon-apply` and see `changes/` anyway, and a
translated name would not match the screen.

### Decision: Two pure checks on prose, reusing the chapter walker

`docs_checks.rs` already walks a chapter line by line, knowing fenced code
and `<details>` blocks. Two functions are added, each returning the
offending items and tested on synthetic chapters:

- `unexplained_terms(md, terms)`: for each term, its first occurrence in
  prose — outside headings, inline code, fenced code and `<details>` —
  must sit in the text of a link to the term's own entry,
  `glossary.md#<anchor>`, the anchor being one the glossary's `## `
  headings actually produce (mdBook's slug, the French one in the French
  edition, such as `#déroulé`), so that a keyboard or screen-reader user
  lands on the definition; "main specs" /
  "specs principales" is reported wherever it appears in prose.
- `anglicisms(md)`: the words of the lexicon's English side, matched as
  whole words, case-insensitive, in prose and headings, outside inline
  code, fenced code, `<details>` blocks and parentheses. "change" and
  "changes" are also forms of the French verb *changer* ("ce qui change"),
  so they count only as a noun: right after a determiner (le, la, les, un,
  une, des, du, au, aux, ce, cet, cette, ces, chaque, votre, vos, son, sa,
  ses, leur, leurs, quel, quelle, d', l').

Tests apply them to the ten everyday chapters of each edition (glossary
excepted for the first) and, for the anglicisms, to `README.fr.md`.

**Alternatives considered**: a spelling or style linter — a new tool for
the contributors to install, for two rules; checking every chapter — the
reference needs the exact terms.

## Risks / Trade-offs

- [The rewrite breaks the caps or the structure the existing checks
  guard] → the existing tests of `docs_checks.rs` run with every task;
  the Start here caps leave room.
- [A French chapter loses a needed English word] → the parentheses keep
  it beside its translation, and the glossary says which English word
  each entry stands for.
- [The anglicism check flags a legitimate French word] → "change" and
  "changes" are matched only after a determiner, so the verb ("ce qui
  change") passes, as a unit test shows; the noun « change » of a
  currency does not occur in this documentation.
- [The two editions drift] → each task rewrites a chapter in both
  editions; `both_languages_have_the_same_chapters` still holds.
