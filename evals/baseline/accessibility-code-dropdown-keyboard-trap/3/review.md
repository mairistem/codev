# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Open sort menu traps keyboard focus: Tab cycles inside and Escape does nothing

```yaml
severity: high
confidence: high
location: "src/components/SortMenu.tsx:21-30"
evidence: |
  if (event.key === "ArrowDown" || event.key === "Tab") {
    event.preventDefault();
    items.current[(index + 1) % SORT_OPTIONS.length]?.focus();
recommendation: >-
  WCAG 2.1.2 No Keyboard Trap (and 2.1.1 Keyboard). Once a keyboard user opens
  the menu, focus moves to the first item; pressing Tab (or Shift+Tab) is
  cancelled and wraps back to an item, and there is no Escape handler nor any
  close on blur, so the only way out is to pick an option — the user cannot
  leave the menu without changing the sort, nor reach the product list. Stop
  intercepting Tab (let it move on and close the menu), add Escape that closes
  the menu and returns focus to the button, as the APG menu button pattern
  requires; or use a native <select> with a <label>, which needs none of this.
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
  WCAG 4.1.2 Name, Role, Value. aria-expanded is hard-coded to "false", so when
  a screen reader user activates the button and the menu opens, the button is
  still announced as "collapsed" — the state contradicts the behaviour. Bind it
  to the state: aria-expanded={open}.
history:
  - status: open
    by: accessibility
```
