# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Focus indicator of the combobox cannot be checked from the change

```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:70-73"
evidence: |
      <div
        ref={combo}
        role="combobox"
        tabIndex={0}
recommendation: >-
  The focusable `div` relies on a stylesheet the change does not include. Check that `.size-picker [role="combobox"]:focus-visible` keeps a visible outline (no `outline: none` without replacement) with at least 3:1 contrast against its background (WCAG 2.4.7 Focus Visible, 1.4.11 Non-text Contrast); keyboard users otherwise cannot see where focus is.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Highlighted option is shown only through a class whose style is not in the change

```yaml
severity: low
confidence: low
location: "src/components/SizePicker.tsx:92"
evidence: |
            className={index === active ? "option option--active" : "option"}
recommendation: >-
  Focus stays on the combobox and the highlighted option is the only visual cue of the keyboard position (`aria-activedescendant`). Check that `.option--active` is styled with an indicator other than colour alone and with at least 3:1 contrast (WCAG 2.4.7 Focus Visible, 1.4.11 Non-text Contrast, 1.4.1 Use of Color), and that the highlighted option is scrolled into view when the list overflows (WCAG 2.4.11 Focus Not Obscured).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
