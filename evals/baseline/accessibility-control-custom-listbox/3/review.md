# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Focus and highlighted-option styles are not part of the change

```yaml
severity: medium
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
            className={index === active ? "option option--active" : "option"}
recommendation: >-
  The highlighted option is shown to sighted users only through the
  `option--active` class, and the combobox `div` (tabIndex 0) relies on the
  default focus ring; no stylesheet in the change or the repository defines
  `.size-picker`, `.option` or `.option--active`. Check that the product
  page's styles give `.option--active` a visible, non-colour-only indication
  with 3:1 contrast, and do not remove the combobox's outline without a
  replacement; otherwise a sighted keyboard user pressing ArrowDown cannot
  see which size Enter or Tab will choose (WCAG 2.4.7 Focus Visible,
  1.4.11 Non-text Contrast).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
