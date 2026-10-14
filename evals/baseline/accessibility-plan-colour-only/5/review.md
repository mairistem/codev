# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### accessibility

#### R-accessibility-1 Field errors are conveyed by a red border only, with no text

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  An invalid field gets the class
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >-
  Errors by colour only (WCAG 1.4.1 Use of Color, 3.3.1 Error Identification,
  1.3.1 Info and Relationships, 4.1.2 Name, Role, Value). A screen reader user
  who tabs out of the postcode field with `ABC` hears nothing, and on returning
  to it hears only "Postcode, edit text": nothing says it is invalid or why. A
  colour-blind user cannot tell the red `#e5484d` border from the grey one, and
  no user learns what a valid postcode is, since the rule messages that
  `rules.ts` returns (today shown in the summary) are no longer displayed
  anywhere. Add a short text message under each invalid field (the rule's
  message), tie it to the input with `aria-describedby`, set
  `aria-invalid="true"` on the input, and keep the red border only as a
  supplement. Update the spec scenarios "An invalid postcode" and "A corrected
  field" — which today assert only "the field's border turns red" / "returns to
  grey" — and task 1.2 to require and test the message and `aria-invalid`.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Removing the error summary leaves a failed submit unannounced and without focus

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  the list of errors at the top of
  the page is removed
recommendation: >-
  Today the summary is the only announced feedback
  (`web/src/checkout/Form.tsx:22` renders `<div role="alert"
  className="error-summary">`). The plan removes it and says nothing of what
  happens on submit. A keyboard or screen reader user who presses Enter on
  "Pay" with a field they never left (validation runs on `blur` only) gets no
  announcement and focus stays on the button: the payment silently does not
  happen (WCAG 3.3.1 Error Identification, 4.1.3 Status Messages, 2.4.3 Focus
  Order). State in the design that submit still validates every field and
  either moves focus to the first invalid field (whose message is then read
  through `aria-describedby`, see R-accessibility-1) or keeps a summary with
  `role="alert"` linking to the fields; add a spec scenario and a task for it.
history:
  - status: open
    by: accessibility
```

### skeptic

Disabled by configuration.
