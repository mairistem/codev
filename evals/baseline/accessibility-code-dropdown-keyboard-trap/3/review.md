# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Open sort menu traps keyboard focus: Tab cycles inside it and Escape does not close it

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
  WCAG 2.1.2 No Keyboard Trap, 2.1.1 Keyboard. A keyboard user opens the menu with Enter
  on "Sort: …" and focus moves to the first item; pressing Tab or Shift+Tab is
  prevented and only wraps to another item, and Escape is not handled, so the
  only way out is to activate an option — the user cannot leave the menu or
  dismiss it without (re)choosing a sort order, and cannot reach the product
  list or the rest of the page. Handle Escape to close the menu and return focus
  to the menu button; let Tab (and Shift+Tab) leave — close the menu without
  calling preventDefault, as the WAI-ARIA APG menu button pattern does; also
  close it when focus leaves the list. Alternatively, a native <select> labelled
  "Sort products" gives all of this for free.
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
  WCAG 4.1.2 Name, Role, Value. The expanded state is hard-coded: after a screen
  reader user activates "Sort: Newest, menu button, collapsed", the menu opens
  but the button still reports "collapsed", so the state the user hears never
  matches what is on screen. Bind it to the state — aria-expanded={open}.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
