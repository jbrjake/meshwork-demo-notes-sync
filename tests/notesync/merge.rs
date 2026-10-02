use crate::{change, scratch};
use notesync::{ManualClock, Replica};

fn body<C: notesync::Clock>(replica: &Replica<C>, doc: &str) -> String {
    replica.doc(doc).unwrap().fields["body"].clone()
}

#[test]
fn lww_higher_timestamp_wins() {
    let dir = scratch("lww_higher_timestamp_wins");
    let mut replica = Replica::open(&dir.join("laptop"), ManualClock::new(1_000)).unwrap();
    replica
        .put("note", crate::fields(&[("body", "mine")]))
        .unwrap();

    replica
        .apply(vec![change("phone", 1, 999, "note", "older")])
        .unwrap();
    assert_eq!(body(&replica, "note"), "mine");

    replica
        .apply(vec![change("phone", 2, 1_001, "note", "newer")])
        .unwrap();
    assert_eq!(body(&replica, "note"), "newer");
}

#[test]
fn lww_tie_breaks_on_device() {
    let dir = scratch("lww_tie_breaks_on_device");
    let mut replica = Replica::open(&dir.join("tablet"), ManualClock::new(0)).unwrap();
    replica
        .apply(vec![
            change("phone", 1, 500, "note", "from phone"),
            change("laptop", 1, 500, "note", "from laptop"),
        ])
        .unwrap();
    assert_eq!(body(&replica, "note"), "from phone");
    assert_eq!(replica.doc("note").unwrap().device, "phone");
}

#[test]
fn lww_is_order_independent() {
    let changes = vec![
        change("laptop", 1, 100, "a", "a1"),
        change("phone", 1, 300, "a", "a2"),
        change("laptop", 2, 300, "a", "a3"),
        change("phone", 2, 200, "b", "b1"),
        change("laptop", 3, 150, "b", "b2"),
    ];
    // Each device's changes stay in sequence order, as an exchange delivers
    // them; only the interleaving between devices differs.
    let mut phone_first = changes.clone();
    phone_first.sort_by_key(|c| c.device != "phone");

    let dir = scratch("lww_is_order_independent");
    let mut one = Replica::open(&dir.join("one"), ManualClock::new(0)).unwrap();
    let mut two = Replica::open(&dir.join("two"), ManualClock::new(0)).unwrap();
    one.apply(changes).unwrap();
    two.apply(phone_first).unwrap();

    let docs = |r: &Replica<ManualClock>| r.docs().cloned().collect::<Vec<_>>();
    assert_eq!(docs(&one), docs(&two));
    assert_eq!(body(&one, "a"), "a2", "equal stamps break on device id");
    assert_eq!(body(&one, "b"), "b1");
}

#[test]
fn replica_reopens_from_its_log() {
    let dir = scratch("replica_reopens_from_its_log").join("laptop");
    {
        let mut replica = Replica::open(&dir, ManualClock::new(10)).unwrap();
        replica
            .put("note", crate::fields(&[("body", "v1")]))
            .unwrap();
        replica
            .apply(vec![change("phone", 1, 20, "note", "v2")])
            .unwrap();
    }
    let replica = Replica::open(&dir, ManualClock::new(30)).unwrap();
    assert_eq!(replica.device(), "laptop");
    assert_eq!(body(&replica, "note"), "v2");
}
