---
id: sy-0dvzg73
title: Compact the change log once every known replica has seen a change
category: storage
seq: 20
verify: run cargo test compact_
docs: [docs/PROTOCOL.md#changes-sp-changes]
status: open
created: 2026-10-02T17:21Z
---
The change log only grows. A change that lost its conflict, or was replaced later by its own device, is still kept and still sent.

Once every replica this one knows of has seen a superseded change, drop it from the log. A replica that comes back after compaction must still converge: anything it lacks arrives as the winning change.

## log
- 2026-10-02T17:21Z created
