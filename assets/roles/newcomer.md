---
id: newcomer
name: { en: Newcomer, fr: Nouvel arrivant }
at: code.review
question: Can a developer who has not seen the plan follow this code?
done_when:
  - "**Without context**: each changed function makes sense without the
    plan or the conversation"
  - "**Names** say what things do; no obscure abbreviation"
  - "**Size and nesting**: a function too long or too nested is split,
    when it splits simply"
  - "**Cleverness**: dense one-liners, premature genericity, single-use
    abstractions — the simple version wins unless the clever one is
    justified"
  - "**Comments** explain the why where it is not obvious, never
    paraphrase the code"
  - "**Tests** read as a description of the behavior"
  - The project's `rules`, such as size thresholds
---

## Signals

- **Boolean parameters**: `render(true, false)` — the call site cannot be
  read without the signature.
- **Long parameter lists**: more than four or five, often a missing type.
- **Deep nesting**: three levels of `if`/loop or more; an early return or an
  extracted function usually flattens it.
- **Mixed levels of abstraction**: parsing, business rules and I/O in one
  function.
- **Magic values**: `if status == 3` — a named constant or an enum says why.
- **Misleading names**: `get_user` that writes, `data`, `tmp`, `handle`.
- **Comments that paraphrase**: `// increment i` above `i += 1`; or a
  comment that contradicts the code.
- **Tests that hide the behavior**: a test named `test_1`, or one that
  asserts on internals rather than on what the caller sees.

## By stack

- **Rust**: `.unwrap()` on a value that can legitimately be absent;
  needless `.clone()` to quiet the borrow checker; long iterator chains with
  side effects in `map`; a generic or a trait with a single implementation.
- **TypeScript/JavaScript**: `any` or non-null `!` hiding a real case;
  nested ternaries; promise chains mixed with `async`/`await`; a callback
  that mutates outer state.
- **Python**: mutable default arguments (`def f(x=[])`); bare
  `except:`; nested comprehensions with conditions; `**kwargs` passed
  through several layers.
- **Go**: an `err` dropped with `_`; an interface declared next to its
  only implementation rather than where it is consumed; a goroutine started
  with no clear owner or stop condition.
- **Java**: a class hierarchy for one variant; checked exceptions caught and
  swallowed; a builder or factory for an object with two fields; `Optional`
  used as a field or parameter.
