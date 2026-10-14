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
  Keyboard users cannot delete a document: the div has no tabindex, no role
  and no key handler, so Tab never lands on it and Enter/Space do nothing
  (WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value). Render a native
  `<button type="button" className="icon-button" onClick={onDelete}>`, which
  is focusable, announced as a button and activated by Enter and Space; make
  sure the `.icon-button` style keeps a visible focus indicator (2.4.7 Focus
  Visible).
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
  (src/components/icons.tsx:3), so the control has no text at all: a screen
  reader user reaching it hears nothing, or just "button" once it becomes a
  button, and cannot tell it deletes the document (WCAG 1.1.1 Non-text
  Content, 4.1.2 Name, Role, Value). Give the button a name, e.g.
  `aria-label="Delete document"`, or visually hidden text inside it.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Owner avatar image has no text alternative

```yaml
severity: medium
confidence: high
location: "src/components/Toolbar.tsx:12"
evidence: |
  <img className="avatar" src={avatarUrl} />
recommendation: >-
  The avatar conveys who owns the document, but the img has no `alt`: screen
  readers announce the image URL or file name instead, or nothing useful
  (WCAG 1.1.1 Non-text Content). Pass the owner's name to the component and
  render `alt={ownerName}` (or `alt=""` only if the name is shown as text
  next to it).
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Delete target size and focus style to check

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13-14"
evidence: |
  <div className="icon-button" onClick={onDelete}>
    <TrashIcon />
recommendation: >-
  The trash icon is 16 by 16 pixels and the `.icon-button` stylesheet is not
  in the change, so the target may be smaller than 24 by 24 CSS pixels
  (WCAG 2.5.8 Target Size (Minimum)) and its focus style and icon contrast
  (2.4.7, 1.4.11) cannot be verified. Check the stylesheet: at least 24x24
  hit area (padding), a visible focus outline, and 3:1 contrast for the icon.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
