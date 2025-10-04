use anyhow::{Context, Result};
use itertools::Itertools;
use std::{fmt::Display, io::Read};

pub fn join_with_bars<T>(mut iter: impl Iterator<Item = T>) -> String
where
    T: Display,
{
    let mut joined = iter.join("|");
    joined.insert(0, '|');
    joined.insert(joined.len(), '|');
    joined
}

pub fn build_hline(rec: &csv::StringRecord, col_from: usize, col_to: usize) -> String {
    let to_join = (col_from..col_to).map(|icol| {
        let range = rec.range(icol).expect("Valid index");
        "-".repeat(range.len().max(1))
    });
    join_with_bars(to_join)
}

pub fn read_headers_records(
    reader: impl Read,
) -> Result<(csv::StringRecord, Vec<csv::StringRecord>)> {
    let mut reader = csv::Reader::from_reader(reader);

    let headers = reader.headers().context("Could not read headers")?.clone();

    let records = reader
        .records()
        .collect::<csv::Result<Vec<csv::StringRecord>>>()
        .context("Could not read records")?;

    Ok((headers, records))
}
