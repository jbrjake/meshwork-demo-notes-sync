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
