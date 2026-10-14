# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `bucket` packs the premium rule into an unreadable bitwise one-liner

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: |
  Rewrite as plain, named steps: a constant `PREMIUM_THRESHOLD_CENTS = 1000` (10 euros),
  a comparison `price.cents >= PREMIUM_THRESHOLD_CENTS`, and a result typed with the two
  buckets the doc comment promises, e.g. `{ cheap: Price[]; premium: Price[] }` built with
  two `filter` calls or a simple loop. A reader of line 10 cannot tell, without the
  proposal, what `(p.cents >> 4) & 0x3e8` means: `0x3e8` is 1000 in hex, which suggests the
  10-euro threshold, but a shift-and-mask is a bit test, not a comparison — e.g. 999 cents
  gives `62 & 1000 = 40` (truthy, "hi") and 128 cents gives `8 & 1000 = 8` ("hi"), so the
  shape hides that the line does not compare against 10 euros at all. The rest of the line
  compounds it: single-letter names (`ps`, `a`, `p`), a comma expression returning the
  accumulator, `||=` creating arrays inside an index, and keys `"hi"`/`"lo"` that the doc
  comment calls "cheap" and "premium", typed as an open `Record<string, …>` so a caller
  cannot know which keys exist or that a bucket may be missing. The simple version makes the
  threshold visible and its correctness checkable; whether the current result is wrong is
  for the developer to confirm and fix.
history:
  - status: open
    by: newcomer
```
