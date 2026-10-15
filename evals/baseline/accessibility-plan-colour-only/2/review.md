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
recommendation: >
  A screen reader user who leaves the postcode field with `ABC` hears nothing:
  the border colour is not exposed to assistive technology, and the only text
  that named the error ("Enter a 5-digit postcode, such as 75011.", from
  rules.ts) is the summary list the design removes. A colour-blind user cannot
  tell the red border from the grey one. Neither can learn which field is
  wrong or how to fix it, so the checkout cannot be completed (WCAG 1.4.1 Use
  of Color, 3.3.1 Error Identification, 3.3.3 Error Suggestion, 4.1.2 Name,
  Role, Value). Show the message rules.ts already returns as text under each
  invalid field, tie it to the input with `aria-describedby`, and set
  `aria-invalid="true"` on the input while it is invalid; keep the red border
  as an addition. Update the spec's scenarios ("THEN the field's border turns
  red") to require the message and its association, and task 1.2 to test them.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 A failed submit is no longer announced and focus is not moved

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  the list of errors at the top of
  the page is removed
recommendation: >
  Today the summary is rendered with `role="alert"` (Form.tsx:22), so pressing
  Pay with invalid fields is announced. The design removes it and says nothing
  of what happens on submit: a keyboard or screen reader user who never left a
  field, or who skipped past one, presses Pay and nothing happens and nothing
  is announced, with focus left on the button (WCAG 3.3.1 Error
  Identification, 4.1.3 Status Messages, 2.4.3 Focus Order). State the submit
  behaviour in the design and the spec: on a failed submit, validate every
  field, show each inline message, and move focus to the first invalid field
  (or keep an error summary with links to the fields), with a scenario for it.
history:
  - status: open
    by: accessibility
```
