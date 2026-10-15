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
  Render the delete action as a native `<button type="button" className="icon-button" onClick={onDelete}>`.
  A keyboard user pressing Tab never reaches the div (no tabindex), and Enter or Space
  does nothing (no key handler), so they cannot delete a document — the very task the
  change adds (WCAG 2.1.1 Keyboard, 4.1.2 Name, Role, Value). A screen reader announces
  no role either. The native button brings focus, Enter/Space activation and the button role.
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
  Give the control a text name, e.g. `aria-label="Delete document"` on the button (or
  visually hidden text inside it). Its only content is `TrashIcon`, whose `<svg>` is
  `aria-hidden="true"` (src/components/icons.tsx:3), so a screen reader user reaching the
  control hears nothing (or just "button" once R-accessibility-1 is fixed) and cannot tell
  it deletes the document (WCAG 1.1.1 Non-text Content, 4.1.2 Name, Role, Value).
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
  The avatar conveys who owns the document (per the component's doc comment), but the
  `<img>` has no `alt`: a screen reader announces the image file name or URL instead of
  the owner, so screen reader users do not learn who owns the document (WCAG 1.1.1
  Non-text Content). Pass the owner's name as a prop and set `alt={ownerName}` (e.g.
  "Owner: Jane Doe"); if the owner is shown as text elsewhere, use `alt=""` instead.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Focus visibility and target size of the icon button not verifiable

```yaml
severity: low
confidence: low
location: "src/components/Toolbar.tsx:13"
evidence: |
  <div className="icon-button" onClick={onDelete}>
recommendation: >-
  The `icon-button` style is not in the change and the trash icon is 16x16
  (src/components/icons.tsx:3). Check that the control shows a visible focus indicator
  (WCAG 2.4.7 Focus Visible), that its target is at least 24x24 CSS pixels or spaced
  enough (WCAG 2.5.8 Target Size (Minimum)), and that the icon stroke meets 3:1 against
  the toolbar background (WCAG 1.4.11 Non-text Contrast).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
