---
id: sy-xejvc9k
title: Say in the README that conflict resolution trusts device clocks
status: done
category: docs
verify: "all(exists README.md, contains README.md /trusts device clocks/)"
docs:
  - docs/PROTOCOL.md#conflict-resolution-sp-conflict-resolution
created: 2026-10-02T17:19Z
---

The README introduces the library to whoever is deciding whether to sync through it: what a replica is, how two devices sync, what lands on disk. It also states the caveat in plain words. Conflict resolution trusts device clocks, so a device whose clock runs fast wins conflicts it should lose. Keep clocks synced.

## log
- 2026-10-02T17:19Z created
- 2026-10-02T17:19Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
- 2026-10-02T17:19Z doing→done — verify exit 0 @ 6e33503+1
