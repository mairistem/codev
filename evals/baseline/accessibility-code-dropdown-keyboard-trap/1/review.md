# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Open sort menu traps keyboard focus: Tab is captured and Escape does nothing

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
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      items.current[(index - 1 + SORT_OPTIONS.length) % SORT_OPTIONS.length]?.focus();
    }
  }
recommendation: >
  Keyboard users (WCAG 2.1.2 No Keyboard Trap, 2.1.1 Keyboard). A keyboard user opens the menu
  with Enter on "Sort: …"; focus moves to "Newest". Pressing Tab (or Shift+Tab) is prevented and
  only cycles focus among the four options, and Escape has no handler, so the menu cannot be
  dismissed and focus cannot leave it to reach the product list or the rest of the page; the only
  way out is to pick an option, which changes (or re-confirms) the sort. Clicking outside does not
  close it either. Follow the APG menu button pattern: stop handling Tab (let it close the menu and
  move focus on naturally), add Escape to close the menu and return focus to the button, and close
  the menu on blur/outside click. Alternatively, use a native `<select>` with a `<label>`, which
  brings all of this for free.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Menu button always announces "collapsed": aria-expanded is hard-coded to false

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:34"
evidence: |
  <button ref={button} type="button" aria-haspopup="menu" aria-expanded="false" onClick={() => setOpen(!open)}>
recommendation: >
  Screen reader users (WCAG 4.1.2 Name, Role, Value). When the menu is open, the button still
  reports `aria-expanded="false"`, so a screen reader announces "Sort: Newest, menu button,
  collapsed" while the menu is shown, misleading the user about the state of the control. Bind the
  attribute to the state: `aria-expanded={open}`.
history:
  - status: open
    by: accessibility
```
