---
id: sy-0wxtj90
title: Exchange missing changes between replicas by version vector
status: done
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
- 2026-10-02T17:18Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
- 2026-10-02T17:19Z doing→done — verify exit 0 @ 78213d2+1
