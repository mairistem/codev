# Review record: jira-and-modern-docs

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The guide check was anchored on a Configuration heading the rewrite removes

```yaml
severity: medium
confidence: high
location: 'specs/docs/spec.md#Requirement: The Jira guide opens with what it changes for the reader'
evidence: |
  - **AND** the guide holds a diagram before its first heading naming
    configuration ("Configuration" / « Configuration »)
recommendation: 'Anchor the check on the first `## ` heading after the diagram, in the spec and the task.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: 'specs/docs/spec.md (diagram before the first `## `, new failing scenario); tasks.md 2.2'
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Checks written before their content left the suite failing across tasks

```yaml
severity: medium
confidence: high
location: 'tasks.md#2. Checks first'
evidence: |
  - [ ] 2.4 Update the header check in `tests/repository.rs`: brand link holds `#cp-logo` and no `#mx-logo`, keeps its accessible name; verified by `cargo test -p compagnon-cli --test repository` failing before task 5.1 and passing after
recommendation: 'Put each check in the same task as the content it guards.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: 'tasks.md regrouped: each check lands with its content (2.1-2.3, 3.1-3.3, 4.1-4.2); design.md checks section'
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The build path page was left without Jira

```yaml
severity: medium
confidence: high
location: 'proposal.md#What Changes'
evidence: |
  a link in the "Next" list of the developer and
    reviewer path pages (the team page already has one, reworded).
recommendation: 'Add paths/build.md to the requirement, its scenario and the task.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: 'proposal.md What Changes and Impact; spec entry-point requirement (four path pages); tasks.md 2.3'
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The --print output changes but the migration reason said no command changes

```yaml
severity: low
confidence: high
location: 'proposal.md#Domains'
evidence: |
  - [ ] migration — no configuration, data or command changes; the Jira guide keeps its URL
recommendation: 'Name the --print change in the reason and in the Impact, and say why it is harmless.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: 'proposal.md Domains reason, Impact and Release note'
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 Two scenarios had no automated test and no reason given for a manual check

```yaml
severity: medium
confidence: high
location: 'tasks.md#5. Apply the approved look (after 1.2)'
evidence: |
  verified by `cargo test -p compagnon-cli --test repository` (profiles first, hreflang, marks, Jira after profiles) and Playwright at 320 px and 1280 px, light and dark, without horizontal scroll, with visible skip link and focus
recommendation: 'Add kept tests where possible, and say why the rest stays manual.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: 'tasks.md 3.1 (contrast test), 3.2 (manual holds the svg unescaped), 4.2-4.3 (manual, reason given); design.md What stays manual'
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 The design cited none of the decisions in effect

```yaml
severity: low
confidence: high
location: 'design.md#Decisions'
evidence: |
  `compagnon docs --print` passes the concatenated markdown through a
  function removing each `<svg>…</svg>` element (the same scan as
  `without_svg` in `docs_checks.rs`, moved to `docs.rs` and shared)
recommendation: 'Cite 0014 and 0001, or say none applies.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: 'design.md Goals / Non-Goals: decisions 0014 and 0001 cited'
  - status: fixed
    by: skeptic
```

#### R-skeptic-7 The final check ran narrower commands than the definition of done

```yaml
severity: low
confidence: high
location: 'tasks.md#6. Final check'
evidence: |
  - [ ] 6.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --workspace`
recommendation: 'Use `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings`.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: 'tasks.md 5.1'
  - status: fixed
    by: skeptic
```

### accessibility

#### R-accessibility-1 The home-page copy of the Jira diagram was outside the text-alternative requirement

```yaml
severity: medium
confidence: high
location: 'specs/docs/spec.md#Requirement: The documentation''s diagrams are accessible in every rendering'
evidence: |
  both home pages; the four paths in `choose-your-path.md`. Each diagram in
  a chapter MUST be an inline `<svg>` inside a `<figure>` with
recommendation: 'Cover diagrams on home pages too, run the check over them, add a failing scenario.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: qa
    ref: 'specs/docs/spec.md (in a chapter or on a home page; scenarios); tasks.md 2.1, 4.2'
  - status: fixed
    by: accessibility
```

#### R-accessibility-2 No task checked the contrast of every diagram in every rendering

```yaml
severity: medium
confidence: high
location: 'tasks.md#4. Diagrams'
evidence: |
  - [ ] 4.2 Draw the workflow diagram in `introduction.md` and `workflow.md`, and the four-paths diagram in `choose-your-path.md`, both editions; verified by the diagram check, the Choose your path checks, and a Playwright screenshot of each in light and navy with text contrast ≥ 4.5:1
recommendation: 'Measure contrast for every diagram in every theme, and keep text off coral fills.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: lead
    ref: 'tasks.md 3.1 contrast_problems over every theme; spec contrast scenario; design.md ink on light tints'
  - status: fixed
    by: accessibility
```

#### R-accessibility-3 The plan did not say how the home pages get a dark mode

```yaml
severity: low
confidence: low
location: 'design.md#Decision: Diagrams are hand-written inline SVG with themed custom properties'
evidence: |
  `--dg-line`, `--dg-accent-*` — defined for `.light` and `.navy` in
  `mairistem.css`, for both schemes in the home pages' CSS, and for
recommendation: 'State the mechanism; if a switch ships, make it an accessible button.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: 'design.md and spec: home pages follow prefers-color-scheme, no switch of their own'
  - status: fixed
    by: accessibility
```

#### R-accessibility-4 The terminal card replaying an exchange had no rule for motion

```yaml
severity: low
confidence: low
location: 'design.md#Decision: The look is approved on a mockup before it touches the theme'
evidence: |
  faint grid, the promise, and a **terminal card** replaying a short
recommendation: 'Keep the card static, in the DOM from the start.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: 'design.md static figure with pre; spec: nothing moves on its own'
  - status: fixed
    by: accessibility
```

#### R-accessibility-5 A non-empty aria-label passes even if it does not give the diagram's content

```yaml
severity: low
confidence: low
location: 'design.md#Decision: Diagrams are hand-written inline SVG with themed custom properties'
evidence: |
  `<text>` in the page font, so it stays searchable, translatable and
  readable by screen readers through the label.
recommendation: 'Require the label to name every step or node in order.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: 'spec: aria-label names every step in reading order; design.md; tasks.md 3.2'
  - status: fixed
    by: accessibility
```

## code.review

Mode: isolated.
Summarized: 43 files — the change exceeds the material limit (1.2 MB for 160 KB); summaries written by nine summarizer sub-agents of about five files each, not one per file, to limit cost.

### accessibility

#### R-accessibility-6 The home pages' Copy buttons no longer copy and announce nothing

```yaml
severity: medium
confidence: high
location: 'docs/site/index.html:487'
evidence: |
        var text = button.parentNode.querySelector('pre').textContent.trim();
recommendation: 'Find the command with button.closest(''.codeblock''); announce the failure when it is missing. Same in fr.html.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: 'docs/site/index.html, docs/site/fr.html, site/build_home.py (script_of); test home_page_copy_buttons_find_their_command'
  - status: fixed
    by: accessibility
```

#### R-accessibility-7 The coral YOU labels in the workflow diagram are below 4.5:1

```yaml
severity: low
confidence: high
location: 'docs/en/src/workflow.md:11'
evidence: |
  style="fill:var(--dg-you)">YOU</text>
recommendation: 'Draw coral text with a text token (--dg-you-text, #c4361b on white) and add it to the checked text pairs; same in FR and on the home pages.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: '--dg-you-text in mairistem.css, docs.css and the home pages; text fills in docs/{en,fr}/src/workflow.md; pairs added to contrast_problems'
  - status: fixed
    by: accessibility
```

#### R-accessibility-8 The digit on the kept step badge is 3.65:1

```yaml
severity: low
confidence: high
location: 'docs/site/index.html:186'
evidence: |
  .n.kept { background: var(--dg-line); color: var(--dg-box); }
recommendation: 'Make the box-on-line pair reach 4.5:1 and add it to the checked text pairs.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: '--dg-line #567079 light, #a7c6d1 dark; pair (--dg-box, --dg-line) added to contrast_problems'
  - status: fixed
    by: accessibility
```

### newcomer

#### R-newcomer-1 The order check in toc_problems depends on problems from earlier parts

```yaml
severity: medium
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:519-526'
evidence: |
          if problems.is_empty()
              && listed
                  .iter()
                  .map(String::as_str)
                  .ne(expected.iter().copied())
          {
              problems.push(format!("\"{title}\" is out of order: {listed:?}"));
          }
recommendation: 'Track problems per part (`let before = problems.len()`) so each part''s order is checked.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'docs_checks.rs toc_problems: per-part `before` counter with a comment'
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 without_svg no longer removes SVG; it replaces each drawing with its words

```yaml
severity: medium
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:1212'
evidence: |
  fn without_svg(md: &str) -> String {
recommendation: 'Rename it for what it returns.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'docs_checks.rs: without_svg renamed with_diagrams_as_words'
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 No comment explains why the example plan is exempt from the everyday check

```yaml
severity: low
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:2637'
evidence: |
              for path in everyday.iter().filter(|p| *p != "guides/example-plan.md") {
recommendation: 'Name the exemption in a documented constant.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'docs_checks.rs: UNCAPPED_EVERYDAY constant with its reason'
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 diagram_problems' blank-line test repeats itself and nests the href scan

```yaml
severity: low
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:1327'
evidence: |
          if svg.contains("\n\n")
recommendation: 'Keep one blank-line test; move the href/src scan into a helper.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'docs_checks.rs: one blank-line test; points_outside helper'
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 The selector extraction in diagram_themes is a dense one-off chain

```yaml
severity: low
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:1372'
evidence: |
                  .rsplit(['}', ';', '{'])
recommendation: 'Move it into a named helper that normalizes whitespace once.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'docs_checks.rs: selector_before helper, whitespace normalized once'
  - status: fixed
    by: newcomer
```

#### R-newcomer-6 SUMMARY.md entry lines are parsed in two places

```yaml
severity: low
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:2584'
evidence: |
      fn summary_title(lang: Lang, path: &str) -> Option<String> {
recommendation: 'Extract one summary_entries helper used by both.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'docs_checks.rs: summary_entries used by title_problems and summary_title'
  - status: fixed
    by: newcomer
```
