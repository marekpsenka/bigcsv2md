use anyhow::{anyhow, Result};
use clap::Parser;
use itertools::Itertools;

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
    println!("{csplit}");

    let headers = reader.headers()?;
    println!("{headers:?}");

    let records = reader
        .records()
        .collect::<csv::Result<Vec<csv::StringRecord>>>()?;

    for line in to_md_table_simple(&records) {
        println!("{line}");
    }

    Ok(())
}
