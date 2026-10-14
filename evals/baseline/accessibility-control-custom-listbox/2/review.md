# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Highlighted option relies on a stylesheet the change does not include

```yaml
severity: medium
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  The keyboard interaction follows the APG select-only combobox (arrows, Home/End,
  Enter/Space, Escape, Tab, aria-activedescendant), but the only visual cue of the
  highlighted option is the `option--active` class, and no stylesheet in the
  repository defines it. Check that `.option--active` gives a visible indicator
  with at least 3:1 contrast (WCAG 2.4.7 Focus Visible, 1.4.11 Non-text Contrast),
  that the combobox keeps a visible focus outline, and that options are at least
  24 by 24 CSS pixels (2.5.8 Target Size, the proposal's mis-tap problem);
  otherwise sighted keyboard users cannot see which size Enter or Tab will choose.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
