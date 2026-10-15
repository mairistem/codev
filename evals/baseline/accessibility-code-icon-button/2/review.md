# Review record: add-toolbar

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Delete action is a clickable div, unreachable and inoperable by keyboard

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
  A keyboard user pressing Tab never lands on the div (no tabindex), and Enter or Space does nothing
  (no key handler), so they cannot delete a document — the change's only feature. A screen reader
  announces no button role either. WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value. Keep the existing
  visual style and make sure the focus indicator stays visible (2.4.7).
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
  The only content is `TrashIcon`, whose `<svg>` is `aria-hidden="true"` (src/components/icons.tsx),
  so the control has no text at all: a screen reader user reaching it hears nothing (or "button" once
  it is a button) and cannot tell it deletes the document. Give the button a name, e.g.
  `aria-label="Delete document"` or visually hidden text. WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value.
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
  With no `alt`, screen readers fall back to announcing the image URL or file name, and the owner
  identity the proposal says the toolbar shows is not conveyed. Pass the owner's name in and set
  `alt={`${ownerName}'s avatar`}` (or `alt=""` if the name is shown as text next to it).
  WCAG 1.1.1 Non-text Content.
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
  The icon is 16 by 16 and the `icon-button` stylesheet is not in the change: check that the control
  is at least 24 by 24 CSS pixels (or spaced) and has a visible focus indicator and 3:1 non-text
  contrast. WCAG 2.5.8 Target Size (Minimum), 2.4.7 Focus Visible, 1.4.11 Non-text Contrast.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
