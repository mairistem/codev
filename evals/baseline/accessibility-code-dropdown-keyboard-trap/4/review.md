# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Open sort menu traps Tab and ignores Escape

```yaml
severity: high
confidence: high
location: "src/components/SortMenu.tsx:23-25"
evidence: |
  if (event.key === "ArrowDown" || event.key === "Tab") {
    event.preventDefault();
    items.current[(index + 1) % SORT_OPTIONS.length]?.focus();
recommendation: >-
  A keyboard user opens the menu with Enter on "Sort: …" (focus moves to the
  first item), then presses Tab to move on to the product list: Tab is
  cancelled and focus cycles back through the four items forever; Shift+Tab
  does the same, and Escape has no handler, so the menu cannot be dismissed.
  The only way out is to activate an option, which changes the sort.
  WCAG 2.1.2 No Keyboard Trap (and the APG menu-button pattern). Do not
  intercept Tab: let it move focus out and close the menu; handle Escape to
  close the menu and return focus to the menu button. A native `<select>`
  with a `<label>` would avoid the custom keyboard handling altogether.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Menu button always announces collapsed

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:34"
evidence: |
  <button ref={button} type="button" aria-haspopup="menu" aria-expanded="false" onClick={() => setOpen(!open)}>
recommendation: >-
  `aria-expanded` is hard-coded to "false": once a screen reader user opens
  the menu, the button still announces "collapsed", so the state of the
  control misreports what is on screen. WCAG 4.1.2 Name, Role, Value. Bind it
  to the state: `aria-expanded={open}`.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
