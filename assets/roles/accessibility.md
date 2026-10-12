---
id: accessibility
name: { en: Accessibility, fr: Accessibilité }
at: [plan.challenge, code.review]
domains: [user-interface]
question: Can everyone use what this change adds, with a keyboard or assistive technology?
done_when:
  - Every interactive element is reachable and operable with the keyboard
    alone, in a logical focus order, with a visible focus
  - Every non-text content that conveys meaning has a text alternative
  - Every form field has a programmatic label, and errors are identified
    in text next to the field
  - Text and meaningful graphics meet the contrast minimums
  - Native semantic elements are used first; ARIA only adds what they
    cannot express, and matches the actual behavior
---

## Scope

Look at the user interface the change adds or modifies: pages,
components, dialogs, forms, messages, and the HTML or native views they
produce. Leave aside visual taste, and screens the change does not touch.

## Signals

- **Clickable non-control**: Example: `<div onclick=…>` with no role, no
  `tabindex`, no key handler — a `<button>` does all three.
- **Keyboard trap or lost focus**: a dialog that does not take focus, cannot
  be closed with Escape, or leaves focus behind it.
- **Focus removed**: Example: `outline: none` with no replacement style.
- **Missing text alternative**: an informative image with no `alt`, or an
  icon-only button with no accessible name.
- **Placeholder as label**: Example: `<input placeholder="Email">` with no
  `<label>`; the hint disappears as soon as the user types.
- **Errors by colour only**: a field turned red with no message, or a
  message not associated with its field (`aria-describedby`).
- **Low contrast**: grey text on white below 4.5:1 for normal text.
- **Wrong ARIA**: `role="button"` on an element that does not react to
  Enter and Space, or `aria-hidden="true"` on focusable content.
- **Dynamic updates not announced**: a toast or a result count that
  changes with no live region.
- **Small targets**: icon buttons smaller than 24 by 24 CSS pixels with no
  spacing.

## By stack

- **TypeScript/JavaScript**: in React, `onClick` on a `div` instead of a
  `button`; `jsx-a11y` lint rules disabled; focus not moved to a route's
  heading after client-side navigation; components that render a `label`
  without `htmlFor` pointing to the input.
- **Rust, Python, Go, Java**: server-side templates (Askama, Jinja,
  `html/template`, Thymeleaf) — check the generated HTML as above: `lang`
  on `<html>`, one `<h1>`, labels bound to inputs, tables with `<th>`.
  Native toolkits must expose names and roles to the platform's
  accessibility API.

## References

- WCAG 2.2 (W3C Recommendation) — success criteria to cite by number and
  name, for example 1.1.1 Non-text Content, 1.3.1 Info and Relationships,
  1.4.3 Contrast (Minimum), 1.4.11 Non-text Contrast, 2.1.1 Keyboard,
  2.1.2 No Keyboard Trap, 2.4.3 Focus Order, 2.4.7 Focus Visible, 2.4.11
  Focus Not Obscured (Minimum), 2.5.8 Target Size (Minimum), 3.3.1 Error
  Identification, 3.3.2 Labels or Instructions, 4.1.2 Name, Role, Value.
  Level AA is the usual target.
- RGAA 4 — the French reference framework for accessibility (référentiel
  général d'amélioration de l'accessibilité), based on WCAG and required of
  French public bodies.
- WAI-ARIA Authoring Practices Guide (APG) — keyboard patterns for
  dialogs, menus, tabs and comboboxes.

## Reporting

One line per finding: the element and its location, who is blocked
(keyboard users, screen reader users, low vision), the WCAG success
criterion by number and name, the severity (high when a task cannot be
completed with the keyboard or a screen reader; medium when it is possible
but confusing; low otherwise), and the fix — preferably the native element.
