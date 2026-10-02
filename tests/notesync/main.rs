//! The crate's one test target. Each topic is a module, so the suite links
//! into a single binary.

mod clock;
mod log;
mod merge;

use notesync::{Change, Fields};
use std::path::PathBuf;

/// An empty directory for one test, under cargo's per-target temp dir.
pub fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn fields(pairs: &[(&str, &str)]) -> Fields {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

pub fn change(device: &str, seq: u64, timestamp_ms: u64, doc: &str, body: &str) -> Change {
    Change {
        device: device.to_string(),
        seq,
        timestamp_ms,
        doc: doc.to_string(),
        fields: fields(&[("body", body)]),
    }
}
