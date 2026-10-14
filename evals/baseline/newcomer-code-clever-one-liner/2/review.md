# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs its rule into an unreadable one-liner with a magic bitmask

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  A developer who has not seen the plan cannot answer "what makes a price
  premium?" from this line: the rule is hidden in `(p.cents >> 4) & 0x3e8`, a
  shift and a bitmask with no name and no comment, while the docstring talks of
  "cheap" and "premium" and the keys say "lo" and "hi". The reader is likely to
  assume a threshold comparison, but a bitwise AND of selected bits is not one
  (e.g. 16000 cents gives 1000 & 1000 → "hi", 16384 cents gives 1024 & 1000 → 0
  → "lo"), so the shape hides what the code actually does. On top of that the
  line combines a `reduce` with a comma-operator return, `||=` lazy init inside
  an index expression, single-letter names (`ps`, `a`, `p`) and an untyped
  `Record<string, …>` result whose keys only appear as string literals. Rewrite
  it as a named function with a named threshold constant in cents and an
  explicit comparison (`const PREMIUM_THRESHOLD_CENTS = …; const isPremium =
  (p: Price) => p.cents >= PREMIUM_THRESHOLD_CENTS;`), a plain loop or two
  `filter` calls, and a typed result such as `{ cheap: Price[]; premium:
  Price[] }` whose keys match the docstring; if the bitmask really is the
  intended rule, a comment must say why.
history:
  - status: open
    by: newcomer
```
