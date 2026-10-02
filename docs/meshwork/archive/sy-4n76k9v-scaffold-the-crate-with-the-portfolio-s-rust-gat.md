---
id: sy-4n76k9v
title: Scaffold the crate with the portfolio's Rust gate scaffold
status: done
category: build
verify: "all(contains Cargo.toml /^\\[profile\\.dev\\]/, exists .cargo/config.toml, exists rust-toolchain.toml, exists tests/notesync/main.rs)"
created: 2026-10-02T17:13Z
---

A std-only library crate, `meshwork-demo-notes-sync`, imported as `notesync`, on Rust 1.97.0.

The scaffold goes in with the first code:
- `[profile.dev]` keeps line tables only, and no debug info for dependencies.
- `-D warnings` lives in `.cargo/config.toml`, so the gate, a plain `cargo test` and the editor share one build.
- One test target, `tests/notesync/main.rs`, with each topic a module.
- `scripts/gate.sh` runs fmt, clippy and the tests, and marks `target/` so Spotlight skips it.

## log
- 2026-10-02T17:13Z created
- 2026-10-02T17:13Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
- 2026-10-02T17:14Z doing→done — verify exit 0 @ 2d5d5b7+1
