# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Highlighted option and focused combobox have no style in the change

```yaml
severity: medium
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  Check that a stylesheet gives `.option--active` a visible highlight and keeps a
  visible focus indicator on `.size-picker [role="combobox"]` (WCAG 2.4.7 Focus Visible,
  1.4.11 Non-text Contrast). Focus stays on the combobox while the arrows move the
  highlight through `aria-activedescendant`, so this class is the only cue a sighted
  keyboard user has of which size Enter, Space or Tab will choose; no stylesheet in
  the repository defines it. If the list can be taller than its container, also scroll
  the active option into view when it changes (2.4.11 Focus Not Obscured).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
