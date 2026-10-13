# Review record: inline-form-validation

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

Disabled by configuration.

### accessibility

#### R-accessibility-1 Errors are conveyed by colour only, with no text message

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  is. No message text is added, to keep the form compact.
recommendation: >
  Show a text error message next to each invalid field (for example under it),
  saying what is wrong and how to fix it, and keep the red border only as a
  secondary cue (optionally with an icon). Screen reader users, colour-blind
  users (red vs grey border) and low-vision users cannot otherwise tell which
  field is wrong or why — WCAG 1.4.1 Use of Color and 3.3.1 Error
  Identification. Reconsider the rejected "message under each field"
  alternative.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 No programmatic invalid state or field–message association

```yaml
severity: high
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
recommendation: >
  Besides the CSS class, set `aria-invalid="true"` on the invalid input and
  link its error message with `aria-describedby`, so assistive technology
  announces the error when the field is focused — WCAG 4.1.2 Name, Role, Value
  and 1.3.1 Info and Relationships. A class name alone exposes nothing to a
  screen reader.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Removing the error summary leaves no feedback on submit

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Invalid fields turn red"
evidence: |
  `field--invalid`, a red border (`#e5484d`); the list of errors at the top of
  the page is removed, since the red border already shows where the problem
recommendation: >
  State what happens when the customer submits with invalid fields: keep an
  error summary (announced, with links to the fields) or move focus to the
  first invalid field, whose message is then read. Without it, keyboard and
  screen reader users who submit get no indication of failure — WCAG 3.3.1
  Error Identification and 2.4.3 Focus Order.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Acceptance scenarios check only the border colour

```yaml
severity: medium
confidence: high
location: "specs/checkout-validation/spec.md#Scenario: An invalid postcode"
evidence: |
  - **THEN** the field's border turns red
recommendation: >
  Make the scenarios require a text message associated with the field
  (`aria-describedby`) and `aria-invalid="true"`, and its removal when the
  field is corrected; then add matching checks to task 1.2. As written, the
  spec makes colour-only error indication the accepted behaviour — WCAG 1.4.1
  Use of Color, 3.3.1 Error Identification.
history:
  - status: open
    by: accessibility
```
