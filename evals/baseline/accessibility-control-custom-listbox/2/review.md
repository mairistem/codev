# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Focus indicator of the combobox depends on a stylesheet outside the change

```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:70-73"
evidence: |
  <div
    ref={combo}
    role="combobox"
    tabIndex={0}
recommendation: >-
  The combobox is a focusable `div`; its focus indicator comes from the
  browser default unless a stylesheet removes it, and no stylesheet is in
  the diff. Check that `.size-picker [role="combobox"]` keeps a visible
  focus style with at least 3:1 contrast (WCAG 2.4.7 Focus Visible, 1.4.11
  Non-text Contrast) — keyboard users must see where focus is.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Highlighted option is shown only through a class whose style is not in the change

```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  With `aria-activedescendant`, DOM focus stays on the combobox, so the
  only visual cue of the highlighted option is `option--active`. Check
  that this class gives a visible, non-colour-only highlight with at least
  3:1 contrast (WCAG 2.4.7 Focus Visible, 1.4.11 Non-text Contrast), and
  that options are at least 24 by 24 CSS pixels for the phone users this
  change targets (2.5.8 Target Size (Minimum)).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
