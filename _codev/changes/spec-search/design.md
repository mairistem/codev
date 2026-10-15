# Design: Search the specs and read one requirement

## Context

codev has no way to find a requirement but reading files: `codev list
--specs` names the capabilities, and the skills then `grep` and read whole
`spec.md` files. The parser (`codev-core::parser`) already gives each spec's
requirements — name, description, scenarios, span — and the language of its
keywords (`Spec.lang`, English or French today); the engine already loads
the main specs and the decision index, inherited decisions included, through
the ports. A study of this repository (BM25 prototype in Python, 25 queries)
measured the ranking choices below; the evaluation suite of
`evaluate-planning` measures their effect on planning.

## Goals / Non-Goals

**Goals**: a ranked search over requirements and decisions whose output
stays bounded; reading one requirement at a time; the planning skill using
both; the gain proven by `codev-evals compare` (decision 0008).

**Non-goals**: a session map or any hook (`SessionStart`,
`UserPromptSubmit`) — the study found the map costs tokens in every session
and is truncated past about 50 capabilities, and the raw prompt finds 13 to
23 % of the requirements; searching the code; a persistent index; semantic
(embedding) search, which would call a model or ship one; changing
`/codev-explore`, which the evaluation suite does not cover yet — decision
0008 asks for a step to be covered, by its own change, before it changes.

## Decisions

### Decision: BM25 over whole requirements, in a pure module of the core

The core gains a module that takes documents already read — an identifier,
a kind, a text — and a query, and returns the ranked identifiers: words
lower-cased, accents folded, common words removed, stems taken, then BM25
(k1 = 1.2, b = 0.75). A requirement is one document: its name, description
and scenarios, the scenarios not indexed apart. Decision 0001 holds: no
disk, no clock, the same input always gives the same ranking, ties broken
by identifier; decision 0002 holds: the engine reads the specs, the
decision index and the archive through the existing ports and hands their
text to the core.

**Alternatives considered**: ranking by the number of query words found,
as `grep` does — the study measured half the recall (0.29 against 0.54 at
5 results; 0.40 against 0.80 on decisions); weighting the requirement's
name three times — no measurable gain in the study, one parameter less;
indexing scenarios apart — more results naming the same requirement.

### Decision: The index is rebuilt at each call

The project's specs are a few hundred requirements: building the index in
memory takes 30 ms for this repository's 210 requirements and 9 decisions,
and 62 ms with `--archive` over its 53 archived changes (2.4 MB), measured
with a release build (`hyperfine`, ten runs each), so `codev search`
rebuilds it every time and keeps nothing
on disk — nothing to invalidate when a spec changes, nothing to commit or
ignore.

**Alternatives considered**: an index file under `_codev/` or a cache
directory — a file to keep in step with the specs, for no gain at this
size; revisit only if a project's search grows past a second.

### Decision: Snowball stemming with `rust-stemmers`, in the language of the specs

Words are stemmed with the Snowball algorithms of the `rust-stemmers`
crate, in the language the specs' keywords are written in (`Spec.lang`):
English, or French; a project whose specs mix languages is stemmed in
English. Decisions carry no structural keywords: they are stemmed in the
same language as the specs, so that one query matches both, an inherited
decision written in another language matching less well — a loss accepted
here, since the study measured a mismatch losing 60 to 75 % of the recall,
and reported by `language` in the JSON. The query is stemmed the same way, which is why the skills write
it in the language of the specs: the study measured a French query on
English specs losing 60 to 75 % of the recall. Accents are folded by a
fixed table of Latin letters, and the common words are two short fixed
lists, English and French, so that neither needs another crate.
`rust-stemmers` is pure code, depends only on `serde`, already in the tree,
and is MIT or BSD-3 licensed; the CI's `cargo audit` covers it.

**Alternatives considered**: a stemmer written in codev — a Porter stemmer
and a light French one, a few hundred lines to keep correct for no gain
over a stable implementation of the reference algorithms; no stemming —
the study measured it loses the plural/singular and conjugated forms, the
French queries most.

### Decision: The archive is a separate ranking, read only on request

`--archive` builds a second index from the archived changes, one document
per change (proposal and design), ranked on its own and printed after the
requirements and decisions. The study measured that one shared ranking
halves the requirements' recall: a long archived design dominates the
scores. Without the flag the archive is not read.

**Alternatives considered**: one ranking with a weight on archived
documents — a weight to tune per project, where two lists need none.

### Decision: `codev show` reads one requirement from the parsed spec

`codev show <capability>::<requirement>` slices the requirement from the
spec's source by the span the parser gives — heading, description and
scenarios, byte for byte — so that what an agent copies into a `MODIFIED`
block is exactly the main spec's text. `codev show <capability>` prints the
purpose and the requirement names. The `<capability>::<name>` form is the
one the evaluation suite and `codev search` already use.

**Alternatives considered**: `codev search --full`, printing the matching
requirements whole — the output would grow with the results and lose the
bound that makes the search cheap; extending `codev list --specs` — a list
command that prints content would blur both.

### Decision: The skills search first, then read what the search names

`assets/workflows/propose.md` (step 4c, `plan.investigate`) gains a
short paragraph: run
`codev search` with words in the language of the specs, search again with
other words when the results miss the request's subject, read the named
requirements with `codev show`, read a whole spec only when the change
rewrites much of it, and never treat a result list as having read the
requirement. The paragraph stays short: the skill is read in every planning
run (12 % of its tokens today), and every added line is paid each time.
Decision 0004 holds: the skill keeps its single identity; its
`allowed-tools`, `Bash(codev:*)`, already covers both commands.
`/codev-explore` would gain the same paragraph, but only once the suite
measures it (decision 0008).

**Alternatives considered**: the investigator role's criteria — they apply
at review steps too, where no search is due; a `CLAUDE.md` line — read in
every session, including those that never plan.

### Decision: The change is judged on planning and on review

Following decision 0008, the change is measured with `codev-evals` against
the baseline that `evaluate-planning` records: `run --step plan.write
--repeat 3`, compared with `compare` — the primary recall and the other
planning measures within the baseline's range or better, the cost under its
floor; and `run --step review --repeat 3` compared with `--quality-only`,
since the `propose` skill the review cases read grows by a paragraph. The
pull request pastes both comparisons and the sources table, where `search`
and `file` on the specs should fall and `codev search`, `codev show` appear.

**Alternatives considered**: measuring planning alone — the review steps
read the same skill and could pay for its longer text unseen.

## Risks / Trade-offs

- [The agent stops at the result list and misses a requirement — recall@5
  is 0.54] → The skills say the search points to where to read and require
  reading the requirement to modify; `compare` refuses a primary recall
  under the baseline's minimum.
- [The agent keeps `grep`-ing as before and the search only adds calls, as
  in the study] → The measure shows it: `codev search` in the sources table
  without `search`/`file` falling, and a cost above the floor, fail the
  verdict; the skill paragraph is then revised or the change dropped.
- [A French query on English specs finds little] → The skills require the
  query in the language of the specs; `language` in the JSON tells which.
- [`rust-stemmers` is no longer actively developed] → It implements fixed
  reference algorithms, has no I/O and no unsafe network surface; `cargo
  audit` watches it, and the module isolates it behind one function should
  it need replacing.
- [An inherited decision written in another language than the specs is
  found less well] → Accepted and stated: `language` in the JSON says which
  stemming was used; a per-source language can come later if a project
  needs it.
- [`--archive` grows with the project's history — 53 archived changes,
  about 2.4 MB here — and the skill may search several times] → The
  archive is read only on request, and the timing task measures it; past
  one second, a persistent index is revisited.
- [A longer skill costs every planning and review run] → The paragraph is
  kept to a few lines, and the review comparison reports its cost.

## Migration Plan

Nothing to migrate: two commands and two JSON documents are added. Users
get the new skills with `codev update`; a skill rendered by this version
calls commands this version provides (`metadata.version`).
