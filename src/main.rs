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

use spacehog::find_top_n_largest_files;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, default_value_t = 5)]
    number: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut sp = Spinner::new(spinners::Dots, "Scanning files...", Color::Blue);

    let results = find_top_n_largest_files(&args.path, args.number)?;
    sp.clear();

    println!("*** Top {} largest files ***", args.number);
    for (size, path) in results {
        println!("{} {}", size, path.display());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::Parser;

    #[test]
    fn cli_args_can_be_parsed_without_panicing() {
        Args::parse();
    }
}
