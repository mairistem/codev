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
  Keyboard users cannot delete a document: the div has no tabindex, no role and
  no key handler, so Tab never reaches it and Enter or Space do nothing; screen
  readers do not expose it as a control (WCAG 2.1.1 Keyboard, 4.1.2 Name, Role,
  Value). Render a native `<button type="button" className="icon-button"
  onClick={onDelete}>`, which is focusable, announced as a button and activated
  by Enter and Space.
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
  The only content is `TrashIcon`, whose svg is `aria-hidden="true"`
  (src/components/icons.tsx:3), so the control has no text alternative: once it
  is a button, a screen reader announces just "button", and the user cannot
  tell it deletes the document (WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role,
  Value). Give the button an accessible name, e.g. `aria-label="Delete
  document"`, or visually hidden text inside it.
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
  The avatar conveys who owns the document, but the img has no `alt`: screen
  readers fall back to announcing the image URL or file name, or "image", and
  the owner is never identified (WCAG 1.1.1 Non-text Content). Pass the owner's
  name and set `alt={ownerName}` (or `alt=""` if the owner's name is already
  shown as text next to it).
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
  The icon is 16 by 16 (src/components/icons.tsx:3) and the `.icon-button`
  stylesheet is not part of the change: check that the clickable target is at
  least 24 by 24 CSS pixels or spaced accordingly (WCAG 2.5.8 Target Size
  (Minimum)) and that the button keeps a visible focus indicator (WCAG 2.4.7
  Focus Visible).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
