# Review record: add-toolbar

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Delete action is a clickable div, unreachable with the keyboard

```yaml
severity: high
confidence: high
location: "src/components/Toolbar.tsx:13-15"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
  </div>
recommendation: >-
  Render the delete action as a native `<button type="button">` (keeping the
  `icon-button` class) instead of a `<div>` with `onClick`. A keyboard user
  pressing Tab never lands on the div (no `tabindex`), and Enter or Space
  does nothing (no key handler), so a keyboard-only user cannot delete a
  document — the one task this change adds. A screen reader announces no
  role, so the action is not exposed as a control either. WCAG 2.1.1
  Keyboard, 4.1.2 Name, Role, Value. Once it is a button, check that the
  `.icon-button` style keeps a visible focus indicator (2.4.7).
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Icon-only delete control has no accessible name

```yaml
severity: high
confidence: high
location: "src/components/Toolbar.tsx:13-14"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
recommendation: >-
  Give the control a text name, e.g. `aria-label="Delete document"` on the
  button (or visually hidden text), and mark the icon decorative
  (`aria-hidden="true"` on the SVG). The only content is `<TrashIcon />`, so a
  screen reader user reaching the control hears no name, or "button" alone,
  and cannot tell it deletes the document. WCAG 1.1.1 Non-text Content,
  4.1.2 Name, Role, Value.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Owner avatar has no alt attribute

```yaml
severity: medium
confidence: high
location: "src/components/Toolbar.tsx:12"
evidence: |
  <img className="avatar" src={avatarUrl} />
recommendation: >-
  Add an `alt`: the owner's name (e.g. `alt={ownerName}`, passing the name as
  a prop) since the avatar identifies the document's owner, or `alt=""` if the
  owner's name is shown as text elsewhere. Without `alt`, screen readers
  announce the image URL or file name, which is noise and does not convey who
  owns the document. WCAG 1.1.1 Non-text Content.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
