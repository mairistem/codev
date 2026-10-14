# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### accessibility

#### R-accessibility-1 Field errors are conveyed by a red border only, with no text

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >-
  Screen reader users and colour-blind users cannot tell that a field is
  invalid nor why: a customer who tabs out of the postcode field with `ABC`
  hears nothing, and the only signal left once the top error list is removed is
  a border colour (WCAG 1.4.1 Use of Color, 3.3.1 Error Identification, 4.1.2
  Name, Role, Value). Show a short text message under each invalid field (or
  next to it), link it with `aria-describedby`, set `aria-invalid="true"` on
  the input, and do not rely on colour alone (add an icon or the text). Update
  the spec scenarios in `specs/checkout-validation/spec.md` ("An invalid
  postcode" / "A corrected field"), which today only check that the border
  turns red and back to grey, so the tests in task 1.2 require the message and
  its association, and drop the "invalid fields are shown in red" wording from
  the proposal's What Changes.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Nothing tells the customer about errors on submit once the top error list is removed

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  the list of errors at the top of
  the page is removed
recommendation: >-
  The design describes validation on `blur` only and says nothing about what
  happens when the form is submitted with invalid or untouched fields. With the
  error summary gone, a keyboard or screen reader user who presses Enter on the
  submit button gets no announcement and no focus change, and an invalid field
  may be off-screen (WCAG 3.3.1 Error Identification, 2.4.3 Focus Order, 4.1.3
  Status Messages). State the submit behaviour in the design: validate every
  field on submit and move focus to the first invalid field, or keep an error
  summary that receives focus and links to each field.
history:
  - status: open
    by: accessibility
```
