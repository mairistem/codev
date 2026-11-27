# Writing guide for the refactor (tasks 4.x)

Shared by everyone who rewrites a chapter of this change. Read the change's
`proposal.md`, `specs/docs/spec.md`, `design.md` and `inventory.md` first.

## The reader

Accessible first: a person who has never used an AI coding assistant must
follow every Start here and Everyday page. Deeper on demand: what only some
readers need goes in an "Under the hood" block (`<details><summary>Under the
hood: …</summary> … </details>`, French « Sous le capot : … ») or in a
linked chapter, never deleted when it is the only place it is said.

- Short sentences, active voice, the reader's words first; a technical term
  links to its glossary entry the first time (`glossary.md#<anchor>`).
- One topic, one place: the owner table in `design.md` ("One topic, one
  place") says which chapter owns what. Elsewhere, one sentence and a link.
- Each Everyday use chapter (example plan excepted) holds at most 1 200
  words in English, 1 500 in French, not counting fenced code, `<details>`
  blocks and `<figure>` blocks, and ends with a last section
  `## Go further` / `## Pour aller plus loin` listing links whose text is
  exactly the linked chapter's title (as in `SUMMARY.md`).
- The Start here caps stay: introduction 450 / 560 words, Choose your path
  200 / 250, each path page 250 / 310, quickstart 500 / 625.

## Both editions

Every change is made in `docs/en/src` and `docs/fr/src`, same structure,
same headings order. French uses "vous", and no anglicism: never "skill",
"spec", "workflow", "review", "commit", "design", "template", "change" as a
noun, "pull request", "merge request" in French prose, headings or diagram
labels — say compétence, spécification, déroulé, relecture,
enregistrement, conception, modèle, modification, demande de fusion. An
English term may follow in parentheses. Identifiers in inline code are
fine.

## Diagrams

Models: `mockup/jira-diagram.html`, `mockup/workflow-diagram.html`,
`mockup/plan-diagram.html` (approved). Each diagram in a chapter is:

```html
<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 H" role="img" aria-label="…every step or node, in reading order…">
…shapes…
</svg>
</div>
<figcaption>One sentence saying what the picture shows.</figcaption>
</figure>
```

- **No blank line anywhere between `<figure` and `</figure>`** (a blank
  line ends the markdown HTML block). One element per line is fine.
- Colors only through the tokens, as `style="fill:var(--dg-box)"`,
  `style="stroke:var(--dg-line)"`: `--dg-bg`, `--dg-box` (box fill),
  `--dg-file` (file card fill), `--dg-ink` (text), `--dg-muted` (secondary
  text), `--dg-line` (arrows, neutral borders), `--dg-you` (borders and
  dots where the reader decides), `--dg-on-you` (digits on a `--dg-you`
  dot), `--dg-claude` (borders and dots where Claude works; digits on it
  use `--dg-box`). The three Mairistem dots (`#ff5331`, `#ffab03`,
  `#00c39b`) may appear as small decorative dots only. Text is always ink
  or muted on box, file or bg — never on a colored fill.
- Text as `<text>`, 12 to 18 units, `class="mono"` for file names and
  identifiers; width 760 units; keep everything inside the viewBox.
- `aria-label` in the chapter's language names every step or node in
  reading order; the `figcaption` says what to take away.
- Arrow markers: the `id` must be unique in the whole book (the offline
  manual puts every chapter on one page): `id="dg-<chapter>-<n>"`, e.g.
  `dg-quickstart-arrow`, referenced as `marker-end="url(#dg-quickstart-arrow)"`.
  No other `href`/`src` than `#…`.
- Steps to open under a diagram (optional, used for the workflow):
  `<div class="diagram-steps">` holding `<details><summary>…</summary>…</details>`
  per step, each with a "Go further" link.
- The check `every_diagram_has_a_text_alternative` and the French lexicon
  check run on every chapter.

## Jira

The example request is always "Prepare a plan for PROJ-123" /
« Prépare un plan pour PROJ-123 »: Claude reads the ticket once, without
changing anything in Jira, and cites it at the top of the plan. Link
`guides/mcp.md` with the text "Start from a Jira ticket" / « Partir d'un
ticket Jira ».

## Constraints from the tests

Do not break what `inventory.md` lists under "Constraints from the tests".
Keep the anchors other pages use (`inventory.md`, "Most linked"); if you
must move one, update every link to it (`grep -rn 'file.md#anchor' docs`).
Do not edit `SUMMARY.md`, Rust files, the theme or the site: report what
you would need instead.

## Done

`cargo test -p compagnon-cli`, `mdbook build docs/en` and
`mdbook build docs/fr` pass. Do not commit; report the files you changed,
the word counts before and after, and anything left undone.
