## ADDED Requirements

### Requirement: Every entry point says that a plan can start from a Jira ticket

Both home pages (`docs/site/index.html`, `docs/site/fr.html`), both READMEs,
and both editions of `introduction.md` and `quickstart.md` SHALL say, in
plain words, that naming a Jira ticket in a request — the example
"PROJ-123" — makes Claude read the ticket and cite it at the top of the
plan, and SHALL link the Jira guide of their language (`guides/mcp.md`,
or its published `guides/mcp.html` from the home pages and the READMEs).
On the home pages, this mention MUST come after the four profiles of
Choose your path, so that the question still comes first. The "Next, if
you need it" section of the four path pages SHALL link the Jira guide,
with its title as the link text.

#### Scenario: Every entry point mentions Jira and links the guide

- **GIVEN** both home pages, both READMEs, and both editions of
  `introduction.md`, `quickstart.md` and the four path pages
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds "Jira" and a link to the Jira guide of its language
- **AND** on each home page, the last of the four profile labels comes
  before the first mention of Jira

#### Scenario: An entry point without Jira is caught

- **GIVEN** an introduction that never names Jira, and a home page naming
  Jira between its first and its last profile
- **WHEN** the check runs on them
- **THEN** each fails naming the page

### Requirement: The Jira guide opens with what it changes for the reader

The Jira guide SHALL be titled "Start from a Jira ticket" in English and
« Partir d'un ticket Jira » in French — its `# ` heading and its entry in
`SUMMARY.md` — and SHALL be listed in the "Everyday use" part, keeping
its file `guides/mcp.md` so that existing links still resolve. Before its
first configuration section, it MUST show what the reader says (a request
naming a ticket), what Claude does (reads the ticket, once, read-only)
and what the reader gets (the plan citing the ticket), with the diagram
"From a Jira ticket to a plan". The configuration, the read-only rules
and the section on other MCP servers follow.

#### Scenario: The guide is in Everyday use and leads with the benefit

- **GIVEN** both editions of `SUMMARY.md` and `guides/mcp.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** the guide's entry is in the "Everyday use" part, titled
  "Start from a Jira ticket" / « Partir d'un ticket Jira »
- **AND** the guide holds a diagram before its first `## ` heading

#### Scenario: A guide that configures before it shows is caught

- **GIVEN** a Jira guide whose first `## ` heading comes before its
  diagram
- **WHEN** the check runs on it
- **THEN** it fails naming the guide

### Requirement: The documentation's diagrams are accessible in every rendering

The documentation SHALL hold three diagrams, in both editions: the
workflow (ask, plan, approve, build, file away) in `introduction.md` and
`workflow.md`; "From a Jira ticket to a plan" in `guides/mcp.md` and on
both home pages; the four paths in `choose-your-path.md`. Each diagram, in
a chapter or on a home page, MUST be an inline `<svg>` inside a
`<figure>` with `role="img"` and an `aria-label` in the page's language
naming every step or node in reading order (for example "Five steps: you
ask, Claude plans, you approve, Claude builds, the change is filed
away"), followed by a `<figcaption>`; it MUST reference no external file, draw
its text as text (no outlined letters), and take its colors from the
Mairistem palette through CSS custom properties, so that it reads in the
light and dark themes of the books, of the home pages and of the offline
manual. In both themes, its text MUST reach a contrast of at least 4.5:1
against the fill it sits on, and its lines and arrows at least 3:1
against the page background. Every word of the French diagrams is
French.

#### Scenario: Every diagram has a text alternative and no external file

- **GIVEN** every chapter of both editions and both home pages
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each diagram `<svg>` sits in a `<figure>`, carries
  `role="img"` and a non-empty `aria-label`, and is followed by a
  `<figcaption>`
- **AND** none holds `href` or `src` to another file

#### Scenario: A diagram without alternative is caught

- **GIVEN** a chapter holding an `<svg>` with no `aria-label`, and a
  home page whose diagram has no `aria-label`
- **WHEN** the check runs on them
- **THEN** each fails naming its page

#### Scenario: Every diagram color pair is contrasted enough

- **GIVEN** the diagram colors defined for the light and dark themes of
  the books, of the home pages and of the offline manual
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** every text color reaches 4.5:1 against the fill it is drawn
  on, and every line color 3:1 against the page background
- **AND** a pair below its threshold fails naming the theme and the two
  colors

#### Scenario: The offline manual draws the diagrams in both themes

- **WHEN** the user runs `compagnon docs --write manual.html`
- **THEN** `manual.html` holds the workflow diagram's `<svg role="img"`
  inline, unescaped
- **AND** its styles define the diagram colors for the light scheme and
  for `prefers-color-scheme: dark`

### Requirement: The design is approved on a mockup before it is applied

The new look of the home pages and of the books SHALL first be shown to
the user as a mockup — a home page and a book chapter, in light and
dark — and applied to `docs/site/` and `docs/theme/` only once the user
has approved it. The applied look MUST keep: the Mairistem palette, the
skip link, visible keyboard focus, text contrast of at least 4.5:1 in
both themes, a layout with no horizontal scroll at 320 px wide, and
every behavior the existing requirements give the home pages and the
books. The home pages follow the reader's color scheme
(`prefers-color-scheme`), with no switch of their own; the books keep
mdBook's theme menu. Nothing on the home pages moves on its own: the
example exchange with Claude is static text, present in the page from
the start.

#### Scenario: The applied look keeps the page usable

- **GIVEN** the published home pages and a book chapter
- **WHEN** they are opened at 320 px wide, in light and in dark
- **THEN** no page scrolls horizontally
- **AND** the skip link and the keyboard focus are visible

#### Scenario: A page wider than the screen is not approved

- **GIVEN** a restyled home page that scrolls horizontally at 320 px
- **WHEN** it is checked before the restyle is committed
- **THEN** the restyle is not committed until the layout is fixed

## MODIFIED Requirements

### Requirement: Compagnon's mark identifies the product in every documentation surface

The repository SHALL hold the Compagnon mark as SVG files under
`docs/assets/`: a color version for light backgrounds and a version for
dark backgrounds, drawn only in the Mairistem palette (`#ff5331`,
`#ffab03`, `#004c66`, `#00c39b`, plus black or white). It SHALL also hold
the "Compagnon by Mairistem" logo — the mark, the word "Compagnon" and
the line "BY MAIRISTEM" — in the same two versions. The logo MUST be
shown:

- at the top of `README.md` and `README.fr.md`, with the variant matching
  the reader's color scheme;
- in the header of the mdBook documentation, in both languages, in place
  of the Mairistem logo and the word "Compagnon" beside it;
- in the header of the landing pages `docs/site/index.html` and
  `docs/site/fr.html`, in place of the Mairistem logo and the word
  "Compagnon" beside it.

The Mairistem logo stays in the footer of the landing pages. The mark
MUST be shown:

- as the favicon of the documentation and of the landing pages;
- in the self-contained HTML file written by `compagnon docs`, inline, with
  no external asset.

A mark or logo that stands alone carries a text alternative naming
Compagnon: `alt` on an image, `role="img"` with `aria-label` on inline SVG
— a `<title>` inside the SVG file is not enough. A mark placed next to the
visible word "Compagnon", or a logo inside a link that has its own
accessible name, is decorative (`aria-hidden="true"`). The brand links
name Compagnon ("Compagnon, by Mairistem — home"; "Accueil du site
Compagnon" in French). The header logo switches to its dark-background
version in the dark themes.

#### Scenario: The offline manual shows the mark without fetching anything

- **WHEN** the user runs `compagnon docs --write manual.html`
- **THEN** `manual.html` holds the Compagnon mark as inline SVG with a text
  alternative naming Compagnon
- **AND** it references no external image

#### Scenario: Both READMEs show the mark

- **GIVEN** the repository's `README.md` and `README.fr.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails unless both reference the light and dark variants of
  the mark under `docs/assets/`, with a non-empty `alt` naming Compagnon

#### Scenario: A mark outside the palette is rejected

- **GIVEN** a mark file under `docs/assets/` using a fill outside the
  Mairistem palette, black and white
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming the file and the color

#### Scenario: The headers show the Compagnon by Mairistem logo

- **GIVEN** `docs/theme/index.hbs` (the sidebar brand and the menu bar
  brand), `docs/site/index.html` and `docs/site/fr.html`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each header brand link holds the "Compagnon by Mairistem"
  logo, and no Mairistem logo
- **AND** each brand link keeps its accessible name naming Compagnon

#### Scenario: A header still showing the Mairistem logo is caught

- **GIVEN** a home page whose header brand link uses the Mairistem logo
- **WHEN** the check runs on it
- **THEN** it fails naming the page

### Requirement: `compagnon docs --print` prints the markdown source to stdout

`compagnon docs --print` MUST print the **markdown source** of the
documentation to stdout, without conversion, except for diagrams: each
`<svg>…</svg>` element is left out, and the `<figcaption>` that follows
it stays, so that a terminal reader sees the caption instead of drawing
instructions. The output is useful
for piping into `less`, `bat`, or a tool that consumes markdown
(an LLM, for example).

This form MUST NOT open a browser, MUST NOT write any file, and
MUST return exit code 0.

#### Scenario: `--print` outputs non-empty markdown

- **GIVEN** the current binary
- **WHEN** the user runs `compagnon docs --print`
- **THEN** stdout carries non-empty content (> 1000 bytes — the docs
  have several sections)
- **AND** the content starts with a level-1 markdown heading
  (`# compagnon`)
- **AND** no file is written

#### Scenario: `--print` leaves out the drawings and keeps the captions

- **GIVEN** the current binary, whose introduction holds the workflow
  diagram
- **WHEN** the user runs `compagnon docs --print`
- **THEN** stdout holds no `<svg`
- **AND** it holds the diagram's `<figcaption>` text
