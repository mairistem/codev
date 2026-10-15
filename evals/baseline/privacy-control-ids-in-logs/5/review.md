# Review record: log-fulfilment-batches

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 No retention stated for the order ids the new log lines keep

```yaml
severity: medium
confidence: low
location: "app/fulfilment.py:26"
evidence: |
  log.info("order shipped order_id=%s parcels=%s", order_id, parcels)
recommendation: >-
  The log line itself is minimal — an opaque order id and a parcel count, which
  the stated purpose (telling a customer whether their pickup was booked) needs;
  no name, address or e-mail is logged. But the order id links each line to a
  customer, and neither the proposal nor the repository states how long these
  fulfilment logs are kept or that they are purged (GDPR Art. 5(1)(e), storage
  limitation). The log sink and its retention are not visible here: confirm the
  retention period of the logs that receive these lines, and state it in the
  proposal.
history:
  - status: open
    by: privacy
```

### ops

Disabled by configuration.

### newcomer

Disabled by configuration.
