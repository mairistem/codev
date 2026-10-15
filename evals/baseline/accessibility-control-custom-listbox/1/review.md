# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Highlighted option relies on a stylesheet the change does not include

```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  The highlighted option is shown to sighted keyboard users only through the
  `option--active` class, while DOM focus stays on the combobox; no stylesheet
  in the repository defines `.option--active`, `.option` or `.size-picker`.
  Check that the product page's stylesheet gives `option--active` a visible
  indicator with at least 3:1 contrast against the other options, and that the
  combobox keeps a visible focus outline (WCAG 2.4.7 Focus Visible, 1.4.11
  Non-text Contrast), and that options are at least 24 by 24 CSS pixels for
  touch (2.5.8 Target Size). Without it, a keyboard user pressing ArrowDown
  cannot see which size Enter or Tab will choose.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
