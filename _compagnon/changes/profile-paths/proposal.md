# Proposal: One question to find your profile, one short path per profile

## Why

Five experts and six test readers went through the documentation: finding
one's profile is still too slow — five overlapping profiles, one of them a
level ("discovering AI agents") rather than an activity, an English-only
home page, three entry points with three different lists — and once on a
path, the reader does not know where they are. People who do not code but
want Claude to build something for them have no profile at all.

## What Changes

- "Choose your path" becomes one question, "What will you do with
  Compagnon?", with four answers phrased as activities, each linking to a
  page of its own:
  - "I review and approve plans — I do not code";
  - "I do not code, but I want Claude to build something for me" (new);
  - "I am a developer and I have Claude write code";
  - "I set Compagnon up for my team";
  and, below, a discreet link for those who want to change Compagnon
  itself. "Discovering AI agents" stops being a profile.
- Each profile gets a short path page: who it is for in one sentence, how
  long it takes, at most three numbered steps with one link each, labelled
  with the exact title of the page it opens, then "Next, if you need it",
  and a way back to the other paths.
- The "Start here" part lists the introduction, Choose your path with its
  four path pages, and the glossary; the quickstart moves to the head of
  "Everyday use", out of the way of readers who do not code.
- The home page of the site gets a French edition, and both editions ask
  the same question first, before any installation or technical content;
  the READMEs and the introduction use the same four labels, linking the
  same path pages.
- Pages read mostly by developers (the workflow, the skills, concepts, the
  FAQ) drop the explanations in parentheses of everyday developer words
  (terminal, test): the glossary link stays for whoever needs it.
- Links whose label differs from the page they open ("Step by step" for
  "The workflow") are fixed.

## Capabilities

### New Capabilities

### Modified Capabilities

- docs

### Removed Capabilities

## Impact

- `docs/en/src/` and `docs/fr/src/`: `choose-your-path.md`, four new
  `paths/*.md` pages, `SUMMARY.md`, `introduction.md`, the four developer
  chapters, and the links that announce a chapter under another name.
- `docs/site/index.html` and a new French home page, `docs/site/fr.html`;
  `README.md` and `README.fr.md`.
- `crates/compagnon-cli/src/docs_checks.rs` and
  `crates/compagnon-cli/tests/repository.rs`: the checks on the Start here
  part, the profiles, the path pages, the home pages and the READMEs.
- The offline manual (`compagnon docs`) follows the new table of contents.
- The missing pages the readers asked for (a commented example plan,
  giving feedback without Claude Code, evaluating Compagnon, adopting it on
  an existing project, deploying it for a team) are a following change;
  the vocabulary of the technical pages, another.

## Domains

- [ ] security
- [ ] personal-data — the match is "téléphone" in the example scenario of the French home page (`docs/site/fr.html`), not personal data
- [x] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Changed: The documentation asks one question — what will you do with Compagnon? — and gives each of four profiles a short path of at most three pages, including a new one for people who do not code but want Claude to build something for them. The site's home page now exists in French.
- Removed: The "Discovering AI agents" profile; links to the former profile sections of Choose your path (`#i-am-discovering-ai-agents`…) now open the page at its top.
