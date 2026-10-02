---
id: sy-2yj87p9
title: Write the protocol the library promises
status: done
category: protocol
verify: "all(exists docs/PROTOCOL.md, contains docs/PROTOCOL.md /sp-change-timestamp/, contains docs/PROTOCOL.md /sp-conflict-resolution/)"
created: 2026-10-02T17:14Z
---

The library's contract, as clauses consumers can pin: what a change is, how its timestamp is taken, how conflicts resolve, and how replicas exchange changes. Each clause is a heading with an `{#sp-…}` anchor.

Conflict resolution trusts device clocks, and the clause says so: a device whose clock runs fast wins conflicts it should lose.

## log
- 2026-10-02T17:14Z created
- 2026-10-02T17:14Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
- 2026-10-02T17:14Z doing→done — verify exit 0 @ c2dae53+1
