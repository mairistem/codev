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
  The div has no role, no tabindex and no key handler: a keyboard user pressing Tab never reaches it, and
  Enter or Space cannot trigger it, so a keyboard-only owner cannot delete a document — the change's only
  purpose. Screen readers do not expose it as a control either. WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value.
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
  Give the control a text name, e.g. `aria-label="Delete document"` on the button (or visually hidden text).
  Its only content is `TrashIcon`, whose `<svg>` is `aria-hidden="true"` (src/components/icons.tsx:3), so even
  once it is a `<button>` a screen reader announces just "button": the user cannot tell that it deletes the
  document. WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value.
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
  Add a text alternative naming the owner (e.g. `alt={`${ownerName}'s avatar`}`, which needs the owner's name as
  a prop), or `alt=""` if the owner is named elsewhere in the toolbar. Without `alt`, screen readers read the image
  URL or file name instead of who owns the document. WCAG 1.1.1 Non-text Content.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Focus style and target size of the icon button are not visible in the change

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13"
evidence: |
  <div className="icon-button" onClick={onDelete}>
recommendation: >-
  The `.icon-button` styles are not in the diff and the icon is 16 by 16. Check that the control is at least
  24 by 24 CSS pixels (or spaced enough) and keeps a visible focus indicator once it is focusable. WCAG 2.5.8
  Target Size (Minimum), 2.4.7 Focus Visible.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
