use std::collections::BTreeMap;

/// A document's fields, by name.
pub type Fields = BTreeMap<String, String>;

/// One device's replacement of one whole document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    /// The device that made the change.
    pub device: String,
    /// The device's sequence number for this change, counting from 1.
    pub seq: u64,
    /// The hybrid logical clock reading's milliseconds: the later of the
    /// authoring device's wall clock and the newest timestamp it had stored.
    /// Not the time the change was made; it can run ahead of every device's
    /// clock.
    pub timestamp_ms: u64,
    /// Breaks ties between changes with the same `timestamp_ms`.
    pub counter: u32,
    /// The id of the document the change replaces.
    pub doc: String,
    /// Every field of the document after the change.
    pub fields: Fields,
}

impl Change {
    /// The change's hybrid logical clock reading, in sort order.
    pub fn stamp(&self) -> (u64, u32) {
        (self.timestamp_ms, self.counter)
    }
}

/// A document as its winning change left it.
#[derive(Clone, Debug)]
pub struct Doc {
    pub id: String,
    pub fields: Fields,
    /// The winning change's timestamp.
    pub timestamp_ms: u64,
    /// The winning change's counter.
    pub counter: u32,
    /// The device that made the winning change.
    pub device: String,
    /// That device's sequence number for the winning change.
    pub seq: u64,
    observed_ms: u64,
}

impl Doc {
    pub(crate) fn from_change(change: &Change, observed_ms: u64) -> Self {
        Self {
            id: change.doc.clone(),
            fields: change.fields.clone(),
            timestamp_ms: change.timestamp_ms,
            counter: change.counter,
            device: change.device.clone(),
            seq: change.seq,
            observed_ms,
        }
    }

    /// When this replica first stored the winning change, by this replica's
    /// clock. Use it to show when something happened. It is local: replicas
    /// never send it, and two replicas holding the same document can differ.
    pub fn observed_at_ms(&self) -> u64 {
        self.observed_ms
    }
}

/// Two replicas agree on a document when the same change won; when each
/// observed it is local, so it is not compared.
impl PartialEq for Doc {
    fn eq(&self, other: &Self) -> bool {
        (
            &self.id,
            &self.fields,
            self.timestamp_ms,
            self.counter,
            &self.device,
            self.seq,
        ) == (
            &other.id,
            &other.fields,
            other.timestamp_ms,
            other.counter,
            &other.device,
            other.seq,
        )
    }
}

impl Eq for Doc {}
