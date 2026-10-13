# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

### accessibility

#### R-accessibility-1 Tab is captured inside the open sort menu (keyboard trap)

```yaml
severity: high
confidence: high
location: "src/components/SortMenu.tsx:23-25"
evidence: |
    if (event.key === "ArrowDown" || event.key === "Tab") {
      event.preventDefault();
      items.current[(index + 1) % SORT_OPTIONS.length]?.focus();
recommendation: >-
  Keyboard users are trapped (WCAG 2.1.2 No Keyboard Trap, level A; also 2.1.1 Keyboard).
  A keyboard user opens "Sort: Newest" with Enter, focus moves to the first item,
  then presses Tab to move on to the product list: the default is prevented and
  focus wraps to the next menu item, forever, Shift+Tab included. The only way out
  is to pick an option, which the menu never tells the user — and which changes the
  sort unless they find the already-checked one. Stop handling Tab in onKeyDown:
  on Tab (and Shift+Tab), close the menu and let focus move on naturally (APG menu
  button pattern). With Tab no longer captured, give the items tabIndex={-1} so the
  menu is a single tab stop navigated with the arrow keys, as the APG roving focus
  expects.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Escape does not close the sort menu

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:26-29"
evidence: |
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      items.current[(index - 1 + SORT_OPTIONS.length) % SORT_OPTIONS.length]?.focus();
    }
recommendation: >-
  The handler reacts only to ArrowDown, Tab and ArrowUp; pressing Escape in the open
  menu does nothing, so a keyboard or screen reader user who opened it by mistake
  cannot dismiss it without choosing an option (WCAG 2.1.1 Keyboard; APG menu button
  pattern). Handle Escape in onKeyDown: setOpen(false) and move focus back to the
  menu button (button.current?.focus()), without calling onChange.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 aria-expanded is hard-coded to "false"

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:34"
evidence: |
      <button ref={button} type="button" aria-haspopup="menu" aria-expanded="false" onClick={() => setOpen(!open)}>
recommendation: >-
  Screen reader users are misled (WCAG 4.1.2 Name, Role, Value): the menu button
  always announces "collapsed", even after Enter opened the menu, so its state never
  matches what is on screen. Bind it to the state: aria-expanded={open}.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 Focus visibility of the menu items is not shown in the change

```yaml
severity: low
confidence: low
location: "src/components/SortMenu.tsx:33"
evidence: |
    <div className="sort-menu">
recommendation: >-
  The change adds a .sort-menu class but no stylesheet; the repository has none
  either, so whether the focused menu item and the menu button keep a visible focus
  indicator (WCAG 2.4.7 Focus Visible) and enough contrast (1.4.11) could not be
  checked. Verify in the stylesheet that styles .sort-menu that no rule removes the
  outline without a replacement.
history:
  - status: open
    by: accessibility
```
