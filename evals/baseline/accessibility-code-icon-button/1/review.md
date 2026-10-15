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
  A keyboard user pressing Tab never lands on the div (no tabindex), and Enter or Space
  cannot trigger it (no key handler, no role), so the change's one feature — deleting a
  document — cannot be done without a mouse (WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value).
  The button gives focus, Enter/Space activation and the button role at once.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Icon-only delete control has no accessible name

```yaml
severity: high
confidence: high
location: "src/components/Toolbar.tsx:14"
evidence: |
  <TrashIcon />
recommendation: >-
  Give the control a name, e.g. `aria-label="Delete document"` on the button (or visually
  hidden text). Its only content is `TrashIcon`, whose `<svg ... aria-hidden="true">`
  (src/components/icons.tsx:3) is removed from the accessibility tree, so a screen reader
  user reaching the control hears nothing (or just "button") and cannot tell it deletes the
  document (WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value).
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
  Add an `alt` naming the owner (e.g. `alt={`${ownerName}'s avatar`}`, which needs the
  owner's name as a prop), or `alt=""` if the owner is named in adjacent text. Without
  `alt`, screen readers announce the image URL or "image", and screen reader users do not
  learn who owns the document, which the toolbar is meant to show (WCAG 1.1.1 Non-text Content).
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Delete target size and focus style not verifiable

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13"
evidence: |
  <div className="icon-button" onClick={onDelete}>
recommendation: >-
  The icon is 16 by 16 and no stylesheet for `.icon-button` is in the change or the repository.
  Check that the control is at least 24 by 24 CSS pixels or spaced (WCAG 2.5.8 Target Size
  (Minimum)) and, once it is a button, that its focus is visible (2.4.7 Focus Visible).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
