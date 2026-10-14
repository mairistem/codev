# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### accessibility

#### R-accessibility-1 Field errors are conveyed by a red border only, with no text
```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  Each field validates on `blur`. An invalid field gets the class
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >-
  A screen reader user who tabs out of the postcode field with `ABC` hears
  nothing — the class only changes a border, so the field is not exposed as
  invalid and no reason is given; a colour-blind or low-vision user cannot tell
  the red border from the grey one. Neither can learn what is wrong nor how to
  fix it (WCAG 1.4.1 Use of Color, 3.3.1 Error Identification, 3.3.3 Error
  Suggestion, 4.1.2 Name, Role, Value). Add a text message next to each
  invalid field, associated with it through `aria-describedby`, set
  `aria-invalid="true"` on the input, and keep the red border only as a
  complement. Update the spec scenarios ("the field's border turns red" /
  "returns to grey") to require the message and its removal, and drop the
  rejected alternative's rationale from the design.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Submitting an invalid form no longer tells the user anything
```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The error list at the top of the form is removed.
recommendation: >-
  With the top error list removed and no submit behaviour specified, a keyboard
  or screen reader user who presses Submit with a field still invalid (or never
  visited, so never blurred) gets no announcement and focus stays on the
  button: they cannot find which field blocks the order (WCAG 3.3.1 Error
  Identification, 2.4.3 Focus Order). Specify what happens on submit: validate
  every field, then move focus to the first invalid field (or to an error
  summary with links to the fields), and add a scenario for it in
  `specs/checkout-validation/spec.md`.
history:
  - status: open
    by: accessibility
```
