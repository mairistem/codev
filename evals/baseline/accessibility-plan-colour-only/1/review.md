# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### accessibility

#### R-accessibility-1 Field errors are conveyed by colour only, with no text and no programmatic state

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >-
  Screen reader users and colour-blind users cannot tell which field is
  invalid or why: a screen reader user tabs out of the postcode field with
  `ABC`, the border turns red, and nothing is announced; on the field, only
  its label is read. This fails WCAG 1.4.1 Use of Color, 3.3.1 Error
  Identification and 4.1.2 Name, Role, Value, and the removal of the error
  list leaves no text alternative anywhere. Keep the red border, but add for
  each invalid field a visible text message next to it (say what is wrong and
  how to fix it, WCAG 3.3.3), associate it with `aria-describedby`, and set
  `aria-invalid="true"` while the field is invalid. Update the spec's
  requirement and scenarios ("the field's border turns red", "the border
  returns to grey") so they require the message and the invalid state, not
  just the colour, and add them to task 1.2's tests.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Nothing tells a keyboard or screen reader user where the errors are on submit

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  Each field validates on `blur`. An invalid field gets the class
recommendation: >-
  Validation only runs on blur, and the error list at the top is removed, but
  the plan does not say what happens when the form is submitted with invalid
  or never-visited fields. A keyboard user who presses Enter on the submit
  button with an empty required field they never focused gets no blur
  validation and no list: either nothing happens or fields turn red off
  screen, with no announcement and focus left on the button (WCAG 3.3.1
  Error Identification, 2.4.3 Focus Order, 4.1.3 Status Messages). Specify
  the submit path: validate every field on submit, move focus to the first
  invalid field (or to an error summary linking to each field), and add a
  scenario for it in the spec.
history:
  - status: open
    by: accessibility
```
