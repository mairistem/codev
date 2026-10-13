# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

Disabled by configuration.

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
recommendation: >-
  Fails WCAG 1.4.1 Use of Color, 3.3.1 Error Identification and 1.3.1 Info and
  Relationships. A screen reader user who leaves the postcode field with `ABC`
  hears nothing: the field is not marked `aria-invalid` and no text says what
  is wrong; a colour-blind or low-vision user cannot tell the red border from
  the grey one, and nobody learns what the rule is (the current text messages
  from `rules.ts` are dropped). Render the rule's message as text under each
  invalid field, associate it with `aria-describedby`, set `aria-invalid="true"`
  on the input, and rewrite the spec scenarios "An invalid postcode" and "A
  corrected field" (which only check the border colour) to require the message
  and its association; the spec's Purpose already promises "errors next to the
  fields".
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Removing the error summary leaves a failed submit unannounced

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The error list at the top of the form is removed.
recommendation: >-
  Today `Form.tsx` renders the summary in a `role="alert"` region, so a failed
  submit is announced. With it removed and validation only on `blur`, a
  keyboard or screen reader user who presses Pay with an untouched or invalid
  field hears nothing and focus stays on the button: the submit silently does
  nothing (WCAG 3.3.1 Error Identification, 4.1.3 Status Messages). On a failed
  submit, either keep a summary in a live region linking to the fields, or move
  focus to the first invalid field whose message is associated by
  `aria-describedby`; add a scenario for submitting with invalid fields.
history:
  - status: open
    by: accessibility
```
