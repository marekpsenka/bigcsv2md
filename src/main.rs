use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'o')]
    output: String,
    input: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut reader = csv::Reader::from_path(args.input)?;
    for rec_result in reader.records() {
        let rec = rec_result?;
        println!("{:?}", rec);
    }
    Ok(())
}
