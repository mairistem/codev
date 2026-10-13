# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

### accessibility

#### R-accessibility-1 Highlighted option relies on an unseen style

```yaml
severity: medium
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
            className={index === active ? "option option--active" : "option"}
recommendation: >
  Check that a stylesheet gives `.option--active` a highlight that keeps 3:1
  contrast against the other options (WCAG 2.4.7 Focus Visible, 1.4.11
  Non-text Contrast). DOM focus stays on the combobox while ArrowUp/ArrowDown
  move `aria-activedescendant`, so this class is the only cue a sighted
  keyboard user gets about which size Enter or Tab will pick. Also check that
  the combobox `div` keeps a visible focus outline. The diff and the
  repository contain no stylesheet, so this could not be verified.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
