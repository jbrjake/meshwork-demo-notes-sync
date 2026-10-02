# Changelog

## 0.2.0

Breaking: a timestamp no longer means wall-clock time.

- Changes are stamped with a hybrid logical clock: the later of the device's wall clock and the newest timestamp it has stored, plus a counter (`Change::counter`, `Change::stamp()`). An edit now always beats the changes its device had already stored, even when another device's clock runs fast. Edits made without seeing each other still race on timestamp.
- A timestamp can run ahead of every device's clock. `Doc::observed_at_ms()` gives when this replica first stored the winning change, by its own clock; it is local and never sent.
- The log gains two columns, the counter and the observed time (`log::Entry`). v0.1 logs still load: their counter is 0 and their observed time is their timestamp.

## 0.1.0

The first release.

- A replica keeps every change it stores in an append-only log, `<dir>/changes.log`.
- Conflicting changes to a document resolve last-writer-wins by timestamp, with ties broken on device id. Timestamps are device wall-clock time, so a device whose clock runs fast wins conflicts it should lose.
- Replicas sync by trading the changes the other has not seen, by version vector. Applying a change twice is harmless.
- `ManualClock` stands in for `SystemClock` in tests and simulations.
