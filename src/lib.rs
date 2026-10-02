//! notesync replicates documents between devices.
//!
//! A replica keeps an append-only log of changes, each replacing one whole
//! document. Replicas exchange the changes the other has not seen and resolve
//! conflicts the same way everywhere. Moving changes between replicas is the
//! caller's concern. The contract is `docs/PROTOCOL.md`.

mod change;
mod clock;
pub mod log;
mod merge;
mod replica;
mod vv;

pub use change::{Change, Doc, Fields};
pub use clock::{Clock, ManualClock, SystemClock};
pub use replica::Replica;
pub use vv::VersionVector;
