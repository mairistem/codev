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
recommendation: >-
  Render the delete action as a native `<button type="button" className="icon-button" onClick={onDelete}>`.
  As written the div has no role, no tabindex and no key handler: a keyboard user pressing Tab never
  lands on it and cannot press Enter or Space to delete the document, so the change's only task
  (deleting a document from its page) cannot be completed without a mouse, and a screen reader does
  not announce it as a button. WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value.
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
  Give the control a text name, e.g. `aria-label="Delete document"` on the button (or visually hidden
  text). Its only content is `TrashIcon`, whose `<svg>` is `aria-hidden="true"` (src/components/icons.tsx:3),
  so even once it is a button a screen reader announces just "button", and the user cannot tell that
  it deletes the document. WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value.
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
  Add an `alt`: the owner's name (e.g. `alt={ownerName}`, passed as a prop) if the avatar is meant to
  say who owns the document, or `alt=""` if the owner is named in text elsewhere. Without `alt`, a
  screen reader reads the image's URL or file name instead of the owner. WCAG 1.1.1 Non-text Content.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Delete target size and focus style not verifiable

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13-14"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
recommendation: >-
  The icon is 16x16 (src/components/icons.tsx:3) and the `icon-button` styles are not in the change.
  Check that the control is at least 24x24 CSS pixels or spaced accordingly, and that it shows a
  visible focus indicator once it is focusable. WCAG 2.5.8 Target Size (Minimum), 2.4.7 Focus Visible.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
