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
  Render the delete action as a native `<button type="button">` (keeping the
  `icon-button` class): it is focusable, exposes the button role and fires on
  Enter and Space. A keyboard user pressing Tab never reaches the trash icon,
  and assistive technology hears no button, so the document cannot be deleted
  without a mouse — WCAG 2.1.1 Keyboard and 4.1.2 Name, Role, Value.
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
  button (or visually hidden text inside it). Its only content is `TrashIcon`,
  whose `<svg>` is `aria-hidden="true"` (src/components/icons.tsx), so a screen
  reader user reaching the control hears no name and cannot tell it deletes the
  document — WCAG 1.1.1 Non-text Content and 4.1.2 Name, Role, Value.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Owner's avatar image has no text alternative

```yaml
severity: medium
confidence: high
location: "src/components/Toolbar.tsx:12"
evidence: |
  <img className="avatar" src={avatarUrl} />
recommendation: >-
  The avatar identifies the document's owner, so give it an `alt` naming the
  owner (e.g. `alt={ownerName}`, which needs an owner name prop). With no
  `alt`, screen readers announce the image file name or URL instead of who
  owns the document — WCAG 1.1.1 Non-text Content. If the owner's name is
  already shown as text next to it, use `alt=""` instead.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Check focus visibility and target size of the 16px icon button

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13-14"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
recommendation: >-
  The `icon-button` stylesheet is not in the change. Once the control is a
  button, check that it shows a visible focus indicator (WCAG 2.4.7 Focus
  Visible) and that its target is at least 24 by 24 CSS pixels or spaced
  enough, since the icon itself is 16 by 16 (WCAG 2.5.8 Target Size (Minimum)).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
