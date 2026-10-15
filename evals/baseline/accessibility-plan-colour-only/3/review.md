# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### accessibility

#### R-accessibility-1 Invalid fields are identified by colour only, with no text message

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >
  A screen reader user who tabs out of the postcode field with `ABC` hears
  nothing: the only signal is a border colour change, which is not exposed to
  assistive technology, and colour-blind or low-vision users may not perceive
  red against grey either. With the top error list removed, no text anywhere
  says which field is wrong or why, so the checkout cannot be completed
  reliably (WCAG 1.4.1 Use of Color, 3.3.1 Error Identification, 3.3.3 Error
  Suggestion, 4.1.2 Name, Role, Value). Add a text message under each invalid
  field, linked with `aria-describedby`, set `aria-invalid="true"` on the
  input, and keep a non-colour cue (icon or text) next to the border. Update
  the spec scenarios in `specs/checkout-validation/spec.md` ("the field's
  border turns red" / "returns to grey") to require the message and
  `aria-invalid`, and the tests named in tasks 1.2 accordingly; the
  "Alternatives considered" entry rejected exactly this fix.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Removing the error summary leaves no feedback or focus target on submit

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The error list at the top of the form is removed.
recommendation: >
  A keyboard or screen reader user who presses Enter on the submit button with
  an invalid or never-visited required field gets no announcement and focus
  stays on the button: the plan validates only on `blur` and says nothing about
  what happens on submit, so the user cannot find which field blocks the order
  (WCAG 3.3.1 Error Identification, 2.4.3 Focus Order). Specify the submit
  behaviour: validate every field on submit and move focus to the first invalid
  field (whose message is announced via `aria-describedby`), or keep an error
  summary that takes focus and links to each field. Add a scenario for it in
  `specs/checkout-validation/spec.md`.
history:
  - status: open
    by: accessibility
```

### skeptic

Disabled by configuration.
