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
    /// When the change was made, by the device's clock, in milliseconds
    /// since the Unix epoch.
    pub timestamp_ms: u64,
    /// The id of the document the change replaces.
    pub doc: String,
    /// Every field of the document after the change.
    pub fields: Fields,
}

/// A document as its winning change left it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Doc {
    pub id: String,
    pub fields: Fields,
    /// The winning change's timestamp.
    pub timestamp_ms: u64,
    /// The device that made the winning change.
    pub device: String,
    /// That device's sequence number for the winning change.
    pub seq: u64,
}

impl Doc {
    pub(crate) fn from_change(change: &Change) -> Self {
        Self {
            id: change.doc.clone(),
            fields: change.fields.clone(),
            timestamp_ms: change.timestamp_ms,
            device: change.device.clone(),
            seq: change.seq,
        }
    }
}
