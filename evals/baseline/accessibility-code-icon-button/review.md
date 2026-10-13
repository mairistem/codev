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
  Render the delete action as a native `<button type="button" onClick={onDelete}>`.
  As a `div` with only `onClick`, it has no role, is not in the tab order and does
  not react to Enter or Space, so keyboard and screen reader users cannot delete a
  document — the very task the proposal adds. WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value.
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
  Give the control a name, e.g. `<button type="button" aria-label="Delete document">`.
  Its only content is `TrashIcon`, whose `<svg>` is `aria-hidden="true"`
  (src/components/icons.tsx:3) and has no title, so screen reader users hear nothing
  that says what the control does. WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value.
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
  Add a text alternative: `alt` naming the owner (e.g. `alt={ownerName}`, which needs
  the owner's name as a prop) since the proposal presents the avatar as showing the
  owner, or `alt=""` if the name is shown in text elsewhere. Without `alt`, screen
  readers announce the image file name or URL. WCAG 1.1.1 Non-text Content.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Delete target likely smaller than 24×24 CSS pixels

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13"
evidence: |
  <div className="icon-button" onClick={onDelete}>
recommendation: >-
  Ensure the `icon-button` hit area is at least 24 by 24 CSS pixels (padding around the
  16×16 icon from src/components/icons.tsx:3). The `icon-button` styles were not found
  in the diff, so the actual size could not be verified. WCAG 2.5.8 Target Size (Minimum).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
