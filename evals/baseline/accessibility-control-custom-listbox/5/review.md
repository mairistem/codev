# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Highlighted option is marked only by a class with no style in the repository

```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  The combobox keeps DOM focus and points to the highlighted option with
  aria-activedescendant, so the browser draws no focus ring on the option:
  sighted keyboard users see which size Arrow keys, Enter and Tab will choose
  only through the `option--active` style. No stylesheet defining
  `option--active` (or `.size-picker`) exists in the repository. Check that a
  stylesheet gives `.option--active` a visible indicator with at least 3:1
  contrast against adjacent colours (WCAG 2.4.7 Focus Visible, 1.4.11
  Non-text Contrast), and that options on phones are at least 24 by 24 CSS
  pixels (2.5.8 Target Size (Minimum)), the mis-tap problem the proposal
  addresses.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
