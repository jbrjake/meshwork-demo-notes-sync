# Changelog

## 0.1.0

The first release.

- A replica keeps every change it stores in an append-only log, `<dir>/changes.log`.
- Conflicting changes to a document resolve last-writer-wins by timestamp, with ties broken on device id. Timestamps are device wall-clock time, so a device whose clock runs fast wins conflicts it should lose.
- Replicas sync by trading the changes the other has not seen, by version vector. Applying a change twice is harmless.
- `ManualClock` stands in for `SystemClock` in tests and simulations.
