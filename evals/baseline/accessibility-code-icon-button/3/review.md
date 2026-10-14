# Review record: add-toolbar

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Delete action is a clickable div unreachable by keyboard

```yaml
severity: high
confidence: high
location: "src/components/Toolbar.tsx:13-15"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
  </div>
recommendation: >-
  Render the delete action as a native `<button type="button" className="icon-button" onClick={onDelete}>`.
  As written, the `div` has no role, no `tabindex` and no key handler: a keyboard user pressing Tab never
  lands on it, and Enter or Space cannot trigger it, so the change's only feature — deleting the document —
  cannot be done without a mouse; a screen reader does not announce it as a control either.
  WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value.
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
  Give the control a text name, e.g. `aria-label="Delete document"` on the button (or visually hidden text).
  Its only content is `TrashIcon`, whose `<svg>` is `aria-hidden="true"` (src/components/icons.tsx), so even
  once it is a button a screen reader user tabbing to it hears only "button", with nothing saying it deletes
  the document. WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Owner avatar image has no alt attribute

```yaml
severity: medium
confidence: high
location: "src/components/Toolbar.tsx:12"
evidence: |
  <img className="avatar" src={avatarUrl} />
recommendation: >-
  Add a text alternative naming the owner, e.g. `alt={`${ownerName}'s avatar`}` (the component would need the
  owner's name as a prop), or `alt=""` if the owner is named in text nearby. Without `alt`, screen readers
  fall back to announcing the image URL or "image", so screen reader users cannot learn whose document it is,
  which the proposal says the avatar conveys. WCAG 1.1.1 Non-text Content.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Delete icon target size and focus style not verifiable

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13-14"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
recommendation: >-
  The icon is 16 by 16 pixels and the `.icon-button` stylesheet is not part of the change. Check that the
  control's target is at least 24 by 24 CSS pixels (or spaced accordingly) and that it shows a visible focus
  indicator once it is a button. WCAG 2.5.8 Target Size (Minimum), 2.4.7 Focus Visible.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
