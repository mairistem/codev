# Design: Put Jira forward, draw the workflow, and give the documentation a modern developer look

## Context

- The books (`docs/en`, `docs/fr`, mdBook 0.5.4) share `docs/theme/`:
  `mairistem.css` (1 796 lines: tokens on `.light` and on `.navy`, the
  Mairistem dark variant), `header.hbs` (only the sprites `#mx-logo` and
  `#cp-mark`), `index.hbs` (the two brand spots: the sidebar brand,
  `.mx-sidebar-brand`, and the menu bar brand, `.mx-bar-brand` with the
  menu title), `mairistem.js`, IBM Plex Sans in `fonts/`.
- The home pages have no dark scheme today (`color-scheme: light`).
- The home pages `docs/site/index.html` and `fr.html` (840 lines each)
  carry their own inline CSS and the same two sprites; their header shows
  the Mairistem logo, then the mark and the word "Compagnon".
- The "Compagnon by Mairistem" logo is in `docs/assets/compagnon-logo.svg`
  and `compagnon-logo-dark.svg` (viewBox `0 0 1188 250`, mark + outlined
  word + "BY MAIRISTEM"); only the READMEs use it.
- The offline manual (`crates/compagnon-cli/src/docs.rs`) renders the same
  markdown through pulldown-cmark, which passes raw HTML through, with its
  own inline `<style>`; `compagnon docs --print` prints the markdown as is.
- The checks live in `crates/compagnon-cli/src/docs_checks.rs` (pages,
  vocabulary, French lexicon — which already strips `<svg>` through
  `without_svg`) and `crates/compagnon-cli/tests/repository.rs` (READMEs,
  home pages, marks, brand links).
- Jira appears only in `guides/mcp.md` ("Going further"), `paths/team.md`,
  `workflow.md`, `skills.md`, `faq.md` and the reference.

## Goals / Non-Goals

Decisions in effect: 0014 (the product is named Compagnon) governs the
logo and the brand links' names; 0001 (functional core, imperative shell)
puts the SVG-stripping and contrast functions in pure functions of
`compagnon-cli`'s docs modules, tested without I/O. No other decision
applies.

**Goals**: Jira visible from every entry point; three diagrams that work in
the books, on the home pages, offline and in a terminal; a modern look
approved on a mockup first; the new logo in the headers.

**Non-Goals**: no change to the Jira integration, the skills or the
commands; no new page beyond the guide's retitling; no animation library,
diagram generator (mermaid, kroki) or web font beyond IBM Plex Sans; no
redesign of the reference pages' content.

## Decisions

### Decision: The look is approved on a mockup before it touches the theme

The first task builds a static mockup in the change folder
(`_compagnon/changes/jira-and-modern-docs/mockup/`): the English home page
and one book chapter (the Jira guide, with its diagram), each with a
light/dark switch, published as a private page for the user to look at.
Implementation stops there until the user approves or asks for changes;
only then are `docs/site/` and `docs/theme/` restyled, copying the
approved CSS. The mockup stays in the change folder as the record of what
was approved.

Direction of the mockup, for a developer audience working with AI:

- home page hero on a dark Mairistem blue (`#00263a`→`#004c66`) with a
  faint grid, the promise, and a **terminal card** showing a short
  exchange as static text in a `<figure>` with a `<pre>` and a caption —
  no typing animation, nothing injected later: `> Prepare a plan for
  PROJ-123`, then Claude's lines (reads the ticket, writes
  `proposal.md`, `specs/`, `design.md`, `tasks.md`, "Ready for your
  review");
- monospace accents (labels, eyebrows, step numbers) in the system
  monospace stack already declared (`--mono-font`), no new font file;
- cards with a 1 px border, 14 px radius and a coloured top rule per
  profile (the four Mairistem colours), a "Start from a Jira ticket"
  band with its diagram, code blocks with a title bar;
- books: same tokens, sharper headings, callouts as cards, code blocks
  with a header strip, sidebar with clearer active state; light stays the
  default theme, `navy` the dark one;
- home pages: a dark scheme under `prefers-color-scheme: dark`, no switch
  of their own (the mockup's switch only previews both schemes).

**Alternatives considered**: restyling directly (rejected: the user asked
to validate the design first); a CSS framework such as Tailwind (rejected:
a build step and a dependency for two pages and one theme).

### Decision: Diagrams are hand-written inline SVG with themed custom properties

Each diagram is an inline `<svg>` in the markdown, inside
`<figure class="diagram">`, with `role="img"`, an `aria-label` in the
chapter's language, a `viewBox` and `width="100%"`, followed by a
`<figcaption>`. Shapes take their colours from custom properties with a
light fallback — `style="fill:var(--dg-box,#f4f8f9)"`, `--dg-ink`,
`--dg-line`, `--dg-accent-*` — defined for `.light` and `.navy` in
`mairistem.css`, for both schemes in the home pages' CSS, and for
`prefers-color-scheme` in the offline manual's inline CSS. Text is
`<text>` in the page font, so it stays translatable; as `role="img"`
hides it from assistive technology, the `aria-label` names every step or
node in reading order. Text is always drawn in ink on a light tint (dark
theme: light ink on a dark tint); the Mairistem colors mark borders,
dots and arrows, never the fill under text — white on coral would be
3.2:1. The SVG holds no blank
line (a blank line ends a CommonMark HTML block and would break it) and
no `href`.

The three diagrams:

1. **Workflow** — five steps on two rows, narrow enough to stay
   legible at 320 px: You ask → Claude plans → You approve → Claude
   builds → Filed away, the steps where you decide in coral and
   Claude's in blue.
2. **From a Jira ticket to a plan** — ticket card `PROJ-123` → Claude
   (one read-only call) → `proposal.md` whose first line cites the ticket.
3. **Four paths** — the question in the middle and the four profiles
   around it, each with its colour and number of pages.

The same SVG source is reused on the home pages (copied, with the
home page's CSS variables).

**Alternatives considered**: mermaid (rejected: needs a script, does not
render in the offline manual or in GitHub's rendering of a raw HTML
block, and gives little control over the palette); separate `.svg`
files through `<img>` (rejected: the offline manual must reference no
external file, and an `<img>` cannot follow the page theme); PNG
(rejected: not themable, blurry, untranslatable).

### Decision: The Jira guide keeps its file and moves to Everyday use

`guides/mcp.md` keeps its path, so the published URL, the links from
`skills.md`, `faq.md`, the reference and outside stay valid. Its title
becomes "Start from a Jira ticket" / « Partir d'un ticket Jira », its
entry moves from "Going further" to "Everyday use" (after "Reviewing a
plan"), and its order becomes: what you say, what Claude does, what you
get (with the diagram) → "Set it up" (the existing configuration) → the
rules (one call, read-only, first ticket only) → "Other MCP servers".
Moving to Everyday use subjects it to the vocabulary check (first use of
listed terms linked to the glossary), which the rewrite honours. Links
elsewhere that name the old title are updated to the new one.

**Alternatives considered**: renaming the file to `guides/jira.md` with an
mdBook redirect (rejected: `output.html.redirect` does not cover the
offline manual's anchors nor links from outside to the French edition,
for no gain to the reader).

### Decision: One short Jira sentence per entry point, in the same words

Each entry point gets the same idea, sized to its place: home pages, a
"Start from a Jira ticket" band after the profiles, with the diagram and
a link to the guide; READMEs and introduction, one sentence and a link;
quickstart, a "You have a Jira ticket?" note in step 3; developer and
reviewer paths, one item in "Next, if you need it"; team path, its
existing item retitled. The example is always "Prepare a plan for
PROJ-123" / « Prépare un plan pour PROJ-123 ». The README's word caps
(600 / 750) still hold.

### Decision: The header shows the "Compagnon by Mairistem" logo

In `index.hbs` (the sidebar brand and the menu bar brand) and in both home
pages' headers, the `#mx-logo` + `#cp-mark` + "Compagnon" brand gives way
to a `#cp-logo` symbol, added to the sprites of `header.hbs` and of the
home pages, built from
`docs/assets/compagnon-logo.svg`, whose ink paths use
`var(--cp-logo-ink,#000)` (white under `.navy`, on the dark hero and in
dark scheme), so one symbol serves both themes. Inside the brand link,
which keeps its accessible name, the logo is `aria-hidden="true"`. The
`#mx-logo` symbol stays only where the footer uses it; the books' header
no longer needs it. The README already uses the logo files. The existing
test `inline_marks_are_named_or_decorative` (which requires
`<use href="#cp-mark">` on each page) is updated to accept `#cp-logo` as
the page's mark, decorative inside a named brand link.

### Decision: `--print` leaves the drawings out

`compagnon docs --print` passes the concatenated markdown through a
function removing each `<svg>…</svg>` element (the same scan as
`without_svg` in `docs_checks.rs`, moved to `docs.rs` and shared); the
`<figure>` and `<figcaption>` stay, so a terminal or an LLM reads the
caption. The HTML rendering keeps the SVG. The `--print` help in `main.rs`, the
module doc of `docs.rs` and both editions of `reference/cli.md` say so.

### Decision: New checks guard the promises

In `docs_checks.rs` / `tests/repository.rs`:

- `jira_mentions`: each entry point of the spec holds "Jira" and a link
  to the guide; on home pages, after the last profile label;
- `diagram_problems(html)`: every `<svg>` is inside a `<figure>`, has
  `role="img"`, a non-empty `aria-label`, a following `<figcaption>`, no
  `href`/`src`, and is not split across HTML blocks; run over all
  chapters of both editions and both home pages;
- `contrast_problems`: reads the `--dg-*` values of each theme (books'
  `.light` and `.navy`, home pages' two schemes, the manual's two
  schemes) and computes the WCAG ratio of each text/fill pair (≥ 4.5)
  and each line/background pair (≥ 3);
- the French lexicon check also reads the `aria-label` and
  `<text>` contents of French diagrams instead of skipping them;
- the header check: brand link holds `#cp-logo`, no `#mx-logo`;
- the Jira guide check: title, part, diagram before the first `## `.

Each has a failing example, as the existing checks do. Each check lands
in the same task as the content it guards, so the suite stays green
between tasks.

What stays manual, and why: horizontal scroll at 320 px and visible focus
need a browser; the test suite has none and the repository has no
JavaScript toolchain, so adding Playwright to CI is out of this change's
scope. These are checked with Playwright by hand (it is installed in the
working environment) before each restyle is committed, and the
screenshots are sent to the user.

## Risks / Trade-offs

- [The mockup round-trip delays the work] → the content tasks (Jira
  mentions, guide, diagrams, checks) do not depend on the look and are
  done first; only the restyling waits for the approval.
- [Inline SVG in markdown is fragile: a blank line breaks it] → the
  diagram check renders each chapter through pulldown-cmark and fails if
  an `<svg>` is split across HTML blocks; `mdbook build` in CI also shows
  it.
- [Diagrams duplicated in EN, FR and two home pages drift apart] → they
  differ only in words, and each diagram is written in all its places in
  the same task.
- [Restyling breaks existing behaviours of the home pages and books] →
  the existing tests (profiles first, skip link, hreflang) run unchanged,
  the marks test is updated as said above; a Playwright pass at 320 px and 1280 px, light and dark,
  checks horizontal scroll and contrast before the restyle is committed.
- [The dark hero lowers contrast of coral links] → links on the hero use
  white or `#ffab03`, checked at 4.5:1.

## Migration Plan

Documentation only: published by the existing Pages workflow on merge.
Rollback is reverting the merge. URLs are unchanged.
