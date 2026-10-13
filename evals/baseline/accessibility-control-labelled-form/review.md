# Review record: add-newsletter-form

<!-- codev-review: 1 -->

## code.review

### accessibility

#### R-accessibility-1 Hidden error message is always read as the field's description

```yaml
severity: medium
confidence: high
location: "templates/newsletter.html:10-15"
evidence: |2
      aria-describedby="newsletter-hint newsletter-error"
    />
    <p id="newsletter-hint" class="hint">One e-mail a month. Unsubscribe from any issue.</p>
    <p id="newsletter-error" class="error" role="alert" hidden>
      Enter an e-mail address such as name@example.org.
    </p>
recommendation: >
  Reference newsletter-error from aria-describedby only while the error is
  shown (add it together with aria-invalid="true" when the error is revealed,
  remove both when it is fixed). The accessible-description computation
  includes hidden content when it is referenced directly, so screen reader
  users hear "Enter an e-mail address such as name@example.org." on every
  focus of the field, before any error occurred (WCAG 1.3.1 Info and
  Relationships, 3.3.1 Error Identification).
history:
  - status: open
    by: accessibility
```

#### R-accessibility-2 Native validation disabled with nothing in the change to reveal the error

```yaml
severity: medium
confidence: low
location: "templates/newsletter.html:1"
evidence: |
  <form class="newsletter" action="/newsletter" method="post" novalidate>
recommendation: >
  novalidate turns off the browser's own required/type="email" messages, and
  the diff adds no script that removes `hidden` from #newsletter-error or sets
  aria-invalid on the field (nothing in src/ references the newsletter). As
  written, an empty or malformed address is submitted with no error
  identified in text next to the field (WCAG 3.3.1 Error Identification).
  Either drop novalidate, or add the client or server handling that shows
  #newsletter-error and sets aria-invalid="true", with a test. Confidence is
  low because such handling may exist outside what the reviewer could see.
history:
  - status: open
    by: accessibility
```

#### R-accessibility-3 Form landmark has no accessible name although its heading has an id

```yaml
severity: low
confidence: high
location: "templates/newsletter.html:1-2"
evidence: |
  <form class="newsletter" action="/newsletter" method="post" novalidate>
    <h2 id="newsletter-title">Get the monthly newsletter</h2>
recommendation: >
  Add aria-labelledby="newsletter-title" to the form: a form element is only
  exposed as a landmark when it has an accessible name, and the id on the
  heading is otherwise unused (WCAG 1.3.1 Info and Relationships, 2.4.1 Bypass
  Blocks).
history:
  - status: open
    by: accessibility
```

### newcomer

Disabled by configuration.
