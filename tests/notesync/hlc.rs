use crate::{fields, scratch};
use notesync::{log, ManualClock, Replica};
use std::fs;

const MINUTE: u64 = 60_000;
const T: u64 = 1_790_000_000_000;

#[test]
fn change_sorts_after_everything_its_author_saw() {
    let fast = ManualClock::new(T + 5 * MINUTE);
    let truth = ManualClock::new(T);
    let dir = scratch("change_sorts_after_everything_its_author_saw");
    let mut laptop = Replica::open(&dir.join("laptop"), &fast).unwrap();
    let mut phone = Replica::open(&dir.join("phone"), &truth).unwrap();

    let fix = laptop
        .put("note", fields(&[("body", "typo fixed")]))
        .unwrap();
    phone.apply(laptop.changes_since(&phone.seen())).unwrap();
    truth.advance(MINUTE);
    let rewrite = phone.put("note", fields(&[("body", "rewritten")])).unwrap();

    assert!(
        rewrite.stamp() > fix.stamp(),
        "{:?} must sort after {:?}",
        rewrite.stamp(),
        fix.stamp()
    );
    assert_eq!(rewrite.stamp(), (T + 5 * MINUTE, 1));

    laptop.apply(phone.changes_since(&laptop.seen())).unwrap();
    for replica in [&laptop, &phone] {
        assert_eq!(replica.doc("note").unwrap().fields["body"], "rewritten");
    }

    truth.set(T + 10 * MINUTE);
    let later = phone.put("note", fields(&[("body", "later")])).unwrap();
    assert_eq!(
        later.stamp(),
        (T + 10 * MINUTE, 0),
        "a clock past everything seen stamps its own time"
    );
}

#[test]
fn v1_log_reads_with_counter_zero() {
    let dir = scratch("v1_log_reads_with_counter_zero").join("phone");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("device"), "phone\n").unwrap();
    fs::write(
        dir.join("changes.log"),
        "laptop\t1\t1000\tnote\tbody=v1\nphone\t1\t900\tnote\tbody=v2\n",
    )
    .unwrap();

    for change in log::read(&dir.join("changes.log")).unwrap() {
        assert_eq!(change.counter, 0);
    }

    let mut phone = Replica::open(&dir, ManualClock::new(500)).unwrap();
    let doc = phone.doc("note").unwrap();
    assert_eq!(doc.fields["body"], "v1");
    assert_eq!(
        (doc.timestamp_ms, doc.counter, doc.observed_at_ms()),
        (1000, 0, 1000)
    );

    let next = phone.put("note", fields(&[("body", "v3")])).unwrap();
    assert_eq!(next.stamp(), (1000, 1), "sorts after the v0.1 changes");
    assert_eq!(phone.doc("note").unwrap().fields["body"], "v3");
}

#[test]
fn observed_at_is_local() {
    let fast = ManualClock::new(T + 5 * MINUTE);
    let truth = ManualClock::new(T + 2 * MINUTE);
    let dir = scratch("observed_at_is_local");
    let mut laptop = Replica::open(&dir.join("laptop"), &fast).unwrap();
    let mut phone = Replica::open(&dir.join("phone"), &truth).unwrap();

    laptop.put("note", fields(&[("body", "v1")])).unwrap();
    phone.apply(laptop.changes_since(&phone.seen())).unwrap();

    let on_phone = phone.doc("note").unwrap();
    assert_eq!(on_phone.timestamp_ms, T + 5 * MINUTE);
    assert_eq!(on_phone.observed_at_ms(), T + 2 * MINUTE);
    assert_eq!(laptop.doc("note").unwrap().observed_at_ms(), T + 5 * MINUTE);
    assert_eq!(
        on_phone,
        laptop.doc("note").unwrap(),
        "replicas agree on a document whatever they observed"
    );

    let reopened = Replica::open(&dir.join("phone"), ManualClock::new(0)).unwrap();
    assert_eq!(
        reopened.doc("note").unwrap().observed_at_ms(),
        T + 2 * MINUTE
    );
}
