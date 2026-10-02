use crate::scratch;
use notesync::{Clock, ManualClock, Replica};

#[test]
fn manual_clock_advances() {
    let clock = ManualClock::new(1_000);
    assert_eq!(clock.now_ms(), 1_000);
    clock.advance(250);
    assert_eq!(clock.now_ms(), 1_250);
    clock.set(90);
    assert_eq!(clock.now_ms(), 90);
}

#[test]
fn manual_clock_stamps_a_replica_it_lends_to() {
    let clock = ManualClock::new(5_000);
    let mut replica = Replica::open(&scratch("manual_clock_lent").join("laptop"), &clock).unwrap();
    assert_eq!(
        replica.put("a", Default::default()).unwrap().timestamp_ms,
        5_000
    );
    clock.advance(60_000);
    let change = replica.put("a", Default::default()).unwrap();
    assert_eq!((change.seq, change.timestamp_ms), (2, 65_000));
}
