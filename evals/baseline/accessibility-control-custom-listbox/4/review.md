# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Visible focus and highlighted option rely on styles the change does not ship

```yaml
severity: medium
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >-
  The combobox is a `div` with `tabIndex={0}` and focus never leaves it while
  the list is open, so the highlighted option is shown to sighted keyboard
  users only through the `option--active` class. No stylesheet in the change
  or the repository defines `.size-picker` or `.option--active`. Check that the
  stylesheet loaded on the product page gives the combobox a visible focus
  indicator (WCAG 2.4.7 Focus Visible) and the active option a highlight that
  meets 3:1 against its neighbours (WCAG 1.4.11 Non-text Contrast); without
  them, a keyboard user pressing ArrowDown cannot see which size Enter will pick.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
