# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Open sort menu traps Tab and cannot be closed with Escape

```yaml
severity: high
confidence: high
location: "src/components/SortMenu.tsx:23-25"
evidence: |
  if (event.key === "ArrowDown" || event.key === "Tab") {
    event.preventDefault();
    items.current[(index + 1) % SORT_OPTIONS.length]?.focus();
recommendation: >-
  Keyboard users are trapped in the open menu (WCAG 2.1.2 No Keyboard Trap,
  2.1.1 Keyboard). With the menu open, Tab and Shift+Tab are both cancelled
  and move focus to the next item, wrapping round the four options, so focus
  never reaches the product list or the rest of the page; no Escape handler
  exists either, so the only way out is to pick an option, which changes the
  sort. Following the APG menu button pattern, let Tab close the menu without
  preventing the default move, and handle Escape to close the menu and return
  focus to the "Sort" button (button.current?.focus()). A native <select>
  labelled "Sort products" would give all of this for free.
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
  Screen reader users are told the wrong state (WCAG 4.1.2 Name, Role,
  Value): after activating "Sort: Newest" the menu opens, but the button
  still reports "collapsed", because aria-expanded is the constant "false".
  Bind it to the state: aria-expanded={open}.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
