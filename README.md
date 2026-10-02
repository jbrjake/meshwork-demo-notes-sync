# notesync

Document replication for apps that sync documents between devices. Each device keeps a replica: a directory holding an append-only log of changes. Two replicas sync by trading the changes the other has not seen, and every replica resolves conflicts the same way, so devices that have seen the same changes show the same documents.

notesync speaks documents and fields, never any one app's data. Moving changes between devices (files, a server, a cable) is the caller's job.

The contract is [docs/PROTOCOL.md](docs/PROTOCOL.md).

## Use

```toml
[dependencies]
notesync = { package = "meshwork-demo-notes-sync", git = "https://github.com/jbrjake/meshwork-demo-notes-sync", tag = "v0.1.0" }
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

A change replaces a whole document. When two devices change the same document, the change with the later timestamp wins, and the timestamp is the authoring device's wall clock. Conflict resolution trusts device clocks: a device whose clock runs fast wins conflicts it should lose. Keep device clocks synced.

## On disk

`<dir>/changes.log` holds one change per line, in the order the replica stored it, whether it made the change or received it. Columns are tab-separated: device, sequence number, timestamp in milliseconds, document id, and the fields as `key=value` pairs joined by `;`. Tab, newline, backslash, `;` and `=` are backslash-escaped.

## Develop

`scripts/gate.sh` runs formatting, lints and the tests; CI runs the same script. Work is tracked with [meshwork](https://github.com/jbrjake/meshwork) in `docs/meshwork/`.
