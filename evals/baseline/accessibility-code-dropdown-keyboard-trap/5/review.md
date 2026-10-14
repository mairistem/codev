# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Tab is captured inside the open sort menu: keyboard trap

```yaml
severity: high
confidence: high
location: "src/components/SortMenu.tsx:23-25"
evidence: |
  if (event.key === "ArrowDown" || event.key === "Tab") {
    event.preventDefault();
    items.current[(index + 1) % SORT_OPTIONS.length]?.focus();
recommendation: >-
  WCAG 2.1.2 No Keyboard Trap, 2.1.1 Keyboard. A keyboard user opens the menu
  (Enter on "Sort: …"), focus moves to "Newest"; pressing Tab or Shift+Tab is
  swallowed by preventDefault and only cycles through the four options, so focus
  can never leave the menu except by choosing an option — the user cannot reach
  the product list or the rest of the page without changing the sort. Remove
  "Tab" from this branch; on Tab, close the menu (setOpen(false)) and let the
  browser move focus naturally, as the WAI-ARIA APG menu button pattern requires.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 The open menu cannot be closed with Escape

```yaml
severity: medium
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
  WCAG 2.1.1 Keyboard, 2.1.2 No Keyboard Trap; APG menu button pattern. A
  keyboard or screen reader user who opens the menu and decides not to change
  the sort presses Escape: nothing happens, the menu stays open with focus inside
  it. The handler knows only ArrowDown, ArrowUp and Tab. Add an "Escape" branch
  that calls setOpen(false) and returns focus to the "Sort" button
  (button.current?.focus()), without calling onChange.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 aria-expanded is hard-coded to "false" on the menu button

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:34"
evidence: |
  <button ref={button} type="button" aria-haspopup="menu" aria-expanded="false" onClick={() => setOpen(!open)}>
recommendation: >-
  WCAG 4.1.2 Name, Role, Value. When the menu is open, a screen reader still
  announces the "Sort" button as "collapsed", so its users are told the menu is
  closed while it is open, and after closing it get no change of state. Bind the
  attribute to the state: aria-expanded={open}.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
