use anyhow::Result;
use clap::Parser;
use std::io::Write;
use bigcsv2md::simple::to_md_table_simple;
use bigcsv2md::csplit::to_md_tables_csplit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file path
    #[arg(short = 'o')]
    output: String,
    /// Form multiple tables by splitting after <csplit> columns
    #[arg(long = "csplit")]
    csplit: Option<usize>,
    /// Distribute first column to split tables as row headers
    #[arg(long = "rheaders")]
    rheaders: bool,
    /// Input file path
    input: String,
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
