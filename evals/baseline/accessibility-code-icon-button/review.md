# Review record: add-toolbar

<!-- codev-review: 1 -->

## code.review

### accessibility

#### R-accessibility-1 Delete control is a clickable div unreachable by keyboard

```yaml
severity: high
confidence: high
location: "src/components/Toolbar.tsx:13-15"
evidence: |
  <div className="icon-button" onClick={onDelete}>
recommendation: >-
  Render the delete control as a native `<button type="button" className="icon-button" onClick={onDelete}>`.
  The `div` has no role, no `tabindex` and no key handler: a keyboard user
  pressing Tab never lands on it, and Enter or Space cannot trigger it, so
  the change's one purpose — deleting a document from its page — cannot be
  completed without a mouse. Screen readers also announce it as plain text,
  not as a button. WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Icon-only delete control has no accessible name

```yaml
severity: high
confidence: high
location: "src/components/Toolbar.tsx:13-15"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
  </div>
recommendation: >-
  Give the control a text name, e.g. `aria-label="Delete document"` on the
  button (and `aria-hidden="true"` on the decorative `TrashIcon` SVG), or
  visually hidden text inside it. As written, the only content is an icon:
  a screen reader user reaching the control hears nothing that says it
  deletes the document, so they cannot identify the action. Making it a
  `<button>` (R-accessibility-1) does not supply a name by itself.
  WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Owner avatar image has no alt text

```yaml
severity: medium
confidence: high
location: "src/components/Toolbar.tsx:12"
evidence: |
  <img className="avatar" src={avatarUrl} />
recommendation: >-
  Add an `alt` naming the owner, e.g. `alt={`${ownerName}'s avatar`}` or
  `alt={ownerName}` (the component needs the owner's name as a prop). The
  avatar conveys who owns the document; without `alt`, screen readers read
  the image URL or file name instead, so screen reader users do not learn
  who the owner is. If the owner is shown in text elsewhere, use `alt=""`
  to mark it decorative. WCAG 1.1.1 Non-text Content.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Focus style and target size of the icon button not visible in the change

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13"
evidence: |
  <div className="icon-button" onClick={onDelete}>
recommendation: >-
  The `.icon-button` stylesheet is not part of the change. Once the control
  is a focusable button, check that it shows a visible focus indicator
  (no `outline: none` without replacement, WCAG 2.4.7 Focus Visible) and
  that its target is at least 24 by 24 CSS pixels or spaced accordingly
  (WCAG 2.5.8 Target Size (Minimum)).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
