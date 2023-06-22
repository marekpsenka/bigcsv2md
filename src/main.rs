use clap::Parser;
use anyhow::Result;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'o')]
    output: String,
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
    let records = reader
        .records()
        .collect::<csv::Result<Vec<csv::StringRecord>>>()?;

    for line in to_md_table_simple(&records) {
        println!("{line}");
    }

    Ok(())
}
