---
id: sy-pn651c6
title: Resolve conflicting changes last-writer-wins by timestamp
status: done
category: merge
verify: run cargo test lww_
docs:
  - docs/PROTOCOL.md#conflict-resolution-sp-conflict-resolution
created: 2026-10-02T17:17Z
covers:
  - ref: docs/PROTOCOL.md#sp-conflict-resolution
    sha: 706c89c1367eae12151a6cac3dea64d0e8753c07f49142d5b53a8c580769f338
---

A replica resolves each document to one winning change: the higher timestamp wins, and equal timestamps break on device id. Every replica that stored the same changes must agree, whatever order they arrived in.

The replica takes its time from a `Clock`. `SystemClock` reads the wall clock; `ManualClock` reads whatever it was set to, so tests and simulations can give each device its own skew.

This trusts device clocks, as the protocol says: a device whose clock runs fast wins conflicts it should lose.

## log
- 2026-10-02T17:17Z created
- 2026-10-02T17:17Z cover docs/PROTOCOL.md#sp-conflict-resolution @706c89c1367e
- 2026-10-02T17:17Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
- 2026-10-02T17:18Z doing→done — verify exit 0 @ e309fa3+1
