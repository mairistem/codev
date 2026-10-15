# Review record: add-toolbar

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Delete action is a clickable div, unreachable by keyboard

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
  The `div` has no role, no `tabindex` and no key handler: a keyboard user pressing Tab never lands on it,
  and Enter or Space cannot trigger it, so the only way to delete a document from its page — the purpose
  of the change — is a mouse click. A button is focusable, operable with Enter and Space, and exposes the
  role "button". WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value.
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
  Give the control a text name, for example `aria-label="Delete document"` on the button (or visually
  hidden text inside it). Its only content is `TrashIcon`, whose `<svg>` is `aria-hidden="true"`
  (src/components/icons.tsx:3), so the control has no text at all: a screen reader user reaching it hears
  nothing, or just "button" once it becomes a button, and cannot tell it deletes the document.
  WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value.
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
  Add an `alt` naming the owner (e.g. `alt={`${ownerName}'s avatar`}` — which needs the owner's name as a
  prop), or `alt=""` if the owner is named in visible text elsewhere. Without `alt`, a screen reader
  announces the image file name or URL from `avatarUrl` instead of who owns the document.
  WCAG 1.1.1 Non-text Content.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Delete target may be smaller than 24 by 24 CSS pixels

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13-14"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
recommendation: >-
  The icon inside is 16 by 16 (src/components/icons.tsx:3) and no stylesheet for `.icon-button` is in the
  change or the repository: check that the clickable area is at least 24 by 24 CSS pixels (padding on the
  button) and that the button gets a visible focus style. WCAG 2.5.8 Target Size (Minimum), 2.4.7 Focus Visible.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
