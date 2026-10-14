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
  As a `div` it has no `tabindex`, no role and no key handler: a keyboard user pressing Tab never
  lands on it, and Enter or Space cannot trigger it, so they cannot delete a document — the very
  task the change adds (WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value). Screen reader users do not
  hear it announced as a button either. The native element gives focus, role and Enter/Space
  activation together; check that the `icon-button` style keeps a visible focus indicator
  (WCAG 2.4.7 Focus Visible).
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
  Give the control a text name, e.g. `aria-label="Delete document"` on the button (or visually
  hidden text inside it). Its only content is `TrashIcon`, whose `<svg>` is `aria-hidden="true"`
  (src/components/icons.tsx:3), so even once it becomes a `<button>` a screen reader user
  reaching it hears only "button", with nothing saying it deletes the document — they cannot
  identify the action before triggering a destructive operation (WCAG 1.1.1 Non-text Content,
  4.1.2 Name, Role, Value).
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
  Add a text alternative. The proposal says the toolbar "shows the owner's avatar", so the image
  conveys who owns the document: pass the owner's name in and render
  `alt={`${ownerName}'s avatar`}` (or `alt={ownerName}`). If the owner's name is already shown as
  text next to it, use `alt=""` instead. Without any `alt`, screen readers announce the image
  file name or URL from `avatarUrl`, or "image", and the owner is never conveyed
  (WCAG 1.1.1 Non-text Content).
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Delete target size and focus style not verifiable in the change

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13"
evidence: |
  <div className="icon-button" onClick={onDelete}>
recommendation: >-
  The `icon-button` class is styled outside the change (no stylesheet in the repository diff),
  and the only visible size is the 16×16 `TrashIcon`. Check that the control's hit area is at
  least 24×24 CSS pixels or has enough spacing (WCAG 2.5.8 Target Size (Minimum)), that the icon
  stroke meets 3:1 against the toolbar background (WCAG 1.4.11 Non-text Contrast), and that it
  shows a visible focus once it is a button (WCAG 2.4.7 Focus Visible).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
