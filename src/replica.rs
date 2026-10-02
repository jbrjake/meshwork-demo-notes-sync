use crate::change::{Change, Doc, Fields};
use crate::clock::Clock;
use crate::log::{self, Entry};
use crate::merge;
use crate::vv::VersionVector;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// One device's copy of every document, kept in a directory.
///
/// `<dir>/device` holds the device id and `<dir>/changes.log` every change
/// the replica has stored, authored or received, in the order it stored them.
pub struct Replica<C: Clock> {
    dir: PathBuf,
    device: String,
    clock: C,
    changes: Vec<Change>,
    docs: BTreeMap<String, Doc>,
    seen: VersionVector,
    /// The newest timestamp stored, counter included.
    newest: (u64, u32),
}

impl<C: Clock> Replica<C> {
    /// Opens the replica in `dir`, creating it if needed. A new replica takes
    /// the directory's name as its device id.
    pub fn open(dir: &Path, clock: C) -> io::Result<Self> {
        fs::create_dir_all(dir)?;
        let device = device_id(dir)?;
        let mut replica = Self {
            dir: dir.to_path_buf(),
            device,
            clock,
            changes: Vec::new(),
            docs: BTreeMap::new(),
            seen: VersionVector::new(),
            newest: (0, 0),
        };
        for entry in log::read_entries(&replica.log_path())? {
            replica.remember(entry);
        }
        Ok(replica)
    }

    pub fn device(&self) -> &str {
        &self.device
    }

    /// Replaces document `doc` with `fields`, stamped by a hybrid logical
    /// clock: this device's wall clock, or just past the newest timestamp it
    /// has stored if that is later. The change sorts after every change this
    /// replica has stored.
    pub fn put(&mut self, doc: &str, fields: Fields) -> io::Result<Change> {
        let wall = self.clock.now_ms();
        let (timestamp_ms, counter) = if wall > self.newest.0 {
            (wall, 0)
        } else {
            (self.newest.0, self.newest.1 + 1)
        };
        let change = Change {
            device: self.device.clone(),
            seq: self.seen.get(&self.device) + 1,
            timestamp_ms,
            counter,
            doc: doc.to_string(),
            fields,
        };
        self.store(change.clone(), wall)?;
        Ok(change)
    }

    pub fn doc(&self, id: &str) -> Option<&Doc> {
        self.docs.get(id)
    }

    /// Every document, by id.
    pub fn docs(&self) -> impl Iterator<Item = &Doc> {
        self.docs.values()
    }

    /// The highest sequence number this replica has stored from each device.
    pub fn seen(&self) -> VersionVector {
        self.seen.clone()
    }

    /// The changes this replica has stored that `seen` does not cover, in the
    /// order it stored them.
    pub fn changes_since(&self, seen: &VersionVector) -> Vec<Change> {
        self.changes
            .iter()
            .filter(|change| !seen.has_seen(change))
            .cloned()
            .collect()
    }

    /// Stores the changes this replica has not seen; changes it has seen are
    /// skipped, so applying the same changes twice is harmless. Each device's
    /// changes must arrive in its sequence order, as an exchange delivers them.
    /// Returns how many were new.
    pub fn apply(&mut self, changes: Vec<Change>) -> io::Result<usize> {
        let mut new = 0;
        for change in changes {
            if self.seen.has_seen(&change) {
                continue;
            }
            let expected = self.seen.get(&change.device) + 1;
            if change.seq != expected {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "change {}:{} arrived before {}:{expected}",
                        change.device, change.seq, change.device
                    ),
                ));
            }
            let observed_ms = self.clock.now_ms();
            self.store(change, observed_ms)?;
            new += 1;
        }
        Ok(new)
    }

    fn log_path(&self) -> PathBuf {
        self.dir.join("changes.log")
    }

    fn store(&mut self, change: Change, observed_ms: u64) -> io::Result<()> {
        let entry = Entry {
            change,
            observed_ms,
        };
        log::append(&self.log_path(), &entry)?;
        self.remember(entry);
        Ok(())
    }

    fn remember(&mut self, entry: Entry) {
        let Entry {
            change,
            observed_ms,
        } = entry;
        self.seen.observe(&change);
        self.newest = self.newest.max(change.stamp());
        let replaces = self
            .docs
            .get(&change.doc)
            .is_none_or(|current| merge::wins(&change, current));
        if replaces {
            self.docs
                .insert(change.doc.clone(), Doc::from_change(&change, observed_ms));
        }
        self.changes.push(change);
    }
}

fn device_id(dir: &Path) -> io::Result<String> {
    let path = dir.join("device");
    match fs::read_to_string(&path) {
        Ok(id) => return Ok(id.trim().to_string()),
        Err(e) if e.kind() != io::ErrorKind::NotFound => return Err(e),
        Err(_) => {}
    }
    let id = dir
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "a replica's directory needs a name",
            )
        })?
        .to_string();
    fs::write(&path, format!("{id}\n"))?;
    Ok(id)
}
