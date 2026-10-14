# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### accessibility

#### R-accessibility-1 Field errors are conveyed by colour only, with no text message
```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >-
  Keep a text error message for each invalid field, rendered next to the field
  and associated with it through `aria-describedby`, and set
  `aria-invalid="true"` on the input (WCAG 1.4.1 Use of Color, 3.3.1 Error
  Identification, 4.1.2 Name, Role, Value). A screen reader user who tabs out
  of the postcode field with `ABC` hears nothing, and a colour-blind user cannot
  tell the red border from the grey one; neither learns which field is wrong or
  why (the existing messages from `rules.ts`, e.g. `postcodeError`, already give
  the text). Update the spec scenarios "An invalid postcode" and "A corrected
  field" to require the message text and its association, not only the border
  colour, and task 1.2 accordingly.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Removing the error summary leaves a failed submit silent
```yaml
severity: high
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The error list at the top of the form is removed.
recommendation: >-
  Today `web/src/checkout/Form.tsx:22` renders the errors in a
  `<div role="alert" className="error-summary">`, which a screen reader
  announces on submit. The plan removes it and says nothing about what happens
  when the customer presses Pay with invalid or untouched fields: a keyboard or
  screen reader user who presses Pay gets no announcement and no focus change,
  so the payment simply does not happen without explanation (WCAG 3.3.1 Error
  Identification, 4.1.3 Status Messages). Specify the submit behaviour: either
  keep the summary (with `role="alert"` and links to the fields) or move focus
  to the first invalid field whose message is associated with it; add a
  scenario for submitting with an invalid field.
history:
  - status: open
    by: accessibility
```
