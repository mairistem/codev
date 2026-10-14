# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### accessibility

#### R-accessibility-1 Tab is captured inside the open sort menu

```yaml
severity: high
confidence: high
location: "src/components/SortMenu.tsx:23-25"
evidence: |
  if (event.key === "ArrowDown" || event.key === "Tab") {
    event.preventDefault();
    items.current[(index + 1) % SORT_OPTIONS.length]?.focus();
recommendation: >-
  Do not intercept Tab (or Shift+Tab, which reaches this branch too since
  only event.key is tested): let it move focus out of the menu and close
  the menu, as the WAI-ARIA APG menu button pattern requires. Today a
  keyboard user who opens "Sort" and presses Tab or Shift+Tab only cycles
  through the four options forever; the only way out is to pick an option,
  which changes the sort. WCAG 2.1.2 No Keyboard Trap and 2.4.3 Focus Order.
  A native <select> would avoid the whole custom key handling.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Escape does not close the sort menu

```yaml
severity: medium
confidence: high
location: "src/components/SortMenu.tsx:21-30"
evidence: |
  function onKeyDown(event: React.KeyboardEvent) {
recommendation: >-
  Handle Escape in onKeyDown: close the menu without calling onChange and
  return focus to the "Sort" button (button.current?.focus()). Today a
  keyboard or screen reader user who opens the menu by mistake and presses
  Escape stays in it; they cannot dismiss it without choosing an option.
  WCAG 2.1.1 Keyboard; WAI-ARIA APG menu button pattern.
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
  Bind it to the state: aria-expanded={open}. Today a screen reader user
  who activates "Sort" hears the button announced as collapsed while the
  menu is open, so the announced state contradicts what happened. WCAG
  4.1.2 Name, Role, Value.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
