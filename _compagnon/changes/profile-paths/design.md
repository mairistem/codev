# Design: One question to find your profile, one short path per profile

## Context

The documentation (mdBook, `docs/en` and `docs/fr`, same chapters) opens
with a "Start here" part — introduction, Choose your path, the quickstart,
the glossary — whose structure, caps and vocabulary are tested in
`crates/compagnon-cli/src/docs_checks.rs`; the chapters of "Start here" and
"Everyday use" are the "everyday pages" whose terms and French wording are
checked. The offline manual (`compagnon docs`) embeds the chapters in the
order of `SUMMARY.md` and maps each chapter's anchors to the single page.
The site's home page, `docs/site/index.html`, is English only; the docs
workflow copies `docs/site/` over the built site, after mdBook wrote
`site/en/` and `site/fr/`. The panel of experts and test readers who
prompted this change agreed on: one question, answers phrased as
activities, at most three steps per path, link labels equal to page
titles, a French home page, a register per audience.

## Goals / Non-Goals

**Goals**

- A reader recognises their profile in one question and reaches a path of
  at most three pages, in their language.
- Every entry point — home page, README, introduction, Choose your path —
  uses the same four labels.
- Developer pages stop over-explaining.

**Non-Goals**

- The new pages the readers asked for (commented example plan, feedback
  without Claude Code, evaluating Compagnon, adopting on an existing
  project, deploying for a team): a following change, which will add them
  to the paths' "Next" sections or steps.
- The French vocabulary of the technical pages (reference, roles, guides):
  another change.
- Remembering the reader's profile between visits: no cookie, no script.

## Decisions

### Decision: Four profiles phrased as activities, each with its own page

The question is "What will you do with Compagnon?"; the four answers are
what the reader does, not who they are, so that nobody fits two: review
and approve plans without coding; have Claude build something without
coding (a profile no page served); develop with Claude; set Compagnon up
for a team. Contributors, few and technical, get one sentence below the
four, pointing to the Contributing part. Each answer opens a page of its
own under `paths/` — `review.md`, `build.md`, `develop.md`, `team.md` —
rather than an anchor in one long page: a link to a page works the same on
the site, in the offline manual and from the home page, and mdBook's
"next" button inside a path page leads to the next path, not into a
chapter.

| Path | Steps | Next, if you need it |
|---|---|---|
| Review and approve | introduction, Reviewing a plan, Glossary | FAQ |
| Build without coding | introduction, Your first change, Reviewing a plan | Glossary |
| Develop | Your first change, Claude Code skills, The workflow | Concepts, FAQ |
| Team | Installation, Repository integration and compagnon doctor, Roles | Configuration, MCP servers, Inherited sources |

The quickstart leaves "Start here" for the head of "Everyday use": its
first step installs a tool, a dead end for a reader who only approves
plans; the build path still reaches it, its commands limited to installing
and `compagnon init` as the quickstart's own requirement already
guarantees, and its "Next" names no developer chapter (the skills chapter
asks to run `compagnon update`).

**Alternatives considered**: keeping "Discovering AI agents" — a level, not
an activity, which every reader could claim; a profile for decision makers
— the owner chose not to; a table of contents organised by audience —
moves every chapter and its links for what four short path pages give;
anchors in one page — they break with accented French anchors in the
README, and the manual's next button leads elsewhere.

### Decision: The French home page is a sibling file, `fr.html`

`docs/site/fr.html` sits next to `index.html`, each linking the other in
its header. `fr/index.html` would be overwritten by — or overwrite — the
French book mdBook builds at `site/fr/`. Both pages put the question and
the four profiles right under the title, before the install commands and
the technical sections, with plain HTML links.

**Alternatives considered**: choosing the language with a script from the
browser's settings — fails without script and in caches; one bilingual
page — doubles its length for every reader.

### Decision: The checks follow the new structure

`docs_checks.rs` replaces the five-profile check with
`choose_path_problems` (four labels in order, one link each to its path
page, a link to the Contributing part after them; each path page and its
`SUMMARY.md` entry titled with its label, word for word, so that a link,
a heading and the sidebar always agree) and adds
`path_page_problems` (one to three numbered steps, one link each, link text
equal to the target chapter's `# ` title, a link back to Choose your path).
`START_HERE`, the caps and the everyday-pages list follow the new
`SUMMARY.md`; a check on the four developer chapters rejects the glosses of
"terminal" and "test". `crates/compagnon-cli/tests/repository.rs` checks
that both home pages and both READMEs hold the four labels of their
language, linking the path pages, the first label before the first
install command on the home pages.

**Alternatives considered**: checking only by review — the labels drift
between four entry points, which is how the old lists diverged.

## Risks / Trade-offs

- [A reader who follows a path and opens a chapter shared by several paths
  does not see "step 2 of 3" there] → the path page stays short enough to
  come back to, and each step's link names the page exactly; per-path
  banners on shared chapters would need one variant per path.
- [The old Choose your path anchors (`#i-am-discovering-ai-agents`…),
  linked from the READMEs and the home page, stop existing] → those links
  are rewritten by this change; a bookmark to an old anchor still opens
  Choose your path, at its top. Documentation anchors are not a public
  interface, so this is not a breaking change.
- [Moving the quickstart out of Start here breaks a link that targets its
  position] → links target file paths, which do not move; the manual
  order test is updated.
- [The French home page drifts from the English one] → the repository test
  checks both for the same four profiles and order; the rest of the page
  is reviewed by hand at 375 px, as for the English page.
- [Removing glosses leaves a newcomer developer without an explanation] →
  the glossary link on first use stays.
