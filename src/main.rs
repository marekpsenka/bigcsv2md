use anyhow::{anyhow, Result};
use clap::Parser;
use itertools::Itertools;
use std::fmt::Display;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'o')]
    output: String,
    #[arg(long = "csplit")]
    csplit: Option<usize>,
    input: String,
}

#[derive(Debug)]
struct CsplitGuides {
    ncols: usize,
    csplit: usize,
    div: usize,
    div_ceil: usize,
    rem: usize,
}

impl CsplitGuides {
    fn new(ncols: usize, csplit: usize) -> CsplitGuides {
        CsplitGuides {
            ncols,
            csplit,
            div: ncols / csplit,
            div_ceil: (ncols + csplit - 1) / csplit,
            rem: ncols % csplit,
        }
    }
}

fn join_with_bars<T>(mut iter: impl Iterator<Item = T>) -> String
where
    T: Display,
{
    let mut joined = iter.join("|");
    joined.insert(0, '|');
    joined.insert(joined.len(), '|');
    joined
}

fn to_md_table_simple(records: &[csv::StringRecord]) -> Vec<String> {
    records
        .iter()
        .map(|record| record.iter().join("|"))
        .collect()
}

fn build_hline<'a>(iter: impl Iterator<Item = &'a str>) -> String {
    join_with_bars(iter.map(|s| "-".repeat(s.len())))
}

fn record_columns_to_md(rec: &csv::StringRecord, col_from: usize, col_to: usize) -> String {
    let to_join = (col_from..col_to).map(|icol| {
        let range = rec.range(icol).expect("Valid index");
        &rec.as_slice()[range]
    });
    join_with_bars(to_join)
}

fn to_md_tables_csplit(
    headers: &csv::StringRecord,
    records: &[csv::StringRecord],
    guides: &CsplitGuides,
) -> Vec<Vec<String>> {
    (0..guides.div)
        .map(|isplit| {
            let col_from = isplit * guides.csplit;
            let col_to = (isplit + 1) * guides.csplit;
            let mut this_table = vec![
                record_columns_to_md(headers, col_from, col_to)];

            this_table.extend(records
                .iter()
                .map(|rec| record_columns_to_md(rec, col_from, col_to))
            );
            this_table
        })
        .collect()
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut reader = csv::Reader::from_path(args.input)?;

    let csplit = args.csplit.ok_or(anyhow!("Csplit argument not provided"))?;

    let headers = reader.headers()?.clone();
    let ncols = headers.len();
    let guides = CsplitGuides::new(ncols, csplit);

    let records = reader
        .records()
        .collect::<csv::Result<Vec<csv::StringRecord>>>()?;

    let tables = to_md_tables_csplit(&headers, &records, &guides);
    for line in tables[0].iter() {
        println!("{line}");
    }

    Ok(())
}
