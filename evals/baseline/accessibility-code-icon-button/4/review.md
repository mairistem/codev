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
  A keyboard user pressing Tab never lands on the div (no tabindex), and Enter or Space does nothing
  (no key handler); a screen reader user hears no role, so the only action of the toolbar cannot be
  found or triggered without a mouse — WCAG 2.1.1 Keyboard and 4.1.2 Name, Role, Value. The native
  button brings focus, Enter/Space activation and the button role at once.
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
  Give the control a text name, e.g. `aria-label="Delete document"` on the button (or visually hidden
  text inside it). Its only content is `TrashIcon`, whose `<svg ... aria-hidden="true">`
  (src/components/icons.tsx:3) hides it from assistive technology, so a screen reader user reaching
  the control hears nothing that says it deletes the document — WCAG 1.1.1 Non-text Content and
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
  Add a text alternative naming the owner, e.g. `alt={`${ownerName}'s avatar`}` (the component then
  needs the owner's name as a prop), or `alt=""` if the owner is named in adjacent text. With no `alt`,
  screen readers announce the image file name or URL instead of who owns the document, so the
  information the avatar conveys is lost — WCAG 1.1.1 Non-text Content.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Delete target size and focus style not verifiable in the diff

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13"
evidence: |
  <div className="icon-button" onClick={onDelete}>
recommendation: >-
  The `icon-button` class is styled outside the change and the trash icon is 16 by 16 pixels
  (src/components/icons.tsx:3). Check that the control is at least 24 by 24 CSS pixels or spaced
  enough (WCAG 2.5.8 Target Size (Minimum)) and that it shows a visible focus indicator once it is
  a button (WCAG 2.4.7 Focus Visible).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
