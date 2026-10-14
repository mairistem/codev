# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### accessibility

#### R-accessibility-1 Invalid fields are identified by colour only, with no text and no programmatic state

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >-
  Identify each error in text next to its field (WCAG 1.4.1 Use of Color,
  3.3.1 Error Identification, 4.1.2 Name, Role, Value): render a message
  element under the invalid field, link it with `aria-describedby`, and set
  `aria-invalid="true"` on the input while it is invalid, removing both when
  it is corrected. Scenario that fails: a screen reader user tabs out of the
  postcode field with `ABC`; the only change is the border colour, nothing is
  announced, and with the top error list removed they have no way to learn
  that the field is invalid or why; a colour-blind user cannot tell the red
  border from the grey one either. The spec encodes the same gap — its
  scenarios only require "the field's border turns red" and "the border
  returns to grey" (specs/checkout-validation/spec.md) — so add THEN clauses
  for the message text and `aria-invalid`, and the matching checks to task
  1.2; the proposal's "invalid fields are shown in red" should follow.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Submitting with errors gives no summary and no focus move once the top error list is removed

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  Each field validates on `blur`. An invalid field gets the class
recommendation: >-
  State what happens on submit when fields are invalid (WCAG 3.3.1 Error
  Identification, 2.4.3 Focus Order): validate every field on submit, not
  only the ones blurred, and move focus to the first invalid field — or keep
  an error summary that takes focus and links to each field. Without it, a
  keyboard or screen reader user who never focused a required field, or who
  presses Enter from inside a field, submits, and the plan does not say
  whether anything is announced or where focus goes. Confidence is low
  because the design does not describe the submit path at all.
history:
  - status: open
    by: accessibility
```
