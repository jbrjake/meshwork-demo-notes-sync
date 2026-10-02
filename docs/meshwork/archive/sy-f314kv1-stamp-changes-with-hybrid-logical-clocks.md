---
id: sy-f314kv1
title: Stamp changes with hybrid logical clocks
status: done
category: protocol
answers: meshwork-demo-notes-cli#nt-jmvjckh
verify: "all(run cargo test change_sorts_after_everything_its_author_saw, run cargo test v1_log_reads_with_counter_zero, run cargo test observed_at_is_local)"
seq: 5
created: 2026-10-02T17:52Z
covers:
  - ref: docs/PROTOCOL.md#sp-change-timestamp
    sha: 2a99d54ec1c4ceca1c94dacf0dabe8d7e287de1e79c33dc20ab98b3309e566cd
  - ref: docs/PROTOCOL.md#sp-conflict-resolution
    sha: 5a8fe833c0836e7a9003ea48ae101e6b61b1db69049c74731d0d136e4469bc4b
  - ref: docs/PROTOCOL.md#sp-observed-at
    sha: 514740670d593ddba9b8d46807ceb91a97c5337ac6009ecaae7e16cba01e0df2
---

notes asks that a change always beat the changes its author had already stored. Wall-clock timestamps cannot promise that once a device's clock runs fast.

Stamp changes with a hybrid logical clock: the later of the device's wall clock and the newest timestamp it has stored, plus a counter that breaks ties. A change then sorts after everything its author had seen. Changes made without seeing each other still race on timestamp, and the protocol says so.

A timestamp is no longer the time a change was made; it can run ahead of every device's clock. Add a local `observed_at`, when this replica first stored the change, for anyone who needs to show when something happened. It is never sent.

v0.1 logs must still load: their changes read with counter 0, and their observed_at is their timestamp. This is a breaking change to what a timestamp means, so it ships as v0.2.0.

## log
- 2026-10-02T17:52Z created
- 2026-10-02T17:52Z open→doing — claimed by claude (sync-1)
- 2026-10-02T17:52Z cover docs/PROTOCOL.md#sp-change-timestamp @2a99d54ec1c4
- 2026-10-02T17:52Z cover docs/PROTOCOL.md#sp-conflict-resolution @5a8fe833c083
- 2026-10-02T17:52Z cover docs/PROTOCOL.md#sp-observed-at @514740670d59
- 2026-10-02T17:52Z doing→done — verify exit 0 @ 8a67f02+2
