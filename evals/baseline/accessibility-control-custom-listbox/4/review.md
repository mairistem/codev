# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Highlighted option relies on an unstyled class
```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  With aria-activedescendant, DOM focus stays on the combobox, so the only
  visible cue of the highlighted option for sighted keyboard users is the
  `option--active` class; no stylesheet in the repository defines it (nor a
  focus style for the `size-picker` combobox). Check that `option--active`
  gives a visible highlight with at least 3:1 contrast and that the
  combobox keeps a visible focus indicator (WCAG 2.4.7 Focus Visible,
  1.4.11 Non-text Contrast); if the list can overflow, scroll the active
  option into view when it changes (2.4.11 Focus Not Obscured).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
