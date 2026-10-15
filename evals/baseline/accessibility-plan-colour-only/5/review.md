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
  A screen reader user who tabs out of the postcode field with `ABC` hears
  nothing, and on returning to it hears only its label: the red border is not
  exposed to assistive technology, and with the top error list removed there is
  no text anywhere saying the field is invalid or why; colour-blind and
  low-vision users get only a hue change. This fails WCAG 1.4.1 Use of Color,
  3.3.1 Error Identification and 3.3.3 Error Suggestion. Add a text message next
  to each invalid field, associate it with `aria-describedby`, set
  `aria-invalid="true"` on the field (4.1.2 Name, Role, Value), and do not rely
  on colour alone (an icon or the message itself). Update the spec's
  requirement and scenarios ("the field's border turns red", "the border
  returns to grey") to require the message and its association, and the
  proposal's "invalid fields are shown in red" and task 1.2 to match.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Removing the error list leaves submit with no error identification or focus move

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The error list at the top of the form is removed.
recommendation: >-
  Validation runs only on `blur`, so a keyboard or screen reader user who
  submits without visiting a required field, or who submits right after
  typing in the last field, gets no error on submit: the plan defines no submit
  behaviour, no summary remains and focus is not moved, so the user presses
  Enter on the submit button and nothing is announced (WCAG 3.3.1 Error
  Identification, 2.4.3 Focus Order, 4.1.3 Status Messages). Specify that
  submit validates every field and moves focus to the first invalid field (or
  to an error summary linking to the fields), and add a scenario for it in
  `specs/checkout-validation/spec.md`.
history:
  - status: open
    by: accessibility
```
