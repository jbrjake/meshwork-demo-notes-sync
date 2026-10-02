use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};

/// A source of wall-clock time, in milliseconds since the Unix epoch.
pub trait Clock {
    fn now_ms(&self) -> u64;
}

/// A borrowed clock is a clock, so a caller can keep setting a
/// [`ManualClock`] it has lent to a replica.
impl<C: Clock + ?Sized> Clock for &C {
    fn now_ms(&self) -> u64 {
        (**self).now_ms()
    }
}

/// The system's wall clock.
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_ms(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_millis() as u64)
    }
}

/// A clock that reads whatever it was last set to, for tests and simulations.
#[derive(Debug, Default)]
pub struct ManualClock {
    ms: Cell<u64>,
}

impl ManualClock {
    pub fn new(ms: u64) -> Self {
        Self { ms: Cell::new(ms) }
    }

    pub fn set(&self, ms: u64) {
        self.ms.set(ms);
    }

    pub fn advance(&self, ms: u64) {
        self.ms.set(self.ms.get() + ms);
    }
}

impl Clock for ManualClock {
    fn now_ms(&self) -> u64 {
        self.ms.get()
    }
}
