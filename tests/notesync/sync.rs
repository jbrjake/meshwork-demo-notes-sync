use crate::{change, fields, scratch};
use notesync::{Clock, ManualClock, Replica, VersionVector};

/// Trades each replica the changes it lacks, the way a caller syncs two devices.
fn sync<A: Clock, B: Clock>(a: &mut Replica<A>, b: &mut Replica<B>) -> (usize, usize) {
    let to_b = b.apply(a.changes_since(&b.seen())).unwrap();
    let to_a = a.apply(b.changes_since(&a.seen())).unwrap();
    (to_b, to_a)
}

#[test]
fn sync_exchanges_missing_changes() {
    let dir = scratch("sync_exchanges_missing_changes");
    let mut laptop = Replica::open(&dir.join("laptop"), ManualClock::new(100)).unwrap();
    let mut phone = Replica::open(&dir.join("phone"), ManualClock::new(200)).unwrap();
    laptop
        .put("groceries", fields(&[("body", "eggs")]))
        .unwrap();
    laptop
        .put("todo", fields(&[("body", "call home")]))
        .unwrap();
    phone.put("ideas", fields(&[("body", "a demo")])).unwrap();

    assert_eq!(sync(&mut laptop, &mut phone), (2, 1));

    let ids = |r: &Replica<ManualClock>| r.docs().map(|d| d.id.clone()).collect::<Vec<_>>();
    assert_eq!(ids(&laptop), ["groceries", "ideas", "todo"]);
    assert_eq!(ids(&laptop), ids(&phone));
    assert_eq!(laptop.seen(), phone.seen());
    assert_eq!(laptop.seen().get("laptop"), 2);
    assert_eq!(laptop.seen().get("phone"), 1);
    assert!(laptop.changes_since(&phone.seen()).is_empty());
}

#[test]
fn sync_apply_is_idempotent() {
    let dir = scratch("sync_apply_is_idempotent");
    let mut laptop = Replica::open(&dir.join("laptop"), ManualClock::new(100)).unwrap();
    let mut phone = Replica::open(&dir.join("phone"), ManualClock::new(200)).unwrap();
    laptop.put("note", fields(&[("body", "v1")])).unwrap();

    let everything = laptop.changes_since(&VersionVector::new());
    assert_eq!(phone.apply(everything.clone()).unwrap(), 1);
    assert_eq!(phone.apply(everything).unwrap(), 0);
    assert_eq!(sync(&mut laptop, &mut phone), (0, 0));

    let log = std::fs::read_to_string(dir.join("phone").join("changes.log")).unwrap();
    assert_eq!(
        log.lines().count(),
        1,
        "a repeated change is not stored twice"
    );
}

#[test]
fn sync_refuses_a_change_that_skips_one() {
    let dir = scratch("sync_refuses_a_change_that_skips_one");
    let mut phone = Replica::open(&dir.join("phone"), ManualClock::new(0)).unwrap();
    let err = phone
        .apply(vec![change("laptop", 2, 10, "note", "v2")])
        .unwrap_err();
    assert!(
        err.to_string().contains("laptop:2 arrived before laptop:1"),
        "{err}"
    );
    assert!(phone.doc("note").is_none());
}
