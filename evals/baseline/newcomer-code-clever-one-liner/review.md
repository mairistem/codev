# Review record: group-prices

<!-- codev-review: 1 -->

## code.review

### newcomer

#### R-newcomer-1 `bucket` packs its grouping rule into an unreadable one-liner

```yaml
severity: medium
confidence: high
location: "src/pricing.ts:9-10"
evidence: |
  export const bucket = (ps: Price[]) =>
    ps.reduce<Record<string, Price[]>>((a, p) => ((a[(p.cents >> 4) & 0x3e8 ? "hi" : "lo"] ||= []).push(p), a), {});
recommendation: >-
  Rewrite `bucket` as a plain function a newcomer can read without the plan:
  name the threshold as a constant in cents (e.g. `PREMIUM_THRESHOLD_CENTS`)
  and compare `p.cents >= PREMIUM_THRESHOLD_CENTS` instead of the bit trick
  `(p.cents >> 4) & 0x3e8`, which a reader cannot translate into a price
  boundary (and which, read literally, tests bits rather than a threshold,
  so the shape hides what the rule really does); replace the comma-operator
  reduce with `||=` mutation by a loop or two `filter` calls; give the result
  an explicit type (`{ cheap: Price[]; premium: Price[] }`) with both keys
  always present, instead of `Record<string, Price[]>` with the magic keys
  "hi"/"lo" that the doc comment calls "cheap" and "premium"; and use
  descriptive names (`prices`, `price`, `groups`) instead of `ps`, `a`, `p`,
  and a verb such as `splitByPriceTier` instead of `bucket`.
history:
  - status: open
    by: newcomer
```
