# Review record: add-sort-menu

<!-- codev-review: 1 -->

## code.review

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
  Keyboard users cannot leave the open menu without picking a sort order —
  WCAG 2.1.2 No Keyboard Trap (Level A), 2.1.1 Keyboard. Stop intercepting
  Tab (let it close the menu and move focus on, per the APG menu button
  pattern), and handle Escape: close the menu and return focus to the
  trigger button. Alternatively use a native <select>, which needs none of it.
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
  Screen reader users are told the menu is collapsed while it is open —
  WCAG 4.1.2 Name, Role, Value. Bind the state: aria-expanded={open}.
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
