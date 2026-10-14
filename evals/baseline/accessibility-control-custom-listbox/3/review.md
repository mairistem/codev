# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Focus and highlighted-option styles are not in the change
```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  The repository holds no stylesheet for `size-picker`, `option` or
  `option--active`, so it cannot be verified from the change that the
  combobox shows a visible focus (WCAG 2.4.7 Focus Visible) or that the
  option highlighted with the arrow keys is visually distinct with enough
  contrast (WCAG 1.4.11 Non-text Contrast); nor that the button and options
  meet the target size the proposal's phone users need (WCAG 2.5.8 Target
  Size (Minimum)). Check the styles where they live: a focus ring on the
  `role="combobox"` element, a highlight on `.option--active` that does not
  rely on colour alone, and option rows of at least 24 by 24 CSS pixels. If
  the list scrolls, keep the active option scrolled into view (WCAG 2.4.11
  Focus Not Obscured (Minimum)).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
