# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### accessibility

#### R-accessibility-1 Field errors are conveyed by a red border only

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >-
  Keep the red border but add, under each invalid field, a text message saying
  what is wrong and how to fix it ("Enter a 5-digit postcode"), associated with
  the input through `aria-describedby`, and set `aria-invalid="true"` on the
  input while it is invalid. Today a screen reader user who tabs out of the
  postcode field with `ABC` hears nothing, and on returning to the field hears
  only its label: nothing tells them it is invalid nor why; a colour-blind user
  cannot tell the red border from the grey one; and with the top error list
  removed, there is no text left anywhere. WCAG 1.4.1 Use of Color, 3.3.1 Error
  Identification, 3.3.3 Error Suggestion, 4.1.2 Name, Role, Value. Update the
  spec's scenarios ("the field's border turns red", "the border returns to
  grey") so that they require the message and `aria-invalid`, and task 1.2 to
  test them.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Nothing says where focus goes when the form is submitted with errors

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  Each field validates on `blur`. An invalid field gets the class
recommendation: >-
  The plan removes the error list at the top of the page and only describes
  validation on `blur`; it does not say what happens on submit when a field is
  invalid or was never visited. If submit is simply blocked, a keyboard or
  screen reader user presses Enter on the submit button and nothing is
  announced. Specify that on a blocked submit every field is validated and
  focus moves to the first invalid field (whose message is then read through
  `aria-describedby`), and add a scenario for it. WCAG 3.3.1 Error
  Identification, 2.4.3 Focus Order. Confidence low: the current submit
  handling in `web/src/checkout/Form.tsx` may already do part of this.
history:
  - status: open
    by: accessibility
```

### skeptic

Disabled by configuration.
