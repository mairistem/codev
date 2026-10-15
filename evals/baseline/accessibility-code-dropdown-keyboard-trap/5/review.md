# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Open sort menu traps Tab and cannot be closed with Escape

```yaml
severity: high
confidence: high
location: "src/components/SortMenu.tsx:21-30"
evidence: |
  if (event.key === "ArrowDown" || event.key === "Tab") {
    event.preventDefault();
    items.current[(index + 1) % SORT_OPTIONS.length]?.focus();
recommendation: >
  Keyboard users (WCAG 2.1.2 No Keyboard Trap, 2.1.1 Keyboard). Once the menu is
  open, pressing Tab or Shift+Tab is cancelled and focus wraps among the four
  menu items, and the handler has no Escape branch, so the only way out is to
  activate an item, which also fires onChange. A keyboard user who opens the
  menu cannot Tab past it to the product list, nor dismiss it without picking
  an order. Per the APG menu-button pattern: on Escape, close the menu and
  return focus to the button; on Tab, do not preventDefault — close the menu
  and let focus move on naturally. Also close it when focus leaves the menu.
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
recommendation: >
  Screen reader users (WCAG 4.1.2 Name, Role, Value). aria-expanded is
  hard-coded to "false", so after pressing Enter on "Sort: Newest" the button
  still reports collapsed while the menu is open, misreporting its state.
  Bind it to the state: aria-expanded={open}.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
