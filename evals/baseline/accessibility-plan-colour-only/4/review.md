# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### accessibility

#### R-accessibility-1 Field errors are conveyed by a red border only, with no text message

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
  is. No message text is added, to keep the form compact.
recommendation: >-
  Show the rule's message (rules.ts already returns "Enter a 5-digit postcode,
  such as 75011.") in text under each invalid field, associate it with
  `aria-describedby`, and set `aria-invalid="true"` on the input — WCAG 1.4.1
  Use of Color, 3.3.1 Error Identification, 4.1.2 Name, Role, Value. Today a
  screen reader user who leaves the postcode field with `ABC` hears nothing
  (only a CSS class changes), and a colour-blind user sees no difference from
  the grey border; nobody learns what the expected format is. Update the spec
  scenario "An invalid postcode" (THEN only "the field's border turns red")
  and task 1.2 so they require the message and its association, not only the
  border colour.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Submitting an invalid form is no longer announced and focus is not moved

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - The error list at the top of the form is removed.
recommendation: >-
  The removed summary is the form's only `role="alert"` (Form.tsx:22), so after
  the change a keyboard or screen reader user who presses Pay with an invalid
  or never-visited field gets no announcement and no focus change — the
  submit silently does nothing (WCAG 3.3.1 Error Identification, 2.4.3 Focus
  Order). State in the design what happens on submit: validate every field,
  then move focus to the first invalid field (or keep a focusable error
  summary with links to the fields), and add a scenario for it in the spec.
history:
  - status: open
    by: accessibility
```

### skeptic

Disabled by configuration.
