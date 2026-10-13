# Review record: add-toolbar

<!-- codev-review: 1 -->

## code.review

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
  Render the delete action as a native `<button type="button" onClick={onDelete}>`,
  which is focusable, exposes the button role and reacts to Enter and Space
  (WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value). Keep a visible focus style on it
  (WCAG 2.4.7 Focus Visible).
history:
  - status: open
    by: accessibility
```

Keyboard-only users cannot focus or activate the delete action at all — the
`div` has no `tabindex`, no role and no key handler — so the change's one task
(deleting a document) cannot be completed without a mouse.

#### R-accessibility-2 Icon-only delete control has no accessible name

```yaml
severity: high
confidence: high
location: "src/components/Toolbar.tsx:14"
evidence: |
        <TrashIcon />
recommendation: >-
  Give the control a text name, e.g. `<button type="button" aria-label="Delete document">`
  or visually hidden text inside the button; `TrashIcon` renders an
  `aria-hidden="true"` svg, so it contributes no name
  (WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value).
history:
  - status: open
    by: accessibility
```

Screen reader users get no announcement for the control: its only content is
an svg marked `aria-hidden="true"` in `src/components/icons.tsx`, and there is
no `aria-label` or text.

#### R-accessibility-3 Owner's avatar image has no alt attribute

```yaml
severity: medium
confidence: high
location: "src/components/Toolbar.tsx:12"
evidence: |
      <img className="avatar" src={avatarUrl} />
recommendation: >-
  Add an `alt` naming the owner (pass the owner's name as a prop, e.g.
  `alt={ownerName}`), or `alt=""` if the owner is identified in text elsewhere
  (WCAG 1.1.1 Non-text Content).
history:
  - status: open
    by: accessibility
```

The component's own comment says the avatar shows the document's owner, so it
conveys meaning; without `alt`, screen readers announce the file name or URL
instead of who owns the document.

#### R-accessibility-4 Delete target may be smaller than 24 by 24 CSS pixels

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13-14"
evidence: |
      <div className="icon-button" onClick={onDelete}>
        <TrashIcon />
recommendation: >-
  Ensure the `.icon-button` target is at least 24 by 24 CSS pixels (padding
  around the 16 by 16 icon) or has enough spacing (WCAG 2.5.8 Target Size (Minimum)).
history:
  - status: open
    by: accessibility
```

The icon is 16 by 16 pixels; the `.icon-button` stylesheet is not in the diff,
so whether padding enlarges the target could not be verified.

### newcomer

Disabled by configuration.
