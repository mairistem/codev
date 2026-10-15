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
recommendation: >-
  WCAG 2.1.2 No Keyboard Trap (and 2.1.1 Keyboard). A keyboard user who opens
  "Sort: …" gets focus moved into the menu (line 12); pressing Tab (or
  Shift+Tab) is prevented and only cycles to the next item, and Escape has no
  handler, so the only way out is to pick an option — the user cannot leave
  the menu without changing the sort, nor reach the product list or the rest
  of the page. Per the APG menu button pattern, handle Escape (close the menu,
  return focus to the menu button without calling onChange) and stop
  intercepting Tab: let it close the menu and move focus on naturally.
  Also close the menu when focus leaves it, and add a test pressing Tab and
  Escape in the open menu.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Menu button always announces "collapsed", even when the menu is open

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:34"
evidence: |
  <button ref={button} type="button" aria-haspopup="menu" aria-expanded="false" onClick={() => setOpen(!open)}>
recommendation: >-
  WCAG 4.1.2 Name, Role, Value. aria-expanded is hard-coded to "false": after a
  screen reader user presses Enter on "Sort: Newest", the menu opens but the
  button still reports "collapsed", so the state change is misreported. Bind it
  to the state: aria-expanded={open}.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
