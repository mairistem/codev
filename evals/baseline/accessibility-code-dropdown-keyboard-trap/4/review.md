# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Disabled by configuration.

### accessibility

#### R-accessibility-1 Open sort menu traps the keyboard: Tab is captured and Escape does nothing

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
  WCAG 2.1.2 No Keyboard Trap / 2.1.1 Keyboard, blocks keyboard and screen reader users.
  A keyboard user opens "Sort", focus moves into the menu; pressing Tab (or Shift+Tab,
  which also has event.key "Tab" and moves forward) only cycles among the four items,
  and Escape is not handled, so the only way out is to activate an item — the user
  cannot dismiss the menu and reach the product list without picking a sort order.
  Follow the APG menu button pattern: stop calling preventDefault on Tab and let Tab /
  Shift+Tab close the menu and move focus naturally; handle Escape to close the menu
  and return focus to the button (button.current?.focus()); also close it when focus
  leaves the list (onBlur with relatedTarget outside). Alternatively use a native
  <select> labelled "Sort", which provides all of this.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Menu button always announces `aria-expanded="false"`

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:34"
evidence: |
  <button ref={button} type="button" aria-haspopup="menu" aria-expanded="false" onClick={() => setOpen(!open)}>
recommendation: >-
  WCAG 4.1.2 Name, Role, Value, misleads screen reader users. aria-expanded is a
  hard-coded string, so after activating "Sort" the button is still announced as
  "collapsed" while the menu is open, and the state never matches the behaviour.
  Bind it to the state: aria-expanded={open}; optionally add aria-controls pointing
  to the menu's id.
history:
  - status: open
    by: accessibility
```
