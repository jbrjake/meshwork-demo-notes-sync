//! Conflict resolution: the last writer wins, by hybrid logical clock.

use crate::change::{Change, Doc};

/// Whether `change` replaces `current` as its document's state.
///
/// The higher timestamp wins, counter included. Equal timestamps break on
/// device id, then on sequence number, so every replica picks the same
/// winner whatever order it stored the changes in.
pub(crate) fn wins(change: &Change, current: &Doc) -> bool {
    (change.stamp(), change.device.as_str(), change.seq)
        > (
            (current.timestamp_ms, current.counter),
            current.device.as_str(),
            current.seq,
        )
}
