# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Highlighted option and combobox focus rely on styles the change does not include

```yaml
severity: medium
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
            className={index === active ? "option option--active" : "option"}
recommendation: >-
  DOM focus stays on the combobox while `aria-activedescendant` moves the
  highlight, so a sighted keyboard user sees which size Arrow keys reached
  only through the `option--active` class, and sees the closed control's
  focus only through the browser default outline. No stylesheet in the
  repository defines `option--active`, `option` or `size-picker`. Check
  that the stylesheet that will ship with this component gives
  `.option--active` a visible highlight and keeps a visible focus
  indicator on `[role="combobox"]`, each with at least 3:1 contrast
  (WCAG 2.4.7 Focus Visible, 1.4.11 Non-text Contrast).
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Option touch target size is not set by the change

```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:87-95"
evidence: |
          <li
            key={size}
            id={`${id}-option-${index}`}
            role="option"
recommendation: >-
  The proposal's motivation is that customers on phones mis-tap tiny size
  controls, yet nothing in the change sizes the options or the combobox.
  Check that the stylesheet gives each `.option` and the combobox a target
  of at least 24 by 24 CSS pixels, preferably larger for this use case
  (WCAG 2.5.8 Target Size (Minimum)).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
