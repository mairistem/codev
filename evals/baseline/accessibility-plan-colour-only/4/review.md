# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### accessibility

#### R-accessibility-1 Field errors are conveyed by colour alone, with no text and no programmatic state

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >-
  Fails WCAG 1.4.1 Use of Color, 3.3.1 Error Identification and 4.1.2 Name,
  Role, Value. A screen reader user who tabs out of the postcode field with
  `ABC` hears nothing: the border change is not announced and the field is
  not exposed as invalid, so they reach submit without knowing what is wrong;
  a colour-blind user cannot tell a red border from the grey one. Add a text
  message under each invalid field, associated with `aria-describedby`, set
  `aria-invalid="true"` on the input, and keep the red border as a secondary
  cue (optionally with an icon). The rejected alternative ("a message under
  each field") is the one the criterion requires. Carry it into the spec —
  the scenarios "An invalid postcode" and "A corrected field" only assert
  the border colour ("the field's border turns red", "the border returns to
  grey") and should assert the message text and its association — and into
  task 1.2.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Submitting with invalid fields gives no feedback once the error list is removed

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The error list at the top of the form is removed.
recommendation: >-
  The plan says nothing about what happens on submit when fields are still
  invalid (a field never visited, or the customer pressing Enter in the
  form). With the top list gone and no message per field, a keyboard or
  screen reader user presses submit and nothing is announced and focus does
  not move — WCAG 3.3.1 Error Identification, 2.4.3 Focus Order. Add a
  decision and a scenario: on submit with errors, validate every field, show
  each field's message, and move focus to the first invalid field (or keep
  an error summary with links to the fields, announced on submit).
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Contrast of the red and grey borders against their background is not stated

```yaml
severity: low
confidence: low
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
recommendation: >-
  `#e5484d` is about 3.9:1 on white, but the form's background and the grey
  border colour are not in the change. Check WCAG 1.4.11 Non-text Contrast
  (3:1) for both borders against the actual field background once the
  stylesheet exists.
history:
  - status: open
    by: accessibility
```
