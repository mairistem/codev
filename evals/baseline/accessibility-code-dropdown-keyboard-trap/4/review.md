# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Open sort menu traps Tab and cannot be dismissed with Escape

```yaml
severity: high
confidence: high
location: "src/components/SortMenu.tsx:21-30"
evidence: |
  function onKeyDown(event: React.KeyboardEvent) {
    const index = items.current.findIndex((item) => item === document.activeElement);
    if (event.key === "ArrowDown" || event.key === "Tab") {
      event.preventDefault();
      items.current[(index + 1) % SORT_OPTIONS.length]?.focus();
recommendation: >-
  WCAG 2.1.2 No Keyboard Trap (and 2.1.1 Keyboard). A keyboard user opens the
  menu with Enter on "Sort: …", focus moves to the first item, and from then
  Tab and Shift+Tab (both reported as key "Tab") are cancelled and only cycle
  through the four items; Escape is not handled. The user cannot leave the
  menu to reach the product list or the rest of the page without picking a
  sort option, and nothing tells them so. Stop intercepting Tab (let it close
  the menu and move on, as the APG menu button pattern does), handle Escape to
  close the menu and return focus to the button, and close the menu when focus
  leaves it. A native `<select>` with a `<label>` would give all of this for free.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Menu button always announces aria-expanded="false"

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:34"
evidence: |
  <button ref={button} type="button" aria-haspopup="menu" aria-expanded="false" onClick={() => setOpen(!open)}>
recommendation: >-
  WCAG 4.1.2 Name, Role, Value. `aria-expanded` is hard-coded to "false", so
  a screen reader user who activates the button hears "Sort: Newest, menu
  button, collapsed" while the menu is open, and the state never reflects the
  popup. Bind it to the state: `aria-expanded={open}`.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
