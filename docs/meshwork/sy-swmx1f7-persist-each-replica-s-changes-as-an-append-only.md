---
id: sy-swmx1f7
title: Persist each replica's changes as an append-only log
status: open
category: storage
verify: run cargo test log_
docs:
  - docs/PROTOCOL.md#sp-changes
created: 2026-10-02T17:16Z
---

Each replica appends every change it stores to `<dir>/changes.log`, one per line, whether it authored the change or received it. Log order is therefore the order this replica learned things in.

Columns are tab-separated: `device`, `seq`, `timestamp_ms`, `doc`, `fields`. Fields are `key=value` pairs joined by `;`. Tab, newline, backslash, `;` and `=` are backslash-escaped wherever they appear, so any text survives the round trip.

## log
- 2026-10-02T17:16Z created
