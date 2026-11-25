## ADDED Requirements

### Requirement: Compagnon's mark identifies the product in every documentation surface

The repository SHALL hold the Compagnon mark as SVG files under
`docs/assets/`: a color version for light backgrounds and a version for
dark backgrounds, drawn only in the Mairistem palette (`#ff5331`,
`#ffab03`, `#004c66`, `#00c39b`, plus black or white). The mark MUST be
shown:

- at the top of `README.md` and `README.fr.md`, with the variant matching
  the reader's color scheme;
- in the header of the mdBook documentation, in both languages, next to
  the Mairistem logo;
- in the header of the landing page `docs/site/index.html`, next to the
  Mairistem logo;
- as the favicon of the documentation and of the landing page;
- in the self-contained HTML file written by `compagnon docs`, inline, with
  no external asset.

A mark that stands alone carries a text alternative naming Compagnon: `alt`
on an image, `role="img"` with `aria-label` on inline SVG — a `<title>`
inside the SVG file is not enough. A mark placed next to the visible word
"Compagnon", or inside a link that has its own accessible name, is
decorative (`aria-hidden="true"`). The brand links name Compagnon
("Compagnon, by Mairistem — home"; "Accueil du site Compagnon" in French).

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

### Requirement: Moving from codev to Compagnon is documented in both languages

The documentation SHALL hold, in `docs/en` and `docs/fr`, a guide to
moving a project from codev to Compagnon: what is renamed (binary, planning
root, lock file, configuration key, skills, environment variables, JSON
codes, CI workflow), how to install `compagnon`, how to run
`compagnon migrate --dry-run` then `compagnon migrate`, what to do with a
shared source repository still laid out for codev, and what a hand-edited
`codev-*` skill becomes. The CLI reference SHALL document
`compagnon migrate`.

#### Scenario: The migration guide exists in both editions

- **GIVEN** the repository's `docs/en/src/SUMMARY.md` and
  `docs/fr/src/SUMMARY.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** both list the same migration guide chapter
- **AND** both chapters name `compagnon migrate --dry-run`

#### Scenario: A guide missing from one edition is caught

- **GIVEN** the migration guide listed in `docs/en/src/SUMMARY.md` only
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming the missing chapter
