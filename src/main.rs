use std::fs::File;
use std::io;
use std::io::{Cursor, Read};

use clap::Parser;
use rev_lines::{RevLines, RevLinesError};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    long_about = "Concatenate and reverse the lines of a file or stdin."
)]
struct Args {
    /// Name of the file to read. If not specified, reads from stdin.
    name: Option<String>,
    /// Number of lines to display.
    #[clap(short, long)]
    lines: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let rev_lines = match args.name {
        None => read_from_stdin(),
        Some(file_name) => read_from_file(file_name),
    };
    // limit lines if line option is specified
    let lines = match args.lines {
        None => rev_lines,
        Some(n) => Box::new(rev_lines.take(n)),
    };

    for line in lines {
        println!("{}", line.unwrap());
    }
}

type Item = Result<String, RevLinesError>;

fn read_from_file(file_name: String) -> Box<dyn Iterator<Item = Item>> {
    return match File::open(file_name.clone()) {
        Ok(f) => Box::new(RevLines::new(f)),
        Err(err) => {
            eprintln!("Error opening file '{}': {}", file_name, err);
            std::process::exit(1);
        }
    };
}
fn read_from_stdin() -> Box<dyn Iterator<Item = Item>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    return Box::new(RevLines::new(Cursor::new(buf)));
}
