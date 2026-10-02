# notesync

> **This is a demo repo.** It is one of three that show [meshwork](https://github.com/jbrjake/meshwork) tracking work across projects: this library, the [notes app](https://github.com/jbrjake/meshwork-demo-notes-cli) that syncs through it, and the [portfolio](https://github.com/jbrjake/meshwork-demo-notes-portfolio) that registers both. The agent sessions in the history after the `story/0-day0` tag are staged: a script performs them with real meshwork commands, real code changes and real outputs. To replay them, clone the portfolio repo and run `story/replay.sh`; it needs git, cargo and the network.

Document replication for apps that sync documents between devices. Each device keeps a replica: a directory holding an append-only log of changes. Two replicas sync by trading the changes the other has not seen, and every replica resolves conflicts the same way, so devices that have seen the same changes show the same documents.

notesync speaks documents and fields, never any one app's data. Moving changes between devices (files, a server, a cable) is the caller's job.

The contract is [docs/PROTOCOL.md](docs/PROTOCOL.md).

## Use

```toml
[dependencies]
notesync = { package = "meshwork-demo-notes-sync", git = "https://github.com/jbrjake/meshwork-demo-notes-sync", tag = "v0.2.0" }
```

```rust
use notesync::{Fields, Replica, SystemClock};
use std::path::Path;

fn sync_two_devices(laptop_dir: &Path, phone_dir: &Path) -> std::io::Result<()> {
    let mut laptop = Replica::open(laptop_dir, SystemClock)?;
    let mut phone = Replica::open(phone_dir, SystemClock)?;

    let mut fields = Fields::new();
    fields.insert("title".into(), "Keynote outline".into());
    laptop.put("keynote", fields)?;

    // Each side sends what the other lacks.
    phone.apply(laptop.changes_since(&phone.seen()))?;
    laptop.apply(phone.changes_since(&laptop.seen()))?;

    assert_eq!(phone.doc("keynote"), laptop.doc("keynote"));
    Ok(())
}
```

A replica's device id is its directory's name, kept in `<dir>/device`. `ManualClock` stands in for `SystemClock` in tests and simulations.

## Conflicts

A change replaces a whole document. When two devices change the same document, the change with the later timestamp wins. A timestamp is a hybrid logical clock reading: the later of the device's wall clock and the newest timestamp it has stored, plus a counter. So an edit always beats the changes its device had already stored, however fast another device's clock runs. Edits made without seeing each other still race on timestamp, and there a fast clock can still win.

A timestamp can run ahead of every device's clock, so it does not say when an edit happened. `Doc::observed_at_ms()` does: when this replica first stored the winning change, by this replica's clock.

## On disk

`<dir>/changes.log` holds one change per line, in the order the replica stored it, whether it made the change or received it. Columns are tab-separated: device, sequence number, timestamp in milliseconds, document id, the fields as `key=value` pairs joined by `;`, the timestamp's counter, and when this replica observed the change. Tab, newline, backslash, `;` and `=` are backslash-escaped. Logs written by v0.1 have only the first five columns and still load.

## Develop

`scripts/gate.sh` runs formatting, lints and the tests; CI runs the same script. Work is tracked with [meshwork](https://github.com/jbrjake/meshwork) in `docs/meshwork/`.
