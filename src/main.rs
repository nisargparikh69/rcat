use clap::Parser;
use rev_lines::RevLines;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to read
    name: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.name).unwrap();
    let rev_lines = RevLines::new(file);

    for line in rev_lines {
        println!("{}", line.unwrap());
    }
}
