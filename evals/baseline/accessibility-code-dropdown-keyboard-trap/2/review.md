# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Keyboard trap: Tab cycles inside the open sort menu and Escape does not close it

```yaml
severity: high
confidence: high
location: "src/components/SortMenu.tsx:21-30"
evidence: |
    if (event.key === "ArrowDown" || event.key === "Tab") {
      event.preventDefault();
      items.current[(index + 1) % SORT_OPTIONS.length]?.focus();
recommendation: >-
  WCAG 2.1.2 No Keyboard Trap and 2.1.1 Keyboard. A keyboard user opens the
  menu (focus moves to the first item), then presses Tab to leave: the
  handler cancels Tab and wraps focus back among the four items; Shift+Tab
  is cancelled too, and no Escape handler exists, so the only way out is to
  choose an option, which changes the sort. Remove "Tab" from the handler
  and close the menu when Tab moves focus out (APG menu button pattern);
  handle Escape to close the menu and return focus to the button
  (button.current?.focus()). Alternatively use a native <select> with a
  <label>, which has none of these problems.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 The menu button always announces "collapsed"

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:34"
evidence: |
      <button ref={button} type="button" aria-haspopup="menu" aria-expanded="false" onClick={() => setOpen(!open)}>
recommendation: >-
  WCAG 4.1.2 Name, Role, Value. aria-expanded is hard-coded to "false": when
  a screen reader user activates the button and the menu opens, the button
  still reports "collapsed", so its state contradicts what is on screen.
  Bind it to the state: aria-expanded={open}.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
