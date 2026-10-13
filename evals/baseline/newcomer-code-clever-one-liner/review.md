# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

### newcomer

#### R-newcomer-1 The premium test is a bit trick that does not mean "10 euros and more"

```yaml
severity: high
confidence: high
location: "src/pricing.ts:10"
evidence: |
  ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Replace the bit trick with the rule the proposal states, behind a named
  constant: `const PREMIUM_THRESHOLD_CENTS = 1000;` and
  `p.cents >= PREMIUM_THRESHOLD_CENTS`. As written, `(cents >> 4) & 0x3e8`
  only tests some bits of `cents / 16` against the bit pattern of 1000:
  128 cents (1.28 €) gives 8 & 1000 = 8 and lands in "hi", 999 cents gives
  62 & 1000 = 40 and lands in "hi", while 16384 cents (163.84 €) gives
  1024 & 1000 = 0 and lands in "lo". A reader cannot see the threshold, and
  the proposal's "premium products (10 euros and more)" is not what runs.
history:
  - status: open
    by: newcomer
```

#### R-newcomer-2 `bucket` is a dense one-liner with a comma expression and `||=` inside an index

```yaml
severity: low
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Write it as a plain function: compute `isPremium(p)` once, then push into
  one of two arrays (or use two `filter` calls) and return them. A reduce
  whose callback mutates the accumulator through `||=` and returns it via a
  comma operator has to be decoded token by token; the simple version wins
  and costs nothing here.
history:
  - status: open
    by: newcomer
```

#### R-newcomer-3 Names say nothing: `bucket`, `ps`, `a`, `p`, `"hi"` / `"lo"`

```yaml
severity: low
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Name the function after what it does (`splitByPriceBand`, or
  `partitionPremium`), the parameter `prices`, and use keys that match the
  doc comment and the domain (`cheap` / `premium`) instead of the magic
  strings "hi" / "lo".
history:
  - status: open
    by: newcomer
```

#### R-newcomer-4 The return type `Record<string, Price[]>` hides its two keys and their possible absence

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:10"
evidence: |
  ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Declare the result as `{ cheap: Price[]; premium: Price[] }` and always
  initialise both arrays. Today a caller reading the signature cannot know
  the keys are "hi" and "lo", and when no price falls in a band the key is
  missing, so `bucket(prices).hi` is `undefined` while the type claims
  `Price[]`.
history:
  - status: open
    by: newcomer
```

#### R-newcomer-5 No test describes the split; the only verification is looking at a page

```yaml
severity: medium
confidence: high
location: "tasks.md#1. Implementation"
evidence: |
  - [x] 1.1 `bucket`, verified by the catalogue page, where premium products are listed apart
recommendation: >-
  Add a test that reads as the behavior: "a price of 999 cents is cheap",
  "a price of 1000 cents is premium", "a price of 16384 cents is premium",
  "an empty list gives two empty bands". Such a test would have caught
  R-newcomer-1.
history:
  - status: open
    by: newcomer
```
