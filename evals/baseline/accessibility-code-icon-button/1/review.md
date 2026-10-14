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
  `icon-button` class): it is focusable, exposes the button role and fires on
  Enter and Space. Today a keyboard user pressing Tab never lands on the
  control, and a screen reader user hears no button, so neither can delete a
  document — the one task the proposal adds. WCAG 2.1.1 Keyboard, 4.1.2 Name,
  Role, Value. Check that the button keeps a visible focus style (2.4.7).
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
recommendation: >-
  Give the control a text name, for example `aria-label="Delete document"` on
  the button (or visually hidden text inside it). Its only content is
  `TrashIcon`, whose `<svg>` is `aria-hidden="true"` (src/components/icons.tsx),
  so even once it is a button a screen reader announces just "button" and the
  user cannot tell it deletes the document. WCAG 1.1.1 Non-text Content,
  4.1.2 Name, Role, Value.
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
  Add an `alt`. The component's comment says the avatar shows the document's
  owner, so it conveys meaning: pass the owner's name in the props and use it
  (`alt={ownerName}`); if the owner's name is already shown as text next to
  it, use `alt=""` instead. Without `alt`, screen readers announce the image
  file name or URL, and screen reader users never learn who owns the
  document. WCAG 1.1.1 Non-text Content.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Delete target size and focus style not visible in the change

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13-14"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
recommendation: >-
  The icon is 16 by 16 and the `icon-button` stylesheet is not in the change:
  check that the delete control is at least 24 by 24 CSS pixels (or spaced
  enough), that its icon meets the 3:1 non-text contrast, and that it shows a
  visible focus indicator. WCAG 2.5.8 Target Size (Minimum), 1.4.11 Non-text
  Contrast, 2.4.7 Focus Visible.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
