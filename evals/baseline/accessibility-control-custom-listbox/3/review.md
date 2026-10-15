# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Highlighted option's visible indicator depends on an unshipped style

```yaml
severity: medium
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  With aria-activedescendant, DOM focus stays on the combobox, so the class
  `option--active` is the only thing that shows a sighted keyboard user which
  size ArrowUp/ArrowDown, Home/End have reached before Enter, Space or Tab
  commits it. No stylesheet in the repository defines `.option--active` (nor
  `.size-picker`), so check that the shipped CSS gives it a visible indicator
  with at least 3:1 contrast against the unhighlighted options, and that the
  combobox keeps a visible focus outline (WCAG 2.4.7 Focus Visible,
  1.4.11 Non-text Contrast).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
