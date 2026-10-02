---
id: sy-cycv60g
title: Merge per field so concurrent edits to different fields both survive
category: merge
seq: 10
verify: run cargo test per_field_merge
docs: [docs/PROTOCOL.md#conflict-resolution-sp-conflict-resolution]
status: open
created: 2026-10-02T17:21Z
---
A change replaces the whole document, so when one device edits a document's title and another edits its body, only one of the two edits survives: the winning change carries its stale copy of the other field.

Resolve per field instead. Each field takes its value from the latest change that set it, so edits to different fields both survive. Two edits to the same field still resolve as whole-document conflicts do. The conflict clause in PROTOCOL.md changes with it.

## log
- 2026-10-02T17:21Z created
