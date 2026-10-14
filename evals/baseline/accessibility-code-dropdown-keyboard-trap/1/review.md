# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Open sort menu traps keyboard focus: Tab is swallowed and Escape does nothing

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
recommendation: >
  Keyboard users (WCAG 2.1.2 No Keyboard Trap, 2.1.1 Keyboard). Once the menu opens, focus
  moves into it (line 12); pressing Tab or Shift+Tab is prevented and only cycles focus among
  the four items, and Escape has no handler, so a keyboard user cannot leave the menu or move
  on to the product list without picking an option. Follow the APG menu button pattern:
  handle Escape to close the menu and return focus to the "Sort" button, and on Tab let the
  default focus move happen while closing the menu (do not call preventDefault). Keep
  ArrowDown/ArrowUp for moving between items. A native <select> would avoid the whole issue.
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
recommendation: >
  Screen reader users (WCAG 4.1.2 Name, Role, Value). aria-expanded is hard-coded to "false",
  so when the user activates the "Sort" button and the menu opens, the button still announces
  "collapsed"; the state never matches the actual behavior. Bind it to the state:
  aria-expanded={open}.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
