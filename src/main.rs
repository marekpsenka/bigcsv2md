use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'o')]
    output: String,

    input: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args)
}
