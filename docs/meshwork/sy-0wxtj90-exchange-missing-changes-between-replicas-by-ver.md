---
id: sy-0wxtj90
title: Exchange missing changes between replicas by version vector
status: open
category: sync
verify: run cargo test sync_
docs:
  - docs/PROTOCOL.md#sync-sp-sync
created: 2026-10-02T17:18Z
covers:
  - ref: docs/PROTOCOL.md#sp-sync
    sha: 56084d78bcc1fe15c59a87f2ee92dd913ed5bc7123acfa23595313e784d46ae3
---

Two replicas sync by trading what the other lacks: `a.changes_since(&b.seen())` goes to `b.apply`, and the reverse. `seen()` is a version vector, the highest sequence number stored from each device. Applying a change twice is harmless, and a change that skips one of its device's earlier changes is refused rather than stored with a hole behind it.

Carrying the changes between devices is the caller's job.

## log
- 2026-10-02T17:18Z created
- 2026-10-02T17:18Z cover docs/PROTOCOL.md#sp-sync @56084d78bcc1
