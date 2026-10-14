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
  Blocked: screen reader users and users who cannot perceive red (WCAG 1.4.1
  Use of Color, 3.3.1 Error Identification, 4.1.2 Name, Role, Value). A screen
  reader user who tabs out of the postcode field with `ABC` hears nothing — the
  border change is not announced and, with the error list removed, no text
  anywhere says the field is invalid or why; a colour-blind user sees a border
  that barely changes. Add a short text message under each invalid field (or
  on the same line, to keep it compact on mobile), associate it with the input
  through `aria-describedby`, set `aria-invalid="true"` on the input while it
  is invalid, and keep the red border as a complement. Update the spec
  scenarios of "Invalid fields are shown as the customer types", whose THEN
  clauses check only the colour ("the field's border turns red" / "returns to
  grey"), so they also require the message and its association, and task 1.2
  so its tests verify them.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Submitting with invalid or untouched fields gives no feedback once the error list is removed

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  Each field validates on `blur`. An invalid field gets the class
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed
recommendation: >-
  Keyboard and screen reader users (WCAG 3.3.1 Error Identification, 2.4.3
  Focus Order). Validation runs only on `blur`, and the error list at the top
  is removed: a customer who presses Enter to submit while a required field
  was never focused, or while a field is invalid lower in the form, gets no
  announcement and no focus move — the submit appears to do nothing. Decide
  what submit does: validate every field, then move focus to the first invalid
  field (or to a short error summary linking to the fields, the RGAA/WCAG
  pattern), and add a scenario for it in the spec.
history:
  - status: open
    by: accessibility
```

### skeptic

Disabled by configuration.
