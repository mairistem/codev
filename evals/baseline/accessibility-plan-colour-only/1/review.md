# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### accessibility

#### R-accessibility-1 Field errors are signalled by a red border only

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >-
  A customer leaves the postcode field with `ABC`: a screen reader user hears
  nothing (no text, no `aria-invalid`), and a colour-blind user sees a border
  that differs from the grey one by hue only; neither learns which field is
  wrong nor why, so the checkout cannot be completed reliably (WCAG 2.2 1.4.1
  Use of Color, 3.3.1 Error Identification, 3.3.3 Error Suggestion, 4.1.2
  Name, Role, Value). Render the rule's message as text under each invalid
  field, tie it to the input with `aria-describedby`, set
  `aria-invalid="true"` on the input, and keep the red border as a secondary
  cue. Update the spec's scenarios ("the field's border turns red") and task
  1.2 to require and test the message text and its association rather than
  the border colour alone.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Removing the error summary leaves submit errors unannounced

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The error list at the top of the form is removed.
recommendation: >-
  Today `Form.tsx` renders the summary in a `role="alert"` region on submit,
  so screen reader users hear the errors. With it removed and nothing
  specified for submit, a keyboard or screen reader user who presses Pay with
  an untouched or invalid field hears no announcement and focus stays on the
  button, so they cannot tell why payment did not proceed (WCAG 2.2 3.3.1
  Error Identification, 4.1.3 Status Messages). Keep an error summary on
  submit (with links to the fields) or move focus to the first invalid field
  whose message is associated with it; add a spec scenario for submitting
  the form with an invalid field.
history:
  - status: open
    by: accessibility
```
