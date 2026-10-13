# Review record: add-size-picker

<!-- codev-review: 1 -->

## code.review

### accessibility

#### R-accessibility-1 Custom combobox where a native select would do

```yaml
severity: medium
confidence: high
location: "src/components/SizePicker.tsx:66"
evidence: |
  role="combobox"
recommendation: >
  Use a native `<select>` with a `<label htmlFor>` for the size. It gives
  keyboard operation, type-ahead, name/role/value and, on phones, the
  platform's large native picker, which addresses the proposal's "Why"
  (mis-taps on tiny targets). Keep a custom ARIA combobox only if a
  documented need goes beyond what `<select>` offers. This is the "native
  semantic elements first" criterion; WCAG 4.1.2 Name, Role, Value.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Highlighted option has no visible style

```yaml
severity: medium
confidence: low
location: "src/components/SizePicker.tsx:86"
evidence: |
  className={index === active ? "option option--active" : "option"}
recommendation: >
  Focus stays on the combobox and the keyboard position is shown only by
  the `option--active` class. No stylesheet in the project defines that
  class, so sighted keyboard users cannot see which option Enter or Tab will
  choose (WCAG 2.4.7 Focus Visible, 1.4.11 Non-text Contrast). Ship a style
  for `.option--active` with enough contrast, plus a visible focus style on
  the combobox.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Target size of the combobox and options is not set

```yaml
severity: medium
confidence: low
location: "src/components/SizePicker.tsx:79"
evidence: |
  <ul role="listbox" id={`${id}-list`} aria-labelledby={`${id}-label`} hidden={!open}>
recommendation: >
  The change exists because customers on phones mis-tap tiny controls. The
  diff adds no styling for the combobox or the `li` options, and the project
  has no stylesheet that sizes them. So nothing guarantees targets of at least
  24 by 24 CSS pixels (WCAG 2.5.8 Target Size (Minimum)), and nothing makes
  them larger than the radio buttons they replace. Give the combobox and each
  option a minimum size, with 44 px as the target for touch.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-4 aria-selected marks the committed value, not the highlighted option

```yaml
severity: low
confidence: high
location: "src/components/SizePicker.tsx:85"
evidence: |
  aria-selected={size === value}
recommendation: >
  In the APG select-only combobox that the component's doc comment cites, the
  option that `aria-activedescendant` references carries
  `aria-selected="true"`. Here every option the user arrows to except the
  current value is announced as "not selected", which confuses screen reader
  users (WCAG 4.1.2 Name, Role, Value). Set `aria-selected={index === active}`.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-5 Highlight is not reset to the current value when the list opens

```yaml
severity: low
confidence: high
location: "src/components/SizePicker.tsx:11"
evidence: |
  const [active, setActive] = useState(Math.max(0, sizes.indexOf(value)));
recommendation: >
  `active` is computed only on the first render. After the user arrows to an
  option and presses Escape, or after the parent changes `value`, reopening
  the list highlights and announces a stale option instead of the current
  size. Enter or Tab then commits that stale option. Set
  `active` to `sizes.indexOf(value)` each time the list opens (lines 22-24 and
  74).
history:
  - status: open
    by: accessibility
```

#### R-accessibility-6 No type-ahead on the select-only combobox

```yaml
severity: low
confidence: high
location: "src/components/SizePicker.tsx:28-58"
evidence: |
  switch (event.key) {
recommendation: >
  The key handler covers only arrows, Home, End, Enter, Space, Escape and Tab.
  The APG select-only combobox pattern that the doc comment claims to follow
  also moves to the next option that starts with a typed character, and a
  native `<select>` does this too. Add type-ahead, or use `<select>` (see
  R-accessibility-1).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
