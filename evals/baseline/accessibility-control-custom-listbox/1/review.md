# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Focus and highlighted option rely on styles the repository does not contain

```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  Focus stays on the combobox while `aria-activedescendant` moves, so the
  browser draws no focus ring on the highlighted option: a sighted keyboard
  user sees which size ArrowDown/ArrowUp reached only through the
  `option--active` style. No stylesheet in the repository defines
  `.size-picker`, `.option` or `.option--active`. Check, where the styles
  live, that `option--active` gives a visible indication with at least 3:1
  contrast (WCAG 2.4.7 Focus Visible, 1.4.11 Non-text Contrast), that the
  combobox keeps a visible focus indicator, and, if the list can scroll,
  that the highlighted option is scrolled into view.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
