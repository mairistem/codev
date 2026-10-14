# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Open sort menu traps Tab and ignores Escape

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
  Keyboard users (WCAG 2.1.2 No Keyboard Trap, 2.1.1 Keyboard): once the menu
  is open, pressing Tab or Shift+Tab is cancelled and cycles focus among the
  four items, and Escape does nothing, so the user cannot move on to the
  product list or dismiss the menu without picking a sort order. Stop handling
  Tab (let it close the menu and move focus on, as the WAI-ARIA APG menu button
  pattern does), and add an Escape handler that calls setOpen(false) and
  returns focus to the "Sort" button. Shift+Tab is also caught by the same
  branch and moves forward instead of backward. A native <select> would give
  all of this for free.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Menu button always announces "collapsed"

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:34"
evidence: |
  <button ref={button} type="button" aria-haspopup="menu" aria-expanded="false" onClick={() => setOpen(!open)}>
recommendation: >-
  Screen reader users (WCAG 4.1.2 Name, Role, Value): aria-expanded is
  hard-coded to "false", so after pressing Enter on "Sort: Newest" the button
  is still announced as collapsed while the menu is open. Bind it to the state:
  aria-expanded={open}.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
