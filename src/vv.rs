use crate::change::Change;
use std::collections::BTreeMap;

/// The highest sequence number a replica has stored from each device.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VersionVector(BTreeMap<String, u64>);

impl VersionVector {
    pub fn new() -> Self {
        Self::default()
    }

    /// The highest sequence number stored from `device`, or 0 for none.
    pub fn get(&self, device: &str) -> u64 {
        self.0.get(device).copied().unwrap_or(0)
    }

    /// Whether `change` is already covered.
    pub fn has_seen(&self, change: &Change) -> bool {
        change.seq <= self.get(&change.device)
    }

    pub(crate) fn observe(&mut self, change: &Change) {
        let highest = self.0.entry(change.device.clone()).or_insert(0);
        *highest = (*highest).max(change.seq);
    }
}
