use anyhow::Result;
use clap::Parser;
use csplit::to_md_tables_csplit;
use itertools::Itertools;
use std::{fmt::Display, io::Write};

mod csplit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'o')]
    output: String,
    #[arg(long = "csplit")]
    csplit: Option<usize>,
    #[arg(long = "rheaders")]
    rheaders: bool,
    input: String,
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

fn build_hline(rec: &csv::StringRecord, col_from: usize, col_to: usize) -> String {
    let to_join = (col_from..col_to).map(|icol| {
        let range = rec.range(icol).expect("Valid index");
        "-".repeat(range.len().max(1))
    });
    join_with_bars(to_join)
}

fn to_md_table_simple(headers: &csv::StringRecord, records: &[csv::StringRecord]) -> Vec<String> {
    let mut table = vec![
        join_with_bars(headers.iter()),
        build_hline(headers, 0, headers.len()),
    ];

    table.extend(records.iter().map(|record| join_with_bars(record.iter())));
    table
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut reader = csv::Reader::from_path(args.input)?;

    let headers = reader.headers()?.clone();

    let records = reader
        .records()
        .collect::<csv::Result<Vec<csv::StringRecord>>>()?;

    let output = std::fs::File::create(args.output)?;
    let mut writer = std::io::LineWriter::new(output);

    if let Some(csplit) = args.csplit {
        let tables = to_md_tables_csplit(&headers, &records, csplit, args.rheaders);
        for table in tables {
            for line in table {
                writer.write_all(line.as_bytes())?;
                writer.write_all(b"\n")?
            }
            writer.write_all(b"\n")?
        }
    } else {
        let table = to_md_table_simple(&headers, &records);
        for line in table {
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?
        }
    }

    Ok(())
}
