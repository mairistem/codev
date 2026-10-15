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
recommendation: >-
  Keyboard users are trapped (WCAG 2.1.2 No Keyboard Trap, 2.1.1 Keyboard). Once the menu is
  open, focus moves to its first item; Tab — and Shift+Tab, since the shift key is not checked —
  is cancelled and wraps around the four items, and no handler closes the menu on Escape. The only
  way out is to activate an option, which changes the sort order. Per the APG menu button pattern:
  close the menu on Escape and return focus to the "Sort" button; let Tab/Shift+Tab leave the menu
  (close it without preventDefault) instead of cycling; or use a native <select> with a <label>,
  which needs none of this code.
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
  Screen reader users are misled (WCAG 4.1.2 Name, Role, Value): `aria-expanded` is the literal
  "false", so after pressing Enter on "Sort: Newest" the button still reports collapsed while the
  menu is open. Bind it to state — `aria-expanded={open}` — and, optionally, point `aria-controls`
  at the menu's id.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Visible focus on the menu items is unverified

```yaml
severity: low
confidence: low
location: "src/components/SortMenu.tsx:33"
evidence: |
  <div className="sort-menu">
recommendation: >-
  No stylesheet for `.sort-menu` is in the change or the repository, so the focus indicator of the
  button and of the menu items (WCAG 2.4.7 Focus Visible) and their contrast (1.4.11) cannot be
  checked here. Check in the app's styles that focused items keep a visible outline, since
  arrow-key navigation is the only cue of which option is active.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
