use anyhow::{anyhow, Result};
use clap::Parser;
use csplit::to_md_tables_csplit;
use itertools::Itertools;

mod csplit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'o')]
    output: String,
    #[arg(long = "csplit")]
    csplit: Option<usize>,
    input: String,
}

fn to_md_table_simple(records: &[csv::StringRecord]) -> Vec<String> {
    records
        .iter()
        .map(|record| record.iter().join("|"))
        .collect()
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut reader = csv::Reader::from_path(args.input)?;

    let csplit = args.csplit.ok_or(anyhow!("Csplit argument not provided"))?;

    let headers = reader.headers()?.clone();

    let records = reader
        .records()
        .collect::<csv::Result<Vec<csv::StringRecord>>>()?;

    let tables = to_md_tables_csplit(&headers, &records, csplit);
    for line in tables[0].iter() {
        println!("{line}");
    }

    Ok(())
}
