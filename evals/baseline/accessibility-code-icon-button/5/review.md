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
  Replace the div with a native `<button type="button" className="icon-button" onClick={onDelete}>`:
  it is in the tab order, reacts to Enter and Space, and exposes the button role.
  A keyboard user pressing Tab never lands on the delete action and cannot delete a
  document; screen readers announce no control. WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value.
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
  Give the control a text name, e.g. `aria-label="Delete document"` on the button
  (or visually hidden text). The only content is `TrashIcon`, whose svg is
  `aria-hidden="true"` (src/components/icons.tsx:3), so a screen reader user
  reaching the control hears nothing, or just "button" once it becomes one, and
  cannot tell it deletes the document. WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value.
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
  Add an `alt`: the owner's name (e.g. `alt={ownerName}`, which needs a prop)
  since the avatar identifies the document's owner, or `alt=""` if the owner is
  named in text elsewhere. Without it, screen readers announce the image file
  name or URL instead of who owns the document. WCAG 1.1.1 Non-text Content.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Icon button target size and focus style not verifiable

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13-14"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
recommendation: >-
  The icon is 16 by 16 (src/components/icons.tsx:3) and the `icon-button` style is
  not in the change: check that the control is at least 24 by 24 CSS pixels (or
  spaced enough) and has a visible focus indicator once it is a button.
  WCAG 2.5.8 Target Size (Minimum), 2.4.7 Focus Visible.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
