# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

### accessibility

#### R-accessibility-1 Field errors are conveyed by colour only

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >
  Keep the red border as a supplement only: render a text message next to
  each invalid field (e.g. "Enter a valid postcode"), tie it to the input with
  `aria-describedby`, and set `aria-invalid="true"` on the input. Screen reader
  users and colour-blind users cannot otherwise tell a field is invalid nor
  why (WCAG 1.4.1 Use of Color, 3.3.1 Error Identification, 3.3.3 Error
  Suggestion, 4.1.2 Name, Role, Value).
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Removing the error summary leaves no announced feedback on submit

```yaml
severity: high
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The error list at the top of the form is removed.
recommendation: >
  Specify what happens when the customer submits with invalid fields: either
  keep an error summary (focused on submit, each entry linking to its field)
  or move focus to the first invalid field whose error text is announced.
  Blur-only validation with no message and no summary leaves keyboard and
  screen reader users unable to find or understand the blocking error (WCAG
  3.3.1 Error Identification, 2.4.3 Focus Order, 4.1.3 Status Messages).
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Spec scenarios only verify the border colour

```yaml
severity: medium
confidence: high
location: "specs/checkout-validation/spec.md#Scenario: An invalid postcode"
evidence: |
  - **THEN** the field's border turns red
recommendation: >
  Add THEN clauses that require a text error message next to the field,
  programmatically associated with it (`aria-describedby`) and `aria-invalid`
  set, and its removal in "A corrected field"; tasks 1.2 tests would then
  check what assistive technology receives, not only a CSS class (WCAG 3.3.1,
  1.3.1 Info and Relationships).
history:
  - status: open
    by: accessibility
```

### skeptic

Disabled by configuration.
