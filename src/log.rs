//! The change log: one change per line, in the order the replica stored it.
//!
//! Columns are tab-separated: device, seq, timestamp_ms, doc, fields,
//! counter, observed_ms. Fields are `key=value` pairs joined by `;`. Tab,
//! newline, backslash, `;` and `=` are backslash-escaped wherever they
//! appear, so a raw tab is always a column separator and a raw newline always
//! ends the change. Lines written by v0.1 carry only the first five columns;
//! they read with counter 0 and observed_ms equal to their timestamp.

use crate::change::{Change, Fields};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::str::FromStr;

/// One stored change and when this replica observed it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub change: Change,
    /// When this replica first stored the change, by its own clock.
    pub observed_ms: u64,
}

/// One entry as one log line, without the newline.
pub fn encode(entry: &Entry) -> String {
    let change = &entry.change;
    let fields = change
        .fields
        .iter()
        .map(|(key, value)| format!("{}={}", escape(key), escape(value)))
        .collect::<Vec<_>>()
        .join(";");
    format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}",
        escape(&change.device),
        change.seq,
        change.timestamp_ms,
        escape(&change.doc),
        fields,
        change.counter,
        entry.observed_ms
    )
}

/// Parses one log line, from v0.1 or later.
pub fn decode(line: &str) -> io::Result<Entry> {
    let columns: Vec<&str> = line.split('\t').collect();
    let (device, seq, timestamp_ms, doc, fields, counter, observed_ms) = match columns[..] {
        [device, seq, timestamp_ms, doc, fields] => {
            (device, seq, timestamp_ms, doc, fields, None, None)
        }
        [device, seq, timestamp_ms, doc, fields, counter, observed_ms] => (
            device,
            seq,
            timestamp_ms,
            doc,
            fields,
            Some(counter),
            Some(observed_ms),
        ),
        _ => return Err(invalid(format!("expected 5 or 7 columns: {line:?}"))),
    };
    let timestamp_ms = number(timestamp_ms)?;
    Ok(Entry {
        change: Change {
            device: unescape(device)?,
            seq: number(seq)?,
            timestamp_ms,
            counter: counter.map(number).transpose()?.unwrap_or(0),
            doc: unescape(doc)?,
            fields: decode_fields(fields)?,
        },
        observed_ms: observed_ms.map(number).transpose()?.unwrap_or(timestamp_ms),
    })
}

/// Appends one entry to the log at `path`, creating the file if needed.
pub fn append(path: &Path, entry: &Entry) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", encode(entry))
}

/// Every change in the log at `path`, in log order. A missing log is empty.
pub fn read(path: &Path) -> io::Result<Vec<Change>> {
    Ok(read_entries(path)?
        .into_iter()
        .map(|entry| entry.change)
        .collect())
}

/// Every entry in the log at `path`, in log order. A missing log is empty.
pub fn read_entries(path: &Path) -> io::Result<Vec<Entry>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    BufReader::new(file)
        .lines()
        .map(|line| decode(&line?))
        .collect()
}

fn decode_fields(column: &str) -> io::Result<Fields> {
    let mut fields = Fields::new();
    if column.is_empty() {
        return Ok(fields);
    }
    for pair in split_unescaped(column, ';') {
        let [key, value] = split_unescaped(pair, '=')[..] else {
            return Err(invalid(format!("expected key=value: {pair:?}")));
        };
        fields.insert(unescape(key)?, unescape(value)?);
    }
    Ok(fields)
}

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\\' | ';' | '=' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

fn unescape(text: &str) -> io::Result<String> {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some(c @ ('\\' | ';' | '=')) => out.push(c),
            other => return Err(invalid(format!("bad escape \\{other:?} in {text:?}"))),
        }
    }
    Ok(out)
}

/// Splits `text` on each `sep` that is not backslash-escaped.
fn split_unescaped(text: &str, sep: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut escaped = false;
    for (i, c) in text.char_indices() {
        if escaped {
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == sep {
            parts.push(&text[start..i]);
            start = i + c.len_utf8();
        }
    }
    parts.push(&text[start..]);
    parts
}

fn number<N: FromStr>(column: &str) -> io::Result<N> {
    column
        .parse()
        .map_err(|_| invalid(format!("expected a number: {column:?}")))
}

fn invalid(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
