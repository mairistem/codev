# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

### newcomer

#### R-newcomer-1 bucket hides the premium threshold in a bitmask one-liner that does not compute it

```yaml
severity: high
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Replace the one-liner with a plain loop or two filters on a named constant,
  e.g. `const PREMIUM_THRESHOLD_CENTS = 1000;` and `p.cents >= PREMIUM_THRESHOLD_CENTS`,
  so that the rule from the proposal ("10 euros and more") reads in the code.
  The bitmask `(cents >> 4) & 0x3e8` is not a comparison with 1000 cents: 500 cents
  (5 euros) gives 31 & 1000 = 8, so "hi"; 16384 cents (163.84 euros) gives
  1024 & 1000 = 0, so "lo". A newcomer cannot see that the expression is wrong
  without decoding it bit by bit; the author must fix the behavior (the newcomer
  does not change it).
history:
  - status: open
    by: newcomer
```

#### R-newcomer-2 Obscure names and untyped string keys in bucket

```yaml
severity: low
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Name the function after what it does (e.g. `splitByPriceBand`), use readable
  parameter names (`prices`, `groups`, `price`), and return a typed shape such as
  `{ cheap: Price[]; premium: Price[] }` instead of `Record<string, Price[]>` with
  magic keys "hi"/"lo"; today a caller cannot know the keys without reading the
  body, and a band with no price is absent instead of empty.
history:
  - status: open
    by: newcomer
```

#### R-newcomer-3 The verification test cited by the task does not exist

```yaml
severity: medium
confidence: high
location: "tasks.md#1. Implementation"
evidence: |
  - [x] 1.1 `bucket`, verified by `npm test -- pricing`
recommendation: >-
  Add a `pricing` test that describes the behavior at the boundary (999 cents is
  cheap, 1000 cents is premium, a large price is premium). No test file is in the
  repository, so the task was checked off without a test that documents — or
  would have caught — the behavior of `bucket`.
history:
  - status: open
    by: newcomer
```

#### R-newcomer-4 Module comment reads as a comment on the interface

```yaml
severity: low
confidence: high
location: "src/pricing.ts:1-6"
evidence: |
  /** Prices of the catalogue, in cents. */

  export interface Price {
    sku: string;
    cents: number;
  }
recommendation: >-
  Put the unit where it applies: a doc comment on the `cents` field
  (`/** Amount in cents. */`) instead of a detached JSDoc block above the
  interface.
history:
  - status: open
    by: newcomer
```
