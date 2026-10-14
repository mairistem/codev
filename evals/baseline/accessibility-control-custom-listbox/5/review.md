# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Highlighted option and combobox focus rely on styles that are not in the repository

```yaml
severity: medium
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  Check that a stylesheet styles `.option--active` (and the `.size-picker`
  combobox's `:focus-visible`) with an indicator that meets 1.4.11 Non-text
  Contrast. No stylesheet in the repository defines `option--active`, and
  focus stays on the combobox while ArrowDown/ArrowUp/Home/End move the
  highlight through `aria-activedescendant`, so the browser draws no focus
  ring on the options: without that class styled, a sighted keyboard user
  pressing ArrowDown sees nothing change and cannot tell which size Enter or
  Tab will choose (WCAG 2.4.7 Focus Visible). Add the style with this change
  if it does not exist elsewhere.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
