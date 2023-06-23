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
    guides: &CsplitGuides
) -> Vec<Vec<String>> {
    (0..guides.div)
        .map(|isplit| {
            records
                .iter()
                .map(move |record| {
                    let col_from = isplit * guides.csplit;
                    let col_to = (isplit + 1) * guides.csplit;
                    (col_from..col_to)
                        .map(|icol| {
                            let range = record.range(icol).expect("Valid index");
                            String::from(&record.as_slice()[range])
                        })
                        .join("|")
                })
                .collect::<Vec<String>>()
        })
        .collect()
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

    let tables = to_md_tables_csplit(&records, &guides);
    for line in tables[0].iter() {
        println!("{line}");
    }

    Ok(())
}
