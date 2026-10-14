# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Highlighted option relies on a class with no style in the change

```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  With aria-activedescendant, DOM focus stays on the combobox, so the only
  visual cue of which option ArrowUp/ArrowDown/Home/End highlights is the
  `option--active` class. No stylesheet in the repository defines it, so
  check that the project's CSS gives `.option--active` a visible indicator
  meeting 3:1 non-text contrast (WCAG 2.4.7 Focus Visible, 1.4.11 Non-text
  Contrast), and that options are at least 24 by 24 CSS pixels for the
  phone users this change targets (2.5.8 Target Size (Minimum)).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
