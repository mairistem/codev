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
  WCAG 1.4.1 Use of Color, 3.3.1 Error Identification, 4.1.2 Name, Role, Value.
  A screen reader user who tabs out of the postcode field with `ABC` hears
  nothing: no text, no `aria-invalid`, no message, so they cannot tell which
  field is wrong and cannot complete checkout; a colour-blind user cannot tell
  the red border from the grey one. Add a text message under each invalid
  field, associated with it through `aria-describedby`, set
  `aria-invalid="true"` on the input, and keep the red border only as a
  supplement (an icon helps too). Update the spec's scenarios ("the field's
  border turns red" / "returns to grey") to require the message and its
  association, and task 1.2 to verify them.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Removing the error summary leaves submit with no announced errors or focus target

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The error list at the top of the form is removed.
recommendation: >-
  WCAG 3.3.1 Error Identification, 2.4.3 Focus Order, 4.1.3 Status Messages.
  A keyboard or screen reader user who submits without having left every
  field (so blur validation never ran on them) or who missed the inline
  state gets no list, no announcement and no focus move: pressing Enter on
  submit does nothing they can perceive. The plan says nothing of submit
  behaviour once the summary is gone. Specify that on submit with invalid
  fields, focus moves to the first invalid field (or to a kept, linked error
  summary), and its message is announced; add a scenario for it in the spec.
history:
  - status: open
    by: accessibility
```

### skeptic

Disabled by configuration.
