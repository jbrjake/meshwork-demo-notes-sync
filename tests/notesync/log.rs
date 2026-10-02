use crate::{change, fields, scratch};
use notesync::{log, Change};

#[test]
fn log_round_trips() {
    let first = Change {
        device: "laptop".to_string(),
        seq: 1,
        timestamp_ms: 1_790_000_000_123,
        doc: "laptop-1".to_string(),
        fields: fields(&[
            ("title", "Keynote outline"),
            ("body", "Open with the demo."),
        ]),
    };
    let second = change(
        "phone",
        4,
        1_790_000_060_000,
        "laptop-1",
        "Open with the story.",
    );

    assert_eq!(log::decode(&log::encode(&first)).unwrap(), first);

    let path = scratch("log_round_trips").join("changes.log");
    log::append(&path, &first).unwrap();
    log::append(&path, &second).unwrap();
    assert_eq!(log::read(&path).unwrap(), vec![first, second]);
}

#[test]
fn log_escapes_separators() {
    let awkward = "tab\there; a=b, back\\slash\nand a new line";
    let change = Change {
        device: "my\tphone".to_string(),
        seq: 2,
        timestamp_ms: 7,
        doc: "a;b=c".to_string(),
        fields: fields(&[("k=;\\", awkward), ("", ""), ("empty", "")]),
    };

    let line = log::encode(&change);
    assert!(!line.contains('\n'), "one change, one line: {line:?}");
    assert_eq!(
        line.matches('\t').count(),
        4,
        "only the column separators are raw tabs: {line:?}"
    );
    assert_eq!(log::decode(&line).unwrap(), change);
}

#[test]
fn log_reads_a_missing_file_as_empty() {
    let path = scratch("log_reads_a_missing_file_as_empty").join("changes.log");
    assert_eq!(log::read(&path).unwrap(), Vec::<Change>::new());
}

#[test]
fn log_rejects_a_malformed_line() {
    assert!(log::decode("laptop\t1\tnot-a-number\tdoc\tbody=x").is_err());
    assert!(log::decode("laptop\t1\t2").is_err());
    assert!(log::decode("laptop\t1\t2\tdoc\tbody=dangling\\").is_err());
}
