//! A simple utility for finding large files on your system.
//!
//! Usage: spacehog [OPTIONS] [PATH]
//!
//! Arguments:
//!   [PATH]  [default: .]
//!
//! Options:
//!   -n <NUMBER>      [default: 5]
//!   -h, --help       Print help
//!   -V, --version    Print version
use clap::Parser;
use spinoff::{spinners, Color, Spinner};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, default_value_t = 5)]
    number: usize,
}

fn main() {
    let args = Args::parse();
    let mut sp = Spinner::new(spinners::Dots, "Scanning files...", Color::Blue);

    let results = spacehog::find_top_n_largest_files(&args.path, args.number);
    sp.clear();

    match results {
        Ok(files) => {
            println!("*** Top {} largest files ***", args.number);
            for file in files {
                println!("{file}");
            }
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
