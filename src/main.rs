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

fn to_md_table_simple(records: &[csv::StringRecord]) -> Vec<String> {
    records
        .iter()
        .map(|record| record.iter().join("|"))
        .collect()
}

fn to_md_tables_csplit(
    records: &[csv::StringRecord],
    ncols: usize,
    csplit: usize,
) -> Vec<Vec<String>> {
    todo!()
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut reader = csv::Reader::from_path(args.input)?;

    let csplit = args.csplit.ok_or(anyhow!("Csplit argument not provided"))?;

    let headers = reader.headers()?;
    let ncols = headers.len();
    let guides = CsplitGuides::new(ncols, csplit);
    println!("{guides:?}");

    let records = reader
        .records()
        .collect::<csv::Result<Vec<csv::StringRecord>>>()?;

    // for line in to_md_table_simple(&records) {
    //     println!("{line}");
    // }

    Ok(())
}
